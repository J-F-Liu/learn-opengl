extern crate glium;
use glium::glutin::{Event, WindowEvent, ControlFlow};
use glium::Surface;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world")
        .with_visibility(true);
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    events_loop.run_forever(|event| {

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 0.3);
        frame.finish().unwrap();

        match event {
            Event::WindowEvent { event: WindowEvent::Closed, .. } => {
                ControlFlow::Break
            },
            _ => ControlFlow::Continue,
        }
    });
}
