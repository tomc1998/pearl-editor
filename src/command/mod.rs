//! Module to handle commands, in a modular way.
//!
//! Commands should be organised similarly to how vim keybindings work, with 'verbs' giving an
//! action and 'nouns' giving the object to operate on.
//!
//! For example, to create a new class one might assign the command 'cc' for Create Class.
//! Expanding on this, we can create many types of declarations - ci for interface, ce for enum,
//! and even expanding to cm / cf for creating methods and fields.

mod input_trie;

use winit::VirtualKeyCode as VKC;

/// Modifiers represented using the 4 least significant bits of the given number.
/// Order (from most significant to least) is: shift, ctrl, alt, logo (windows
/// key).
/// # Example
/// * `0b1000` would be just the shift key.
pub type Modifiers = u8;

/// A chunk of input, many chunks make up a command
pub type InputChunk = (VKC, Modifiers);

/// The noun for the create command - what object are we creating?
#[derive(Clone, Debug)]
pub enum CreateObject {
    Class,
    Package,
}

/// A create command, for creating classes, interfaces, methods etc.
#[derive(Clone, Debug)]
pub struct CreateCommand(CreateObject);

/// A command
#[derive(Clone, Debug)]
pub enum Command {
    Create(CreateCommand),
}

/// Holds 2 butffers storing the user's currently entered command string, and also all the executed
/// commands. When a command string is recognised, its characters are removed from the input buffer
/// and the specific command is added to the command buffer.
pub struct CommandBuffer {
    /// The current input chars. Convenience data structure which is synced up with node_ref.
    input_buf: Vec<InputChunk>,

    /// A list of commands which haven't been executed.
    cmd_buf: Vec<Command>,

    /// A trie containing all the mappings
    input_trie: input_trie::InputTrie,
    /// The reference to the current node ref in the input tree (according to the input_buf)
    node_ref: Option<input_trie::NodeRef>,
}

impl CommandBuffer {
    fn init_mappings() -> input_trie::InputTrie {
        let mut input_trie = input_trie::InputTrie::new();
        input_trie
            .add_cmd_str("cc", Command::Create(CreateCommand(CreateObject::Class)))
            .unwrap();
        input_trie
            .add_cmd_str("cp", Command::Create(CreateCommand(CreateObject::Package)))
            .unwrap();
        return input_trie;
    }

    pub fn new() -> CommandBuffer {
        CommandBuffer {
            input_buf: Vec::new(),
            cmd_buf: Vec::with_capacity(4),
            input_trie: CommandBuffer::init_mappings(),
            node_ref: None,
        }
    }

    /// Add a character to the command buffer. This should be called when user input is detected.
    pub fn add_key(&mut self, input: InputChunk) {
        self.input_buf.push(input);
        self.node_ref = self.input_trie.advance_node_ref(self.node_ref, input);
        if self.node_ref.is_none() {
            return;
        }
        let cmd = self.input_trie.get_cmd(self.node_ref.unwrap());
        if cmd.is_some() {
            self.node_ref = None;
            self.input_buf.clear();
            self.cmd_buf.push(cmd.unwrap().clone());
            println!("Registered command: {:?}", cmd.unwrap());
        }
    }
}

