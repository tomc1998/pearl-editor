use glium;
use super::Vertex;

/// Basic interface for buffers.
pub trait Buffer {
    /// Buffer the given vertex array.
    fn buffer(&mut self, v_buf: &[Vertex]);

    fn clear(&mut self);
}

const GPU_BUFFER_SIZE: usize = 262144;

/// Fixed size GPU buffer. Buffer straight to the GPU like a stack. Overflows panic.
/// GPU_BUFFER_SIZE is the size of the gpu buffers.
pub struct GpuBuffer {
    vbo: glium::VertexBuffer<Vertex>,
    /// Tracks top of vertex buffer
    size: usize,
}

impl GpuBuffer {
    pub fn new(display: &glium::Display) -> GpuBuffer {
        GpuBuffer {
            vbo: glium::VertexBuffer::dynamic(display, &[Vertex::zero(); GPU_BUFFER_SIZE]).unwrap(),
            size: 0,
        }
    }

    /// Get a slice of the current buffered data
    pub fn get_buffer(&self) -> glium::vertex::VertexBufferSlice<Vertex> {
        self.vbo.slice(0..self.size).unwrap()
    }
}

impl Buffer for GpuBuffer {
    /// Buffer the given vertex slice. Will panic if buffer overflows.
    fn buffer(&mut self, v_buf: &[Vertex]) {
        assert!(
            self.size + v_buf.len() <= GPU_BUFFER_SIZE,
            "GPU buffer overflow"
        );
        self.vbo
            .slice(self.size..self.size + v_buf.len())
            .unwrap()
            .write(v_buf);
        self.size += v_buf.len();
    }

    fn clear(&mut self) {
        self.size = 0;
    }
}
