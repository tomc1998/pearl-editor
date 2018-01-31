use super::{Buffer, GpuBuffer, Vertex};
use common::Rect;

/// A struct to allow for vertex generation into a target T. Contains helper methods for painting.
pub struct Painter<'a, T: Buffer + 'a = GpuBuffer> {
    target: &'a mut T,
}

impl<'a, T: Buffer + 'a> Painter<'a, T> {
    pub fn new(buffer: &'a mut T) -> Painter<'a, T> {
        Painter { target: buffer }
    }

    pub fn fill_rect(&mut self, rect: &Rect, col: &[u8; 4]) {
        self.target.buffer(
            &[
            Vertex { 
                pos: [rect.left(), rect.top()],
                uv: [0.0, 0.0],
                col: col.clone(),
            },
            Vertex { 
                pos: [rect.right(), rect.top()],
                uv: [0.0, 0.0],
                col: col.clone(),
            },
            Vertex { 
                pos: [rect.right(), rect.bottom()],
                uv: [0.0, 0.0],
                col: col.clone(),
            },

            Vertex { 
                pos: [rect.left(), rect.top()],
                uv: [0.0, 0.0],
                col: col.clone(),
            },
            Vertex { 
                pos: [rect.left(), rect.bottom()],
                uv: [0.0, 0.0],
                col: col.clone(),
            },
            Vertex { 
                pos: [rect.right(), rect.bottom()],
                uv: [0.0, 0.0],
                col: col.clone(),
            },
            ],
        );
    }
}
