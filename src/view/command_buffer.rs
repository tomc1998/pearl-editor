//! Rendering code for the command buffer.

use cgmath;
use std;
use state;
use qgfx::{RendererController, FontHandle};

pub struct CommandBufferView {
    pub state: std::sync::Arc<state::State>,

    pub font: FontHandle,
}

impl CommandBufferView {
    pub fn new(state: std::sync::Arc<state::State>, font: FontHandle) -> CommandBufferView {
        CommandBufferView {
            state: state,
            font: font,
        }
    }

    /// # Params
    /// * `display_size` - The size of the current display, so that the command buffer can be
    /// rendered at the bottom of the screen.
    pub fn render(&self, g: &mut RendererController, display_size: cgmath::Vector2<f32>) {
        if self.state.command_buffer.lock().unwrap().is_empty() {
            return;
        }
        g.rect(
            &[0.0, display_size.y - 24.0, display_size.x, 24.0],
            &[0.1, 0.1, 0.4, 1.0],
        );
        g.text(
            &self.state.command_buffer.lock().unwrap().get_input_as_str(),
            &[8.0, display_size.y - 8.0],
            self.font,
            &[1.0, 1.0, 1.0, 1.0],
        );
    }
}
