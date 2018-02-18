//! Module pertaining to application state. All views will keep a reference to this state.

use winit;
use common;
use java_model::*;
use command;
use prompt;
use std::sync::{Arc, Mutex};
use qgfx;
use input;
use search::SearchBuffer;

pub struct Project {
    pub package_list: Mutex<Vec<Package>>,

    /// A searchable list of strings for autocompleting packages
    pub pkg_completion_list: Mutex<SearchBuffer>,

    /// A reference to the current package. This will be highlighted when rendering, and allows for
    /// faster editing due to context-aware commands (i.e. create class will already have package
    /// filled in)
    pub curr_pkg: Mutex<Option<String>>,
}

pub struct State {
    pub project: Project,
    pub command_buffer: Mutex<command::CommandBuffer>,
    pub curr_prompt: Mutex<Option<prompt::PromptInput>>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            package_list: Mutex::new(Vec::new()),
            curr_pkg: Mutex::new(None),
            pkg_completion_list: Mutex::new(SearchBuffer::new()),
        }
    }

    /// Regenerate the package completion list.
    pub fn regen_pkg_completion_list(&self) {
        let pkg_completion_list = &mut *self.pkg_completion_list.lock().unwrap();
        pkg_completion_list.clear();
        let package_list = self.package_list.lock().unwrap();
        for p in package_list.iter() {
            pkg_completion_list.add_strings_owned(
                &p.gen_package_completion_list()[..],
            );
        }
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
    pub fn add_package(&self, name: &str) -> *mut Package {
        let first_pkg_name = &name[0..name.find(".").unwrap_or(name.len())];
        let mut package_list = self.package_list.lock().unwrap();
        for p in &mut *package_list {
            if p.name == first_pkg_name {
                return p.add_subpackage(name);
            }
        }
        let (pkg, mut deepest) = Package::new(name);
        package_list.push(pkg);
        if deepest.is_none() {
            deepest = Some(package_list.last_mut().unwrap() as *mut Package);
        }
        return deepest.unwrap();
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
        prompts: Vec<prompt::PromptType>,
        callback: Box<FnMut(&[String])>,
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
