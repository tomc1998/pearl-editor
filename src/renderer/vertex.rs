#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub col: [u8; 4],
}

impl Vertex {
    /// Constructor to initialise this vertex to empty. Useful when creating GPU buffers. */
    pub fn zero() -> Vertex {
        Vertex {
            pos: [0.0; 2],
            uv: [0.0; 2],
            col: [0; 4],
        }
    }
}

implement_vertex!(Vertex, pos, uv, col);
