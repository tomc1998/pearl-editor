//! Module pertaining to application state. All views will keep a reference to this state.

use winit;
use common;
use java_model::*;
use command;
use prompt;
use std::sync::{Arc, Mutex};
use qgfx;

pub struct Project {
    pub package_list: Arc<Mutex<Vec<Package>>>,
}

pub struct State {
    pub project: Project,
    pub command_buffer: Mutex<command::CommandBuffer>,
    pub curr_prompt: Mutex<Option<prompt::Prompt>>,
}

impl Project {
    pub fn new() -> Project {
        Project { package_list: Arc::new(Mutex::new(Vec::new())) }
    }
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
    pub fn prompt<F: 'static + FnOnce(&[String])>(this: Arc<State>, prompts: Vec<String>, callback: F) -> bool {
        let mut prompt = this.curr_prompt.lock().unwrap();
        if prompt.is_some() {
            return true;
        } else {
            let this = this.clone();
            *prompt = Some(prompt::Prompt::new(prompts, move |data| {
                callback(data);
                *this.curr_prompt.lock().unwrap() = None;
            }));
            true
        }
    }

    /// Returns true if input was used
    pub fn process_input(&self, ev: &qgfx::WindowEvent) -> bool {
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
                        self.command_buffer.lock().unwrap().reset_input();
                        *self.curr_prompt.lock().unwrap() = None;
                    }
                    // If prompt is showing, send data to that first
                    if self.curr_prompt.lock().unwrap().is_some() {
                    } else {
                        // Otherwise, send data to the command buffer
                        // Special case, clear the command buffer on C-g
                        (*self.command_buffer.lock().unwrap()).add_key(
                            command::InputChunk(
                                k.virtual_keycode
                                    .unwrap(),
                                0,
                            ),
                        );
                    }
                    return true;
                }
            }
            qgfx::WindowEvent::ReceivedCharacter(c) => {
                if self.curr_prompt.lock().unwrap().is_some() {
                    self.curr_prompt.lock().unwrap().as_ref().unwrap().char_input(c);
                }
            }
            _ => (),
        }
        return false;
    }
}
