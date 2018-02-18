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
    pub fn render(&self, g: &mut RendererController, display_size: cgmath::Vector2<f32>) {
        let prompt = self.state.curr_prompt.lock().unwrap();
        if prompt.is_none() {
            return;
        }
        let prompt = prompt.as_ref().unwrap();

        const BG_COL: [f32; 4] = [0.1, 0.1, 0.4, 1.0];

        g.rect(&[0.0, display_size.y - 24.0, display_size.x, 24.0], &BG_COL);

        let mut pos = cgmath::Vector2 { x: 0.0, y: 0.0 };

        const PROMPT_COL: [f32; 4] = [0.4, 0.4, 0.9, 0.4];
        const ACTIVE_PROMPT_COL: [f32; 4] = [7.0, 7.0, 1.0, 1.0];

        for (ii, p) in prompt.prompts.iter().enumerate() {
            let col;
            if ii == prompt.get_curr_prompt() {
                col = &ACTIVE_PROMPT_COL;
            } else {
                col = &PROMPT_COL;
            }

            let (w, _h) = g.text(
                p.as_str(),
                &[8.0 + pos.x, display_size.y - 8.0],
                self.font,
                col,
            );
            pos.x += w + 16.0;
        }

        // Render the input, or the completion if active
        match prompt.get_active_completion() {
            Some(ix) => {
                g.text(
                    &prompt.get_completions()[ix],
                    &[8.0 + pos.x, display_size.y - 8.0],
                    self.font,
                    &[1.0, 1.0, 1.0, 1.0],
                );
            }
            _ => {
                g.text(
                    prompt.get_curr_input(),
                    &[8.0 + pos.x, display_size.y - 8.0],
                    self.font,
                    &[1.0, 1.0, 1.0, 1.0],
                );
            }
        }

        const ACTIVE_COMPLETION_COL: [f32; 4] = [0.4, 0.4, 0.7, 1.0];

        // Render completions
        let num_comp = prompt.get_completions().len();
        pos.y += 24.0 * num_comp as f32 + 24.0;
        for (ii, c) in prompt.get_completions().iter().enumerate() {
            // Select colour
            let col = match prompt.get_active_completion() {
                Some(ix) => {
                    if ix == ii {
                        &ACTIVE_COMPLETION_COL
                    } else {
                        &BG_COL
                    }
                }
                _ => &BG_COL,
            };

            // Inc cursor
            pos.y -= 24.0;

            // Render completion
            g.rect(
                &[
                    pos.x,
                    display_size.y - 24.0 - pos.y,
                    display_size.x - pos.x,
                    24.0,
                ],
                col,
            );
            g.text(
                &c,
                &[8.0 + pos.x, display_size.y - 8.0 - pos.y],
                self.font,
                &[1.0, 1.0, 1.0, 1.0],
            );
        }
    }
}
