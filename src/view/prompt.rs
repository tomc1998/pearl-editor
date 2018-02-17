//! Rendering code for the prompt

use cgmath;
use std;
use state;
use qgfx::{RendererController, FontHandle};

pub struct PromptInputView {
    pub state: std::sync::Arc<state::State>,

    pub font: FontHandle,
}

impl PromptInputView {
    pub fn new(state: std::sync::Arc<state::State>, font: FontHandle) -> PromptInputView {
        PromptInputView {
            state: state,
            font: font,
        }
    }

    /// # Params
    /// * `display_size` - The size of the current display, so that the command buffer can be
    /// rendered at the bottom of the screen.
    pub fn render(&self, g: &RendererController, display_size: cgmath::Vector2<f32>) {
        let prompt = self.state.curr_prompt.lock().unwrap();
        if prompt.is_none() {
            return;
        }
        let prompt = prompt.as_ref().unwrap();

        g.rect(
            &[0.0, display_size.y - 24.0, display_size.x, 24.0],
            &[0.1, 0.1, 0.4, 1.0],
        );

        let mut pos = cgmath::Vector2{x: 0.0, y: 0.0};

        let prompt_col = [0.4, 0.4, 0.9, 0.4];
        let active_prompt_col = [7.0, 7.0, 1.0, 1.0];

        for (ii, p) in prompt.prompts.iter().enumerate() {
            let col;
            if ii == prompt.get_curr_prompt() {
                col = &active_prompt_col;
            } else {
                col = &prompt_col;
            }

            let (w, _h) = g.text(
                p.as_str(),
                &[8.0 + pos.x, display_size.y - 8.0],
                self.font,
                col,
            );
            pos.x += w + 16.0;
        }

        g.text(
            prompt.get_curr_input(),
            &[8.0 + pos.x, display_size.y - 8.0],
            self.font,
            &[1.0, 1.0, 1.0, 1.0],
        );

    }
}

