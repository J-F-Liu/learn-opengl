#[macro_use] extern crate glium;
use glium::glutin::{Event, WindowEvent};
use glium::Surface;
use glium::index::PrimitiveType::TrianglesList;

extern crate tobj;

use std::path::Path;
use std::f32;
use std::thread::sleep;
use std::time::Duration;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color_diffuse: [f32; 3],
    color_specular: [f32; 4],
}

implement_vertex!(Vertex, position, normal, color_diffuse, color_specular);

fn load_object(path: &Path) -> (Vec<Vertex>, Vec<u32>, f32) {
    let mut min_pos = [f32::INFINITY; 3];
    let mut max_pos = [f32::NEG_INFINITY; 3];
    let mut vertex_data = Vec::new();
    let mut index_data = Vec::new();
    match tobj::load_obj(path) {
        Ok((models, mats)) => {
            for model in &models {
                let mesh = &model.mesh;
                println!("Loading model: {}", model.name);
                for i in 0..(mesh.positions.len() / 3) {
                    let pos = [mesh.positions[3 * i], mesh.positions[3 * i + 1], mesh.positions[3 * i + 2]];
                    let normal =
                        if !mesh.normals.is_empty() {
                            [mesh.normals[3 * i], mesh.normals[3 * i + 1], mesh.normals[3 * i + 2]]
                        } else {
                            [0.0, 0.0, 0.0]
                        };
                    let (color_diffuse, color_specular) =
                        match mesh.material_id {
                            Some(i) => (mats[i].diffuse, [mats[i].specular[0], mats[i].specular[1],
                                        mats[i].specular[2], mats[i].shininess]),
                            None => ([0.8, 0.8, 0.8], [0.15, 0.15, 0.15, 15.0])
                        };
                    vertex_data.push(Vertex {
                        position: pos,
                        normal: normal,
                        color_diffuse: color_diffuse,
                        color_specular: color_specular,
                    });
                    // Update our min/max pos so we can figure out the bounding box of the object
                    // to view it
                    for i in 0..3 {
                        min_pos[i] = f32::min(min_pos[i], pos[i]);
                        max_pos[i] = f32::max(max_pos[i], pos[i]);
                    }
                }

                for idx in &mesh.indices {
                  index_data.push(*idx);
                }
            }
        },
        Err(e) => panic!("Loading of {:?} failed due to {:?}", path, e),
    }
    // Compute scale factor to fit the model with a [-1, 1] bounding box
    let diagonal_len = 2.0;
    let current_len = f32::powf(max_pos[0] - min_pos[0], 2.0) + f32::powf(max_pos[1] - min_pos[1], 2.0)
        + f32::powf(max_pos[2] - min_pos[2], 2.0);
    let scale = diagonal_len / f32::sqrt(current_len);
    println!("Model scaled by {} to fit", scale);
    (vertex_data, index_data, scale)
}

fn main() {
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Teapot")
        .with_visibility(true);
    let context = glium::glutin::ContextBuilder::new();
    let mut events_loop = glium::glutin::EventsLoop::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let teapot = load_object(&Path::new("res/teapot.obj"));

    let vertex_buffer = glium::VertexBuffer::new(&display, &teapot.0).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, TrianglesList, &teapot.1).unwrap();
    let scale = teapot.2;

    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
            v_normal = normal;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec3 v_normal;
        out vec4 color;

        uniform vec3 light_dir;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(light_dir));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut closed = false;
    while !closed {
        let uniforms = uniform! {
          matrix: [
              [scale, 0.0, 0.0, 0.0],
              [0.0, scale, 0.0, 0.0],
              [0.0, 0.0, scale, 0.0],
              [0.0, 0.0, 0.0, 1.0f32],
          ],
          light_dir: [-1.0, 0.4, 0.9f32],
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        // sleeping for some time in order not to use up too much CPU
        sleep(Duration::from_millis(17));

        // polling and handling the events received by the window
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }
}
