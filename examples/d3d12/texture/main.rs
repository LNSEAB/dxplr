use dxplr;
use dxplr::d3d::IBlob;
use dxplr::d3d12::{
    ICommandAllocator, ICommandQueue, IDebug, IDescriptorHeap, IDevice, IFence,
    IGraphicsCommandList, IResource,
};
use dxplr::d3d12_input_layout_descs;
use dxplr::dxgi::{IFactory2, ISwapChain, ISwapChain1, ISwapChain3};
use dxplr::Interface;
use dxplr::{d3d, d3d12, dxgi};
use std::cell::Cell;
use std::fs::File;
use std::io::{BufReader, Read};
use winit;
use winit::os::windows::WindowExt;

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
}
impl Vertex {
    fn new(position: [f32; 3], uv: [f32; 2]) -> Self {
        Self { position, uv }
    }
}

struct Mesh {
    _vertex_buffer: d3d12::Resource,
    vbv: d3d12::VertexBufferView,
    resource_heap: d3d12::DescriptorHeap,
    _texture: d3d12::Resource,
}

struct Renderer {
    device: d3d12::Device,
    cmd_allocator: d3d12::CommandAllocator,
    cmd_queue: d3d12::CommandQueue,
    cmd_list: d3d12::GraphicsCommandList,
    swap_chain: dxgi::SwapChain3,
    rtv_heap: d3d12::DescriptorHeap,
    render_targets: Vec<d3d12::Resource>,
    fence: d3d12::Fence,
    fence_value: Cell<u64>,
    event: dxplr::EventHandle,
    root_signature: d3d12::RootSignature,
    pipeline: d3d12::PipelineState,
}
impl Renderer {
    fn new(device: &d3d12::Device, wnd: &winit::Window) -> Self {
        let cmd_allocator = device
            .create_command_allocator::<d3d12::CommandAllocator>(d3d12::CommandListType::Direct)
            .unwrap();
        let cmd_queue = device
            .create_command_queue::<d3d12::CommandQueue>(
                &d3d12::CommandQueueDesc::new().list_type(d3d12::CommandListType::Direct),
            )
            .unwrap();
        let cmd_list = device
            .create_command_list::<d3d12::GraphicsCommandList>(
                0,
                d3d12::CommandListType::Direct,
                &cmd_allocator,
                None,
            )
            .unwrap();
        cmd_list.close();
        let dxgi_factory = dxgi::create_dxgi_factory1::<dxgi::Factory2>().unwrap();
        let wnd_size = wnd
            .get_inner_size()
            .unwrap()
            .to_physical(wnd.get_hidpi_factor());
        let swap_chain = dxgi_factory
            .create_swap_chain_for_hwnd(
                &cmd_queue,
                &wnd.get_hwnd(),
                &dxgi::SwapChainDesc1::new()
                    .width(wnd_size.width as u32)
                    .height(wnd_size.height as u32)
                    .format(dxgi::Format::R8G8B8A8Unorm)
                    .buffer_usage(dxgi::Usage::RenderTargetOutput)
                    .buffer_count(2)
                    .swap_effect(dxgi::SwapEffect::FlipDiscard),
                None,
                None,
            )
            .unwrap()
            .query_interface::<dxgi::SwapChain3>()
            .unwrap();
        let rtv_heap = device
            .create_descriptor_heap::<d3d12::DescriptorHeap>(
                &d3d12::DescriptorHeapDesc::new()
                    .heap_type(d3d12::DescriptorHeapType::RTV)
                    .num_descriptors(2),
            )
            .unwrap();
        let render_targets = {
            let mut handle = rtv_heap.get_cpu_descriptor_handle_for_heap_start();
            let offset = device.get_descriptor_handle_increment_size(d3d12::DescriptorHeapType::RTV)
                as usize;
            let desc = d3d12::RenderTargetViewDesc::Texture2D {
                format: dxgi::Format::R8G8B8A8Unorm,
                mip_slice: 0,
                plane_slice: 0,
            };
            let mut rts = Vec::new();
            for i in 0..2 {
                let rt = swap_chain.get_buffer::<d3d12::Resource>(i).unwrap();
                unsafe { device.create_render_target_view(Some(&rt), Some(&desc), handle) };
                rts.push(rt);
                handle += offset;
            }
            rts
        };
        let fence = device.create_fence::<d3d12::Fence>(0, None).unwrap();
        let event = dxplr::EventHandle::new();
        let root_signature = {
            let desc = d3d12::RootSignatureDesc {
                parameters: Some(vec![d3d12::RootParameter::DescriptorTable {
                    descriptor_ranges: vec![d3d12::DescriptorRange {
                        range_type: d3d12::DescriptorRangeType::SRV,
                        num_descriptors: 1,
                        base_shader_register: 0,
                        register_space: 0,
                        offset_in_descriptors_from_table_start:
                            d3d12::DescriptorRange::OFFSET_APPEND,
                    }],
                    shader_visibility: d3d12::ShaderVisibility::Pixel,
                }]),
                static_samplers: Some(vec![d3d12::StaticSamplerDesc::new()
                    .filter(d3d12::Filter::MinMagMipLinear)
                    .shader_register(0)
                    .register_space(0)
                    .shader_visibility(d3d12::ShaderVisibility::Pixel)]),
                flags: Some(d3d12::RootSignatureFlags::AllowInputAssemblerInputLayout),
            };
            let data =
                d3d12::serialize_root_signature(&desc, d3d::RootSignatureVersion(1, 0)).unwrap();
            device
                .create_root_signature::<d3d12::RootSignature>(0, data.as_slice())
                .unwrap()
        };
        let pipeline = {
            let file = File::open("examples/d3d12/texture/tex.hlsl").unwrap();
            let mut reader = BufReader::new(file);
            let mut data = Vec::new();
            reader.read_to_end(&mut data).unwrap();
            let vs_bin = d3d::compile(
                &data,
                Some("tex.hlsl"),
                None,
                None,
                "vs_main",
                "vs_5_0",
                Some(d3d::CompileFlags::Debug),
                None,
            )
            .unwrap();
            let ps_bin = d3d::compile(
                &data,
                Some("tex.hlsl"),
                None,
                None,
                "ps_main",
                "ps_5_0",
                Some(d3d::CompileFlags::Debug),
                None,
            )
            .unwrap();
            let desc = d3d12::GraphicsPipelineStateDesc::new()
                .root_signature(&root_signature)
                .vs(vs_bin.into())
                .ps(ps_bin.into())
                .depth_stencil_state(d3d12::DepthStencilDesc::new()
                    .depth_enable(false)
                )
                .input_layout(d3d12_input_layout_descs![
                    {"POSITION", 0, dxgi::Format::R32G32B32Float, 0, 0, d3d12::InputClassification::PerVertexData, 0},
                    {"TEXCOORD", 0, dxgi::Format::R32G32Float, 0, d3d12::APPEND_ALIGNED_ELEMENT, d3d12::InputClassification::PerVertexData, 0},
                ])
                .primitive_topology_type(d3d12::PrimitiveTopologyType::Triangle)
                .rtv_formats(&[dxgi::Format::R8G8B8A8Unorm])
                .dsv_format(dxgi::Format::Unknown);
            device.create_graphics_pipeline_state(&desc).unwrap()
        };
        Self {
            device: device.clone(),
            cmd_allocator,
            cmd_queue,
            cmd_list,
            swap_chain,
            rtv_heap,
            render_targets,
            fence,
            fence_value: Cell::new(1),
            event,
            root_signature,
            pipeline,
        }
    }

