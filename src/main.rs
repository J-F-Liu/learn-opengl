#[macro_use] extern crate glium;
use glium::glutin::{Event, WindowEvent, ControlFlow};
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);


fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world")
        .with_visibility(true);
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], color: [0.0, 1.0, 0.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 mid_color;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            mid_color = color;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec3 mid_color;
        out vec4 color;

        void main() {
            color = vec4(mid_color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    events_loop.run_forever(|event| {

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 0.3);
        frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        frame.finish().unwrap();

        match event {
            Event::WindowEvent { event: WindowEvent::Closed, .. } => {
                ControlFlow::Break
            },
            _ => ControlFlow::Continue,
        }
    });
}
