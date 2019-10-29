use dxplr;
use dxplr::d3d;
use dxplr::d3d::IBlob;
use dxplr::d3d11;
use dxplr::d3d11::{IDevice, IDeviceContext};
use dxplr::d3d11_input_element_descs;
use dxplr::dxgi;
use dxplr::dxgi::ISwapChain;
use std::fs::File;
use std::io::{BufReader, Read};
use winit;
use winit::os::windows::WindowExt;

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
    device_context: d3d11::DeviceContext,
    swap_chain: dxgi::SwapChain,
    rtv: d3d11::RenderTargetView,
    vertex_buffer: d3d11::Buffer,
    input_layout: d3d11::InputLayout,
    vs: d3d11::VertexShader,
    ps: d3d11::PixelShader,
}
impl Triangle {
    fn new(wnd: &winit::Window) -> Self {
        let wnd_size = wnd
            .get_inner_size()
            .unwrap()
            .to_physical(wnd.get_hidpi_factor());
        let (swap_chain, device, _, device_context) = d3d11::create_device_and_swap_chain(
            None,
            d3d::DriverType::Hardware,
            None,
            Some(d3d11::CreateDeviceFlags::Debug),
            &[d3d::FeatureLevel(11, 0)],
            &dxgi::SwapChainDesc::new()
                .buffer_desc(
                    dxgi::ModeDesc::new()
                        .width(wnd_size.width as u32)
                        .height(wnd_size.height as u32)
                        .refresh_rate(dxgi::Rational::new(60, 1))
                        .format(dxgi::Format::R8G8B8A8Unorm),
                )
                .buffer_usage(dxgi::Usage::RenderTargetOutput)
                .buffer_count(2)
                .output_window(&wnd.get_hwnd())
                .windowed(true)
                .swap_effect(dxgi::SwapEffect::FlipDiscard),
        )
        .unwrap();
        let rtv = {
            let buffer = swap_chain.get_buffer::<d3d11::Texture2D>(0).unwrap();
            device.create_render_target_view(&buffer, None).unwrap()
        };
        let vertex_buffer = device
            .create_buffer(
                &d3d11::BufferDesc::new()
                    .byte_width((std::mem::size_of::<Vertex>() * 3) as u32)
                    .usage(d3d11::Usage::Default)
                    .bind_flags(d3d11::BindFlags::VertexBuffer),
                Some(&d3d11::SubresourceData::new()
                    .sys_mem(VERTICES.as_ptr())
                ),
            )
            .unwrap();
        let (input_layout, vs, ps) = {
            let file = File::open("examples/d3d11/triangle/simple.hlsl").unwrap();
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
            let input_layout = device.create_input_layout(
                &d3d11_input_element_descs![
                    {"POSITION", 0, dxgi::Format::R32G32B32Float, 0, 0, d3d11::InputClassification::PerVertexData, 0},
                    {"COLOR", 0, dxgi::Format::R32G32B32A32Float, 0, d3d11::APPEND_ALIGNED_ELEMENT, d3d11::InputClassification::PerVertexData, 0},
                ],
                vs_bin.as_slice()
            ).unwrap();
            let vs = device
                .create_vertex_shader(vs_bin.as_slice(), None)
                .unwrap();
            let ps = device.create_pixel_shader(ps_bin.as_slice(), None).unwrap();
            (input_layout, vs, ps)
        };
        Self {
            device_context,
            swap_chain,
            rtv,
            vertex_buffer,
            input_layout,
            vs,
            ps,
        }
    }

    fn render(&self) {
        let swap_chain_desc = self.swap_chain.get_desc().unwrap();
        self.device_context
            .om_set_render_targets(Some(&[&self.rtv]), None);
        self.device_context
            .clear_render_target_view(&self.rtv, (0.0, 0.0, 0.3, 0.0).into());
        self.device_context
            .rs_set_viewports(&[d3d11::Viewport::new()
                .width(swap_chain_desc.buffer_desc.width as f32)
                .height(swap_chain_desc.buffer_desc.height as f32)]);
        self.device_context.ia_set_vertex_buffers(
            0,
            &[&self.vertex_buffer],
            &[std::mem::size_of::<Vertex>() as u32],
            &[0],
        );
        self.device_context
            .ia_set_primitive_topology(d3d11::PrimitiveTopology::TriangleList);
        self.device_context.ia_set_input_layout(&self.input_layout);
        self.device_context.vs_set_shader(&self.vs, None);
        self.device_context.ps_set_shader(&self.ps, None);
        self.device_context.draw(3, 0);
        if let Err(e) = self.swap_chain.present(0, None) {
            eprintln!("{}", e);
        }
    }
}

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let wnd = winit::WindowBuilder::new()
        .with_title("dxplr d3d11 triangle")
        .build(&events_loop)
        .unwrap();
    let triangle = Triangle::new(&wnd);
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
        triangle.render();
    }
}