    fn upload_image(&self, wnd: &winit::Window, image: image::RgbaImage) -> Mesh {
        let vertex_buffer = self
            .device
            .create_committed_resource::<d3d12::Resource>(
                &d3d12::HeapProperties::new().heap_type(d3d12::HeapType::Upload),
                None,
                &d3d12::ResourceDesc::new()
                    .dimension(d3d12::ResourceDimension::Buffer)
                    .width((std::mem::size_of::<Vertex>() * 6) as u64)
                    .height(1)
                    .format(dxgi::Format::Unknown)
                    .layout(d3d12::TextureLayout::RowMajor),
                d3d12::ResourceStates::GenericRead,
                None,
            )
            .unwrap();
        let wnd_size = wnd
            .get_inner_size()
            .unwrap()
            .to_physical(wnd.get_hidpi_factor());
        unsafe {
            let w = (image.width() as f32) / (wnd_size.width as f32);
            let h = (image.height() as f32) / (wnd_size.height as f32);
            let vertices = [
                Vertex::new([-w, h, 0.0], [0.0, 0.0]),
                Vertex::new([w, h, 0.0], [1.0, 0.0]),
                Vertex::new([-w, -h, 0.0], [0.0, 1.0]),
                Vertex::new([w, h, 0.0], [1.0, 0.0]),
                Vertex::new([w, -h, 0.0], [1.0, 1.0]),
                Vertex::new([-w, -h, 0.0], [0.0, 1.0]),
            ];
            let data = vertex_buffer.map(0, None).unwrap();
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr() as *const u8,
                data.as_mut_ptr(),
                (std::mem::size_of::<Vertex>() * 6) as usize,
            );
            vertex_buffer.unmap(0, None);
        }
        let vbv = d3d12::VertexBufferView {
            buffer_location: vertex_buffer.get_gpu_virtual_address(),
            stride_in_bytes: std::mem::size_of::<Vertex>() as u32,
            size_in_bytes: (std::mem::size_of::<Vertex>() * 6) as u32,
        };
        let resource_heap = self
            .device
            .create_descriptor_heap::<d3d12::DescriptorHeap>(
                &d3d12::DescriptorHeapDesc::new()
                    .heap_type(d3d12::DescriptorHeapType::CBVSRVUAV)
                    .num_descriptors(1)
                    .flags(d3d12::DescriptorHeapFlags::ShaderVisible),
            )
            .unwrap();
        let texture_desc = d3d12::ResourceDesc::new()
            .dimension(d3d12::ResourceDimension::Texture2D)
            .width(image.width() as u64)
            .height(image.height() as u32)
            .format(dxgi::Format::R8G8B8A8Unorm)
            .layout(d3d12::TextureLayout::Unknown);
        let texture = self
            .device
            .create_committed_resource::<d3d12::Resource>(
                &d3d12::HeapProperties::new().heap_type(d3d12::HeapType::Default),
                None,
                &texture_desc,
                d3d12::ResourceStates::Common,
                None,
            )
            .unwrap();
        unsafe {
            self.device.create_shader_resource_view(
                Some(&texture),
                Some(&d3d12::ShaderResourceViewDesc::Texture2D {
                    format: dxgi::Format::R8G8B8A8Unorm,
                    shader_4_component_mapping: d3d12::DEFAULT_4_COMPONENT_MAPPING,
                    mip_levels: 1,
                    most_detailed_mip: 0,
                    plane_slice: 0,
                    resource_min_lod_clamp: 0.0,
                }),
                resource_heap.get_cpu_descriptor_handle_for_heap_start(),
            );
        }
        let footprints = self.device.get_copyable_footprints(&texture_desc, 0, 1, 0);
        let intermediate = self
            .device
            .create_committed_resource::<d3d12::Resource>(
                &d3d12::HeapProperties::new().heap_type(d3d12::HeapType::Upload),
                None,
                &d3d12::ResourceDesc::new()
                    .dimension(d3d12::ResourceDimension::Buffer)
                    .width(footprints.total_bytes)
                    .height(1)
                    .format(dxgi::Format::Unknown)
                    .layout(d3d12::TextureLayout::RowMajor),
                d3d12::ResourceStates::GenericRead,
                None,
            )
            .unwrap();
        unsafe {
            let data = intermediate.map(0, None).unwrap();
            for y in 0..texture_desc.height as isize {
                let src = image.as_ptr().offset((image.width() * 4) as isize * y);
                let dest = data
                    .as_mut_ptr()
                    .offset(footprints.layouts[0].footprint.row_pitch as isize * y);
                std::ptr::copy_nonoverlapping(src, dest, (image.width() * 4) as usize);
            }
            intermediate.unmap(0, None);
        }
        let src = d3d12::TextureCopyLocation::PlacedFootprint {
            resource: &intermediate,
            footprint: footprints.layouts[0].footprint.clone(),
            offset: 0,
        };
        let dest = d3d12::TextureCopyLocation::SubresourceIndex {
            resource: &texture,
            index: 0,
        };
        self.cmd_allocator.reset().unwrap();
        self.cmd_list.reset(&self.cmd_allocator, None).unwrap();
        self.cmd_list
            .resource_barrier(&[d3d12::ResourceBarrier::Transition {
                flags: None,
                resource: &texture,
                subresource: d3d12::RESOURCE_BARRIER_ALL_SUBRESOURCES,
                state_before: d3d12::ResourceStates::Common,
                state_after: d3d12::ResourceStates::CopyDest,
            }]);
        self.cmd_list
            .copy_texture_region(&dest, 0, 0, 0, &src, None);
        self.cmd_list
            .resource_barrier(&[d3d12::ResourceBarrier::Transition {
                flags: None,
                resource: &texture,
                subresource: d3d12::RESOURCE_BARRIER_ALL_SUBRESOURCES,
                state_before: d3d12::ResourceStates::CopyDest,
                state_after: d3d12::ResourceStates::Common,
            }]);
        self.cmd_list.close();
        self.cmd_queue
            .execute_command_lists(&[self.cmd_list.as_command_list()]);
        self.wait();
        Mesh {
            _vertex_buffer: vertex_buffer,
            vbv,
            resource_heap,
            _texture: texture,
        }
    }

    fn render(&self, mesh: &Mesh) {
        let index = self.swap_chain.get_current_back_buffer_index() as usize;
        let rtv_handle = {
            let handle = self.rtv_heap.get_cpu_descriptor_handle_for_heap_start();
            let offset = self
                .device
                .get_descriptor_handle_increment_size(d3d12::DescriptorHeapType::RTV);
            handle + index * offset as usize
        };
        let swap_chain_desc = self.swap_chain.get_desc1().unwrap();
        self.cmd_allocator.reset().unwrap();
        self.cmd_list
            .reset(&self.cmd_allocator, Some(&self.pipeline))
            .unwrap();
        self.cmd_list.set_descriptor_heaps(&[&mesh.resource_heap]);
        self.cmd_list.rs_set_viewports(&[d3d12::Viewport::new()
            .width(swap_chain_desc.width as f32)
            .height(swap_chain_desc.height as f32)]);
        self.cmd_list.rs_set_scissor_rects(&[dxplr::Rect {
            left: 0,
            top: 0,
            right: swap_chain_desc.width as i32,
            bottom: swap_chain_desc.height as i32,
        }]);
        self.cmd_list
            .resource_barrier(&[d3d12::ResourceBarrier::Transition {
                flags: None,
                resource: &self.render_targets[index],
                subresource: d3d12::RESOURCE_BARRIER_ALL_SUBRESOURCES,
                state_before: d3d12::ResourceStates::Present,
                state_after: d3d12::ResourceStates::RenderTarget,
            }]);
        self.cmd_list
            .om_set_render_targets(&[rtv_handle], false, None);
        self.cmd_list
            .clear_render_target_view(rtv_handle, (0.0, 0.0, 0.3, 0.0).into(), None);
        self.cmd_list
            .set_graphics_root_signature(&self.root_signature);
        self.cmd_list.set_graphics_root_descriptor_table(
            0,
            mesh.resource_heap
                .get_gpu_descriptor_handle_for_heap_start(),
        );
        self.cmd_list
            .ia_set_primitive_topology(d3d::PrimitiveTopology::TriangleList);
        self.cmd_list.ia_set_vertex_buffers(0, &[mesh.vbv.clone()]);
        self.cmd_list.draw_instanced(6, 1, 0, 0);
        self.cmd_list
            .resource_barrier(&[d3d12::ResourceBarrier::Transition {
                flags: None,
                resource: &self.render_targets[index],
                subresource: d3d12::RESOURCE_BARRIER_ALL_SUBRESOURCES,
                state_before: d3d12::ResourceStates::RenderTarget,
                state_after: d3d12::ResourceStates::Present,
            }]);
        self.cmd_list.close();
        self.cmd_queue
            .execute_command_lists(&[self.cmd_list.as_command_list()]);
        if let Err(e) = self.swap_chain.present(0, None) {
            eprintln!("{:?}", e);
        }
        self.wait();
    }

    fn wait(&self) {
        let fence_value = {
            let fv = self.fence_value.get();
            self.fence_value.set(fv + 1);
            fv
        };
        self.cmd_queue.signal(&self.fence, fence_value).unwrap();
        if self.fence.get_completed_value() < fence_value {
            self.fence
                .set_event_on_completion(fence_value, &self.event)
                .unwrap();
            self.event.wait(None);
        }
    }
}

fn main() {
    let _d3d12_debug = {
        let debug = d3d12::get_debug_interface::<d3d12::Debug>().unwrap();
        debug.enable_debug_layer();
        debug
    };
    let device = d3d12::create_device::<d3d12::Device>(None, d3d::FeatureLevel(12, 0)).unwrap();
    let mut events_loop = winit::EventsLoop::new();
    let wnd = winit::WindowBuilder::new()
        .with_title("dxplr triangle")
        .build(&events_loop)
        .unwrap();

    let renderer = Renderer::new(&device, &wnd);
    let mesh = renderer.upload_image(
        &wnd,
        image::open("examples/texture/sample.png")
            .unwrap()
            .to_rgba(),
    );

    let mut exit_flag = false;
    loop {
        events_loop.poll_events(|event| match event {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested,
                ..
            } => exit_flag = true,
            _ => (),
        });
        if exit_flag {
            break;
        }
        renderer.render(&mesh);
    }
}
