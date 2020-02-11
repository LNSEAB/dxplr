use winit;
use winit::platform::windows::WindowExtWindows;
use dxplr::d2d1;
use dxplr::d2d1::{IFactory as _, IRenderTarget as _};
use dxplr::dwrite;
use dxplr::dwrite::{IFactory as _, ITextFormat};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let wnd = winit::window::WindowBuilder::new()
        .with_title("dxplr d2d1 hello")
        .build(&event_loop)
        .unwrap();
    let wnd_size = wnd.inner_size();
    let scale_factor = wnd.scale_factor() as f32;

    let d2d1_factory = d2d1::create_factory::<d2d1::Factory>(d2d1::FactoryType::SingleThreaded, None).unwrap();
    let rt = d2d1_factory.create_hwnd_render_target(
        &d2d1::RenderTargetProperties::new(),
        &d2d1::HwndRenderTargetProperties::new(&wnd.hwnd())
            .pixel_size((wnd_size.width, wnd_size.height)),
    ).unwrap();
    let brush = rt.create_solid_color_brush((1.0, 1.0, 0.0, 1.0), None).unwrap();

    let dwrite_factory = dwrite::create_factory::<dwrite::Factory>(dwrite::FactoryType::Shared).unwrap();
    let text_format = dwrite_factory.create_text_format(
        "Meiryo",
        None,
        dwrite::FontWeight::Normal,
        dwrite::FontStyle::Normal,
        dwrite::FontStretch::Normal,
        32.0,
        ""
    ).unwrap();
    text_format.set_text_alignment(dwrite::TextAlignment::Center).unwrap();
    text_format.set_paragraph_alignment(dwrite::ParagraphAlignment::Center).unwrap();
    let text_layout = dwrite_factory.create_text_layout(
        "Hello",
        &text_format,
        wnd_size.width as f32 / scale_factor,
        wnd_size.height as f32 / scale_factor,
    ).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => *control_flow = winit::event_loop::ControlFlow::Exit,
        winit::event::Event::RedrawRequested(_) => {
            rt.begin_draw();
            rt.clear((0.0, 0.0, 0.3, 0.0));
            rt.draw_text_layout(
                (0.0, 0.0),
                &text_layout,
                &brush,
                None,
            );
            rt.end_draw().unwrap();
        },
        _ => (),
    });
}

