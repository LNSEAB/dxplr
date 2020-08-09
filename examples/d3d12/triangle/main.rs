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
use winit::platform::windows::WindowExtWindows;

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}
impl Vertex {
    const fn new(position: [f32; 3], color: [f32; 4]) -> Self {
        Self { position, color }
    }
}

const VERTICES: [Vertex; 3] = [
    Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0, 1.0]),
    Vertex::new([0.0, 0.5, 0.0], [0.0, 1.0, 0.0, 1.0]),
    Vertex::new([0.5, -0.5, 0.0], [0.0, 0.0, 1.0, 1.0]),
];

struct Triangle {
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
    _vertex_buffer: d3d12::Resource,
    vbv: d3d12::VertexBufferView,
    root_signature: d3d12::RootSignature,
    pipeline: d3d12::PipelineState,
}
impl Triangle {
    fn new(device: &d3d12::Device, wnd: &winit::window::Window) -> Self {
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
        cmd_list.close().unwrap();
        let dxgi_factory = dxgi::create_dxgi_factory1::<dxgi::Factory2>().unwrap();
        let wnd_size = wnd.inner_size();
        let swap_chain = dxgi_factory
            .create_swap_chain_for_hwnd(
                &cmd_queue,
                &wnd.hwnd(),
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
        let vertex_buffer = device
            .create_committed_resource::<d3d12::Resource>(
                &d3d12::HeapProperties::new().heap_type(d3d12::HeapType::Upload),
                None,
                &d3d12::ResourceDesc::new()
                    .dimension(d3d12::ResourceDimension::Buffer)
                    .width((std::mem::size_of::<Vertex>() * 3) as u64)
                    .height(1)
                    .format(dxgi::Format::Unknown)
                    .layout(d3d12::TextureLayout::RowMajor),
                d3d12::ResourceStates::GenericRead,
                None,
            )
            .unwrap();

        unsafe {
            let data = vertex_buffer.map(0, None).unwrap();
            std::ptr::copy_nonoverlapping(
                VERTICES.as_ptr() as *const u8,
                data.as_mut_ptr(),
                (std::mem::size_of::<Vertex>() * 3) as usize,
            );
            vertex_buffer.unmap(0, None);
        }
        let vbv = d3d12::VertexBufferView {
            buffer_location: vertex_buffer.get_gpu_virtual_address(),
            stride_in_bytes: std::mem::size_of::<Vertex>() as u32,
            size_in_bytes: (std::mem::size_of::<Vertex>() * 3) as u32,
        };
        let root_signature = {
            let desc = d3d12::RootSignatureDesc {
                parameters: None,
                static_samplers: None,
                flags: Some(d3d12::RootSignatureFlags::AllowInputAssemblerInputLayout),
            };
            let data =
                d3d12::serialize_root_signature(&desc, d3d::RootSignatureVersion(1, 0)).unwrap();
            device
                .create_root_signature::<d3d12::RootSignature>(0, data.as_slice())
                .unwrap()
        };
        let pipeline = {
            let file = File::open("examples/d3d12/triangle/simple.hlsl").unwrap();
            let mut reader = BufReader::new(file);
            let mut data = Vec::new();
            reader.read_to_end(&mut data).unwrap();
            let vs_bin = d3d::compile(
                &data,
                Some("simple.hlsl"),
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
                Some("simple.hlsl"),
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
                .vs((&vs_bin).into())
                .ps((&ps_bin).into())
                .depth_stencil_state(d3d12::DepthStencilDesc::new()
                    .depth_enable(false)
                )
                .input_layout(d3d12_input_layout_descs![
                    {"POSITION", 0, dxgi::Format::R32G32B32Float, 0, 0, d3d12::InputClassification::PerVertexData, 0},
                    {"COLOR", 0, dxgi::Format::R32G32B32A32Float, 0, d3d12::APPEND_ALIGNED_ELEMENT, d3d12::InputClassification::PerVertexData, 0},
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
            _vertex_buffer: vertex_buffer,
            vbv,
            root_signature,
            pipeline,
        }
    }

    fn render(&self) {
        let index = self.swap_chain.get_current_back_buffer_index() as usize;
        let rtv_handle = {
            let handle = self.rtv_heap.get_cpu_descriptor_handle_for_heap_start();
            let offset = self
                .device
                .get_descriptor_handle_increment_size(d3d12::DescriptorHeapType::RTV)
                as usize;
            handle + offset * index
        };
        let swap_chain_desc = self.swap_chain.get_desc1().unwrap();
        self.cmd_allocator.reset().unwrap();
        self.cmd_list
            .reset(&self.cmd_allocator, Some(&self.pipeline))
            .unwrap();
        self.cmd_list
            .set_graphics_root_signature(&self.root_signature);
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
            .ia_set_primitive_topology(d3d::PrimitiveTopology::TriangleList);
        self.cmd_list.ia_set_vertex_buffers(0, &[self.vbv.clone()]);
        self.cmd_list.draw_instanced(3, 1, 0, 0);
        self.cmd_list
            .resource_barrier(&[d3d12::ResourceBarrier::Transition {
                flags: None,
                resource: &self.render_targets[index],
                subresource: d3d12::RESOURCE_BARRIER_ALL_SUBRESOURCES,
                state_before: d3d12::ResourceStates::RenderTarget,
                state_after: d3d12::ResourceStates::Present,
            }]);
        self.cmd_list.close().unwrap();
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
    let event_loop = winit::event_loop::EventLoop::new();
    let wnd = winit::window::WindowBuilder::new()
        .with_title("dxplr d3d12 triangle")
        .build(&event_loop)
        .unwrap();

    let triangle = Triangle::new(&device, &wnd);

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => *control_flow = winit::event_loop::ControlFlow::Exit,
        _ => triangle.render(),
    });
}
