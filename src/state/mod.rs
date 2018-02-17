//! Module pertaining to application state. All views will keep a reference to this state.

use winit;
use common;
use java_model::*;
use command;
use prompt;
use std::sync::{Arc, Mutex};
use qgfx;
use input;

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

    /// Add a fully qualified package name. If the start of the package name is already used, trace
    /// down the tree and insert new package in the appropriate replaces. Return a mutable
    /// pointer to the last created package.
    /// 
    /// This will lock the package list mutex, and the mutex will stay locked whilst you hold the
    /// package reference.
    /// 
    /// # Caution
    /// See package::Package::new() for details - the mut pointer returned isn't guaranteed to be
    /// valid forever, and is only a convenience measure to quickly add a class to the deepest
    /// package.
    pub fn add_subpackage(&self, name: &str) -> *mut Package {
        let first_pkg_name = &name[0..name.find(".").unwrap_or(name.len())];
        let mut package_list = self.package_list.lock().unwrap();
        for p in &mut *package_list {
            if p.name == first_pkg_name {
                return p.add_subpackage(name);
            }
        }
        let (pkg, deepest) = Package::new(name);
        package_list.push(pkg);
        return deepest
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
    pub fn prompt(
        this: Arc<State>,
        prompts: Vec<String>,
        callback: Box<FnMut(&[String])>,
    ) -> bool {
        let mut prompt = this.curr_prompt.lock().unwrap();
        if prompt.is_some() {
            return true;
        } else {
            *prompt = Some(prompt::Prompt::new(prompts, callback));
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
                    let i = input::InputChunk::from_modifiers_state(
                        k.virtual_keycode.unwrap(),
                        k.modifiers,
                    );
                    // If prompt is showing, send data to that first
                    if self.curr_prompt.lock().unwrap().is_some() {
                        self.curr_prompt
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .key_input(i);
                    } else {
                        // Otherwise, send data to the command buffer
                        // Special case, clear the command buffer on C-g
                        (*self.command_buffer.lock().unwrap()).add_key(i);
                    }
                    return true;
                }
            }
            qgfx::WindowEvent::ReceivedCharacter(c) => {
                if self.curr_prompt.lock().unwrap().is_some() {
                    if self.curr_prompt
                        .lock()
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .char_input(c)
                    {
                        *self.curr_prompt.lock().unwrap() = None;
                    }

                }
            }
            _ => (),
        }
        return false;
    }
}
