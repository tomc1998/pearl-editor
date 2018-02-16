use std::boxed::Box;

/// Generic text prompt
pub struct Prompt {
    /// The list of things we're prompting the user for
    pub prompts: Vec<String>,

    /// The user's input
    pub inputs: Vec<String>,

    /// Called when the user finishes the prompt. Takes a slice of user inputs, the same length as
    /// the length of prompts.
    pub callback: Box<FnOnce(&[String])>,
}

impl Prompt {
    pub fn new<F: 'static + FnOnce(&[String])>(prompts: Vec<String>, callback: F) -> Prompt {
        Prompt {
            inputs: vec!["".to_owned(); prompts.len()],
            prompts: prompts,
            callback: Box::new(callback),
        }
    }

    /// Call to input a char in the prompt. If the user finished the prompt with this input, the
    /// callback will be called.
    pub fn char_input(&self, c: char) {
        match c {
            '\r' | '\n' => println!("NEWLINE"),
            _ => (),
        }
    }
}
