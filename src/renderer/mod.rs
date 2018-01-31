mod vertex;
mod buffer;
mod painter;

pub use self::vertex::Vertex;
pub use self::buffer::{Buffer, GpuBuffer};
pub use self::painter::Painter;

use glium;

pub struct Renderer {
    /// Main buffer for vertex data.
    main_buffer: GpuBuffer,
    program: glium::Program,
}

impl Renderer {
    pub fn new(display: &glium::Display) -> Renderer {
        let vertex_shader_src = r#"
            #version 140
            in vec2 pos;
            void main() {
                gl_Position = vec4(pos, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 col;
            void main() {
                col = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        Renderer {
            main_buffer: GpuBuffer::new(display),
            program: glium::Program::from_source(
                display,
                vertex_shader_src,
                fragment_shader_src,
                None,
            ).unwrap(),
        }
    }

    /// Render the current buffer to the screen. This will clear all the current vertices.
    pub fn render<S: glium::Surface>(&self, target: &mut S) {
        let buf = self.main_buffer.get_buffer();
        target
            .draw(
                buf,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }

    pub fn get_painter(&mut self) -> Painter {
        Painter::new(&mut self.main_buffer)
    }
}
