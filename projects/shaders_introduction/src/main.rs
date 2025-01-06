use glium::{implement_vertex, Surface};
use std::error::Error;
use winit::event_loop::EventLoop;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let f_shader = r#"
        #version 140
        precision mediump float;

        uniform vec2 u_resolution;
        uniform vec2 u_mouse;
        uniform float u_time;

        void main() {
            vec2 st = gl_FragCoord.xy/u_resolution;
            gl_FragColor = vec4(u_mouse,0.0,1.0);
        }
    "#;

    let v_shader = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, v_shader, f_shader, None)?;
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);
    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.5, -0.5],
        },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape)?;
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
    let mut t: f32 = 0.0;
    let _ = event_loop.run(move |event, window_target| {
        if let winit::event::Event::WindowEvent { event, .. } = event {
            if let winit::event::WindowEvent::CloseRequested = event {
                window_target.exit();
            } else if let winit::event::WindowEvent::RedrawRequested = event {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 255.0, 1.0);

                t += 0.005;
                let res = [
                    window.inner_size().width as f32 / 1000.,
                    window.inner_size().height as f32 / 1000.,
                ];
                println!("{:?}", res);
                let pos = window.inner_position().unwrap();
                let m_pos = [pos.x as f32 / 1000., pos.y as f32 / 1000.];
                print!("{:?}", m_pos);
                frame
                    .draw(
                        &vertex_buffer,
                        indices,
                        &program,
                        &glium::uniform! { u_time: t, u_mouse: m_pos, u_resolution: res },
                        &Default::default(),
                    )
                    .unwrap();
                frame.finish().unwrap();
            }
        } else if let winit::event::Event::AboutToWait = event {
            window.request_redraw();
        }
    });
    Ok(())
}
