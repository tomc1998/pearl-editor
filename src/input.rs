use winit::VirtualKeyCode as VKC;

/// Modifiers represented using the 4 least significant bits of the given number.
/// Order (from most significant to least) is: shift, ctrl, alt, logo (windows
/// key).
/// # Example
/// * `0b1000` would be just the shift key.
pub type Modifiers = u8;

/// A chunk of input, containing a key code and modifiers.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InputChunk(pub VKC, pub Modifiers);

impl InputChunk {
    /// Convert this input chunk to a string
    pub fn to_str(&self) -> &'static str {
        match self.0 {
            VKC::A => if self.1 & 8 > 0 { "A" } else { "a" },
            VKC::B => if self.1 & 8 > 0 { "B" } else { "b" },
            VKC::C => if self.1 & 8 > 0 { "C" } else { "c" },
            VKC::D => if self.1 & 8 > 0 { "D" } else { "d" },
            VKC::E => if self.1 & 8 > 0 { "E" } else { "e" },
            VKC::F => if self.1 & 8 > 0 { "F" } else { "f" },
            VKC::G => if self.1 & 8 > 0 { "G" } else { "g" },
            VKC::H => if self.1 & 8 > 0 { "H" } else { "h" },
            VKC::I => if self.1 & 8 > 0 { "I" } else { "i" },
            VKC::J => if self.1 & 8 > 0 { "J" } else { "j" },
            VKC::K => if self.1 & 8 > 0 { "K" } else { "k" },
            VKC::L => if self.1 & 8 > 0 { "L" } else { "l" },
            VKC::M => if self.1 & 8 > 0 { "M" } else { "m" },
            VKC::N => if self.1 & 8 > 0 { "N" } else { "n" },
            VKC::O => if self.1 & 8 > 0 { "O" } else { "o" },
            VKC::P => if self.1 & 8 > 0 { "P" } else { "p" },
            VKC::Q => if self.1 & 8 > 0 { "Q" } else { "q" },
            VKC::R => if self.1 & 8 > 0 { "R" } else { "r" },
            VKC::S => if self.1 & 8 > 0 { "S" } else { "s" },
            VKC::T => if self.1 & 8 > 0 { "T" } else { "t" },
            VKC::U => if self.1 & 8 > 0 { "U" } else { "u" },
            VKC::V => if self.1 & 8 > 0 { "V" } else { "v" },
            VKC::W => if self.1 & 8 > 0 { "W" } else { "w" },
            VKC::X => if self.1 & 8 > 0 { "X" } else { "x" },
            VKC::Y => if self.1 & 8 > 0 { "Y" } else { "y" },
            VKC::Z => if self.1 & 8 > 0 { "Z" } else { "z" },
            _ => "",
        }
    }
}

