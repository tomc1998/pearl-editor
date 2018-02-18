use state::State;
use std::sync::Arc;
use winit::VirtualKeyCode as VKC;
use std::boxed::Box;
use input::InputChunk;

mod prompt_type;

pub use self::prompt_type::PromptType;

/// A prompt. The first string is the text of the prompt - what is displayed to the user - and the
/// boolean indicates whether or not to accept empty input. True for empty allowed - false if empty
/// should be rejected.
pub struct Prompt(pub String, pub bool);

/// Generic text prompt. Contains a list of prompts, which are used to separate the prompt into
/// sections - i.e. the promptinput may need to prompt for a package and a class name.
pub struct PromptInput {
    /// The list of things we're prompting the user for
    pub prompts: Vec<PromptType>,

    /// The user's input
    pub inputs: Vec<String>,

    /// Called when the user finishes the prompt. Takes a slice of user inputs, the same length as
    /// the length of prompts.
    callback: Box<FnMut(&[String])>,

    /// The index of the current prompt
    curr_prompt: usize,

    /// A list of the current completions
    curr_completions: Vec<String>,

    /// Contains the index of the completion to use, or None if no completion selected
    active_completion: Option<usize>,
}

impl PromptInput {
    pub fn new(prompts: Vec<PromptType>, callback: Box<FnMut(&[String])>) -> PromptInput {
        if prompts.len() == 0 {
            panic!("Creating a prompt of length 0")
        }
        PromptInput {
            inputs: vec!["".to_owned(); prompts.len()],
            prompts: prompts,
            callback: callback,
            curr_prompt: 0,
            curr_completions: Vec::new(),
            active_completion: None,
        }
    }

    pub fn get_active_completion(&self) -> Option<usize> {
        self.active_completion
    }

    pub fn get_completions(&self) -> &[String] {
        &self.curr_completions[..]
    }

    /// Key input for 'control' inputs, like S-<TAB> for example
    pub fn key_input(&mut self, i: InputChunk) {
        match (i.0, i.1) {
            (VKC::Tab, 0b1000) => {
                if self.active_completion.is_some() {
                    // Decrement completion
                    if self.active_completion.unwrap() == 0 {
                        self.active_completion = None
                    } else {
                        self.active_completion = Some(self.active_completion.unwrap() - 1);
                    }
                } else {
                    // Return to prev prompt
                    if self.curr_prompt > 0 {
                        self.curr_prompt -= 1;
                    }
                }
            }
            (VKC::Tab, 0b0000) => {
                // Select completion
                if self.active_completion.is_none() {
                    if self.curr_completions.len() > 0 {
                        self.active_completion = Some(0);
                    }
                } else if self.active_completion.unwrap() == self.curr_completions.len() - 1 {
                    self.active_completion = None;
                } else {
                    self.active_completion = Some(self.active_completion.unwrap() + 1);
                }
            }
            (VKC::Back, _) => {
                if self.inputs[self.curr_prompt].len() > 0 {
                    self.inputs[self.curr_prompt].pop();
                }
            }
            _ => (),
        }
    }

    /// Call to input a char in the prompt. If the user finished the prompt with this input, the
    /// callback will be called.
    ///
    /// Returns true if prompt finished here.
    pub fn char_input(&mut self, c: char) -> bool {
        match c {
            '\r' | '\n' => {
                self.curr_prompt += 1;
                if self.curr_prompt >= self.prompts.len() {
                    (self.callback)(&self.inputs[..]);
                    return true;
                }
            }
            c => {
                if !c.is_control() {
                    // If we have a completion selected, then we select that before inserting the
                    // next char
                    if self.active_completion.is_some() {
                        self.inputs[self.curr_prompt] =
                            self.curr_completions[self.active_completion.unwrap()].clone();
                        self.active_completion = None;
                    }
                    self.inputs[self.curr_prompt].push(c);
                }
            }
        }
        return false;
    }

    /// Update the completions on this prompt
    pub fn update_completions(&mut self, state: Arc<State>) {
        self.curr_completions =
            self.prompts[self.curr_prompt].complete(state, &self.inputs[self.curr_prompt])
    }

    /// Get the index of the current prompt we're editing
    pub fn get_curr_prompt(&self) -> usize {
        self.curr_prompt
    }

    /// Get the current user's input
    pub fn get_curr_input(&self) -> &str {
        &self.inputs[self.curr_prompt]
    }
}
