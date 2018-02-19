//! Module pertaining to application state. All views will keep a reference to this state.

mod project;

pub use self::project::*;

use winit;
use common;
use command;
use prompt;
use std::sync::{Arc, Mutex};
use qgfx;
use input;
use prompt::PromptResult;

pub struct State {
    pub project: Project,
    pub command_buffer: Mutex<command::CommandBuffer>,
    pub curr_prompt: Mutex<Option<prompt::PromptInput>>,
}

impl State {
    pub fn new() -> State {
        State {
            project: Project::new(),
            command_buffer: Mutex::new(command::CommandBuffer::new()),
            curr_prompt: Mutex::new(None),
        }
    }

    /// Prompt the user for some input. Returns false if user is already being prompted.
    /// # Params
    /// * `this` - The state in an arc. This is needed because this prompt method creates a closure
    /// which needs to reference self.
    /// * `prompts` - The items to prompt the user for
    /// * `callback` - Called when user completes prompt. Accepts a slice of strings, which are the
    /// user's inputs (corresponding to the given prompts)
    pub fn prompt(
        this: Arc<State>,
        prompts: Vec<prompt::PromptType>,
        callback: Box<FnMut(&[PromptResult])>,
    ) -> bool {
        let mut prompt = this.curr_prompt.lock().unwrap();
        if prompt.is_some() {
            return true;
        } else {
            *prompt = Some(prompt::PromptInput::new(prompts, callback));
            true
        }
    }

    /// Returns true if input was used
    pub fn process_input(this: Arc<State>, ev: &qgfx::WindowEvent) -> bool {
        match *ev {
            qgfx::WindowEvent::KeyboardInput {
                device_id: _,
                input: k,
            } => {
                if k.virtual_keycode.is_some() && k.state == winit::ElementState::Pressed {
                    // C-g cancels everything
                    if common::mods_to_bitflags(k.modifiers) == 0b0100 &&
                        k.virtual_keycode.unwrap() == winit::VirtualKeyCode::G
                    {
                        this.command_buffer.lock().unwrap().reset_input();
                        *this.curr_prompt.lock().unwrap() = None;
                    }
                    let i = input::InputChunk::from_modifiers_state(
                        k.virtual_keycode.unwrap(),
                        k.modifiers,
                    );
                    // If prompt is showing, send data to that first
                    if this.curr_prompt.lock().unwrap().is_some() {
                        let mut curr_prompt = this.curr_prompt.lock().unwrap();
                        curr_prompt.as_mut().unwrap().key_input(i);
                        curr_prompt.as_mut().unwrap().update_completions(
                            this.clone(),
                        );
                    } else {
                        // Otherwise, send data to the command buffer
                        // Special case, clear the command buffer on C-g
                        (*this.command_buffer.lock().unwrap()).add_key(i);
                    }
                    return true;
                }
            }
            qgfx::WindowEvent::ReceivedCharacter(c) => {
                if this.curr_prompt.lock().unwrap().is_some() {
                    let mut curr_prompt = this.curr_prompt.lock().unwrap();
                    if curr_prompt.as_mut().unwrap().char_input(c) {
                        *curr_prompt = None;
                    } else {
                        curr_prompt.as_mut().unwrap().update_completions(
                            this.clone(),
                        );
                    }
                }
            }
            _ => (),
        }
        return false;
    }
}
