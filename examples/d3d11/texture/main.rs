use dxplr;
use dxplr::d3d;
use dxplr::d3d::IBlob;
use dxplr::d3d11;
use dxplr::d3d11::{IDevice, IDeviceContext};
use dxplr::d3d11_input_element_descs;
use dxplr::dxgi;
use dxplr::dxgi::{IFactory2, ISwapChain, ISwapChain1};
use dxplr::Interface;
use image;
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
    vertex_buffer: d3d11::Buffer,
    index_buffer: d3d11::Buffer,
    _texture: d3d11::Texture2D,
    srv_tex: d3d11::ShaderResourceView,
}
impl Mesh {
    fn new(wnd: &winit::Window, device: &d3d11::Device) -> Self {
        let wnd_size = wnd
            .get_inner_size()
            .unwrap()
            .to_physical(wnd.get_hidpi_factor());
        let img = image::open("examples/d3d11/texture/sample.png")
            .unwrap()
            .to_rgba();
        let vertex_buffer = {
            let w = (img.width() as f32) / (wnd_size.width as f32);
            let h = (img.height() as f32) / (wnd_size.height as f32);
            let vertices = [
                Vertex::new([-w, -h, 0.0], [0.0, 1.0]),
                Vertex::new([-w, h, 0.0], [0.0, 0.0]),
                Vertex::new([w, h, 0.0], [1.0, 0.0]),
                Vertex::new([w, -h, 0.0], [1.0, 1.0]),
            ];
            device
                .create_buffer(
                    &d3d11::BufferDesc::new()
                        .byte_width((std::mem::size_of::<Vertex>() * vertices.len()) as u32)
                        .usage(d3d11::Usage::Default)
                        .bind_flags(d3d11::BindFlags::VertexBuffer),
                    Some(&d3d11::SubresourceData::new()
                        .sys_mem(vertices.as_ptr())
                    ),
                )
                .unwrap()
        };
        let index_buffer = {
            let indices = [0, 1, 2, 2, 3, 0];
            device
                .create_buffer(
                    &d3d11::BufferDesc::new()
                        .byte_width((std::mem::size_of::<u32>() * indices.len()) as u32)
                        .usage(d3d11::Usage::Default)
                        .bind_flags(d3d11::BindFlags::IndexBuffer),
                    Some(&d3d11::SubresourceData::new()
                        .sys_mem(indices.as_ptr())
                    ),
                )
                .unwrap()
        };
        let texture = device
            .create_texture2d(
                &d3d11::Texture2DDesc::new()
                    .width(img.width())
                    .height(img.height())
                    .format(dxgi::Format::R8G8B8A8Unorm)
                    .usage(d3d11::Usage::Default),
                Some(&d3d11::SubresourceData::new()
                    .sys_mem(img.as_ptr())
                    .sys_mem_pitch(4 * img.width())
                ),
            )
            .unwrap();
        let srv_tex = device.create_shader_resource_view(&texture, None).unwrap();
        Self {
            vertex_buffer,
            index_buffer,
            _texture: texture,
            srv_tex,
        }
    }
}

struct Renderer {
    device_context: d3d11::DeviceContext,
    swap_chain: dxgi::SwapChain3,
    rtv: d3d11::RenderTargetView,
    input_layout: d3d11::InputLayout,
    vs: d3d11::VertexShader,
    ps: d3d11::PixelShader,
    sampler: d3d11::SamplerState,
}
impl Renderer {
    fn new(wnd: &winit::Window, device: &d3d11::Device) -> Self {
        let wnd_size = wnd
            .get_inner_size()
            .unwrap()
            .to_physical(wnd.get_hidpi_factor());
        let dxgi_factory = dxgi::create_dxgi_factory1::<dxgi::Factory2>().unwrap();
        let swap_chain = dxgi_factory
            .create_swap_chain_for_hwnd(
                device,
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
        let rtv = {
            let buffer = swap_chain.get_buffer::<d3d11::Texture2D>(0).unwrap();
            device.create_render_target_view(&buffer, None).unwrap()
        };
        let (input_layout, vs, ps) = {
            let file = File::open("examples/d3d11/texture/tex.hlsl").unwrap();
            let mut reader = BufReader::new(file);
            let mut data = Vec::new();
            reader.read_to_end(&mut data).unwrap();
            let vs_bin = d3d::compile(
                &data,
                Some("tex.hlsl"),
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
                "ps_main",
                "ps_5_0",
                Some(d3d::CompileFlags::Debug),
                None,
            )
            .unwrap();
            let input_layout = device.create_input_layout(
                &d3d11_input_element_descs![
                    {"POSITION", 0, dxgi::Format::R32G32B32Float, 0, 0, d3d11::InputClassification::PerVertexData, 0},
                    {"TEXCOORD", 0, dxgi::Format::R32G32Float, 0, d3d11::APPEND_ALIGNED_ELEMENT, d3d11::InputClassification::PerVertexData, 0},
                ],
                vs_bin.as_slice()
            ).unwrap();
            let vs = device
                .create_vertex_shader(vs_bin.as_slice(), None)
                .unwrap();
            let ps = device.create_pixel_shader(ps_bin.as_slice(), None).unwrap();
            (input_layout, vs, ps)
        };
        let sampler = device
            .create_sampler_state(&d3d11::SamplerDesc::new().filter(d3d11::Filter::MinMagMipLinear))
            .unwrap();
        Self {
            device_context: device.get_immediate_context(),
            swap_chain,
            rtv,
            input_layout,
            vs,
            ps,
            sampler,
        }
    }

    fn render(&self, mesh: &Mesh) {
        let swap_chain_desc = self.swap_chain.get_desc1().unwrap();
        self.device_context
            .om_set_render_targets(Some(&[&self.rtv]), None);
        self.device_context
            .clear_render_target_view(&self.rtv, (0.0, 0.0, 0.3, 0.0).into());
        self.device_context
            .rs_set_viewports(&[d3d11::Viewport::new()
                .width(swap_chain_desc.width as f32)
                .height(swap_chain_desc.height as f32)]);
        self.device_context.ia_set_vertex_buffers(
            0,
            &[&mesh.vertex_buffer],
            &[std::mem::size_of::<Vertex>() as u32],
            &[0],
        );
        self.device_context
            .ia_set_index_buffer(&mesh.index_buffer, dxgi::Format::R32Uint, 0);
        self.device_context.ia_set_input_layout(&self.input_layout);
        self.device_context
            .ia_set_primitive_topology(d3d11::PrimitiveTopology::TriangleList);
        self.device_context.vs_set_shader(&self.vs, None);
        self.device_context.ps_set_shader(&self.ps, None);
        self.device_context
            .ps_set_shader_resources(0, &[&mesh.srv_tex]);
        self.device_context.ps_set_samplers(0, &[&self.sampler]);
        self.device_context.draw_indexed(6, 0, 0);
        if let Err(e) = self.swap_chain.present(0, None) {
            eprintln!("{}", e);
        }
    }
}

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let wnd = winit::WindowBuilder::new()
        .with_title("dxplr d3d11 texture")
        .build(&events_loop)
        .unwrap();
    let (device, _, _) = d3d11::create_device(
        None,
        d3d::DriverType::Hardware,
        None,
        None,
        &[d3d::FeatureLevel(11, 0)],
    )
    .unwrap();
    let renderer = Renderer::new(&wnd, &device);
    let mesh = Mesh::new(&wnd, &device);
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
