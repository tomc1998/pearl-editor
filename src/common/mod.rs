#![allow(dead_code)]

use cgmath::Vector2;
use winit::ModifiersState;

/// Given a modifiersstate struct, convert that to a bitflag. See the command module for more
/// details.
pub fn mods_to_bitflags(ms: ModifiersState) -> u8 {
    return (if ms.shift { 0b1000 } else { 0 }) + (if ms.ctrl { 0b0100 } else { 0 }) +
        (if ms.alt { 0b0010 } else { 0 }) + (if ms.logo { 0b0001 } else { 0 });
}

pub struct Rect {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            pos: Vector2 { x: x, y: y },
            size: Vector2 { x: w, y: h },
        }
    }

    pub fn left(&self) -> f32 {
        self.pos.x
    }
    pub fn right(&self) -> f32 {
        self.pos.x + self.size.x
    }
    pub fn top(&self) -> f32 {
        self.pos.y
    }
    pub fn bottom(&self) -> f32 {
        self.pos.y + self.size.y
    }
}
