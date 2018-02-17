use winit::VirtualKeyCode as VKC;
use std::boxed::Box;
use input::InputChunk;

/// A prompt. The first string is the text of the prompt - what is displayed to the user - and the
/// boolean indicates whether or not to accept empty input. True for empty allowed - false if empty
/// should be rejected.
pub struct Prompt(pub String, pub bool);

/// A section of a PromptInput. Wraps a prompt, and adds useful info regarding autocompletions -
/// for example, a Package(Prompt) will be subject to package autocompletion.
pub enum PromptType {
    /// String prompt - just a straight string.
    String(Prompt),

    /// Package prompt. This allows tab completion for subpackages.
    Package(Prompt),
}

impl PromptType {
    pub fn as_str(&self) -> &str {
        match *self {
            PromptType::String(ref p) => &p.0,
            PromptType::Package(ref p) => &p.0,
        }
    }
}

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
        }
    }

    /// Key input for 'control' inputs, like S-<TAB> for example
    pub fn key_input(&mut self, i: InputChunk) {
        if i.0 == VKC::Tab && i.1 == 0b1000 {
            // S-<TAB>
            if self.curr_prompt > 0 {
                self.curr_prompt -= 1;
            }
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
            c => self.inputs[self.curr_prompt].push(c),
        }
        return false;
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
