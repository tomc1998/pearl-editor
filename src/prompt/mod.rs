use winit::VirtualKeyCode as VKC;
use std::boxed::Box;
use input::InputChunk;

/// Generic text prompt
pub struct Prompt {
    /// The list of things we're prompting the user for
    pub prompts: Vec<String>,

    /// The user's input
    pub inputs: Vec<String>,

    /// Called when the user finishes the prompt. Takes a slice of user inputs, the same length as
    /// the length of prompts. 
    callback: Box<FnMut(&[String])>,

    /// The index of the current prompt
    curr_prompt: usize,
}

impl Prompt {
    pub fn new(prompts: Vec<String>, callback: Box<FnMut(&[String])>) -> Prompt {
        if prompts.len() == 0 {
            panic!("Creating a prompt of length 0")
        }
        Prompt {
            inputs: vec!["".to_owned(); prompts.len()],
            prompts: prompts,
            callback: callback,
            curr_prompt: 0,
        }
    }

    /// Key input for 'control' inputs, like S-<TAB> for example
    pub fn key_input(&mut self, i: InputChunk) {
        if i.0 == VKC::Tab && i.1 == 0b1000 { // S-<TAB>
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
            c => {
                self.inputs[self.curr_prompt].push(c)
            }
        }
        return false;
    }
}
