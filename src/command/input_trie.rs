//! An implementation of a trie for holding the current input.

use smallvec::SmallVec;
use winit::VirtualKeyCode as VKC;
use super::{InputChunk, Command};
use std::fmt;
use std::error::Error;

/// Convert a char to keycode if possible.
fn char_to_keycode(c: char) -> Option<VKC> {
    match c {
        'a' | 'A' => Some(VKC::A),
        'b' | 'B' => Some(VKC::B),
        'c' | 'C' => Some(VKC::C),
        'd' | 'D' => Some(VKC::D),
        'e' | 'E' => Some(VKC::E),
        'f' | 'F' => Some(VKC::F),
        'g' | 'G' => Some(VKC::G),
        'h' | 'H' => Some(VKC::H),
        'i' | 'I' => Some(VKC::I),
        'j' | 'J' => Some(VKC::J),
        'k' | 'K' => Some(VKC::K),
        'l' | 'L' => Some(VKC::L),
        'm' | 'M' => Some(VKC::M),
        'n' | 'N' => Some(VKC::N),
        'o' | 'O' => Some(VKC::O),
        'p' | 'P' => Some(VKC::P),
        'q' | 'Q' => Some(VKC::Q),
        'r' | 'R' => Some(VKC::R),
        's' | 'S' => Some(VKC::S),
        't' | 'T' => Some(VKC::T),
        'u' | 'U' => Some(VKC::U),
        'v' | 'V' => Some(VKC::V),
        'w' | 'W' => Some(VKC::W),
        'x' | 'X' => Some(VKC::X),
        'y' | 'Y' => Some(VKC::Y),
        'z' | 'Z' => Some(VKC::Z),
        _ => None,
    }
}

pub type NodeRef = usize;

#[derive(Clone, Debug)]
struct TrieNode {
    /// The input for this node.
    input: InputChunk,

    /// A short display name for this node - useful for GUI.
    display_name: SmallVec<[char; 16]>,

    /// This node's children
    children: SmallVec<[NodeRef; 32]>,

    /// Command if this is a leaf
    command: Option<Command>,
}

#[derive(Clone, Debug)]
pub struct InputTrie {
    trie_nodes: Vec<TrieNode>,
    roots: SmallVec<[NodeRef; 32]>,
}

impl InputTrie {
    pub fn new() -> InputTrie {
        InputTrie {
            trie_nodes: Vec::new(),
            roots: SmallVec::new(),
        }
    }

    /// Panics if ref invalid
    #[allow(dead_code)]
    fn get_node(&self, r: NodeRef) -> &TrieNode {
        &self.trie_nodes[r]
    }
    /// Panics if ref invalid
    #[allow(dead_code)]
    fn get_node_mut(&mut self, r: NodeRef) -> &mut TrieNode {
        &mut self.trie_nodes[r]
    }

    /// Given a node ref and some input, return the next node ref in the tree after applying the
    /// input, or none if nothing exists.
    ///
    /// If the input node is none, this method will look through root nodes.
    pub fn advance_node_ref(&self, r: Option<NodeRef>, input: InputChunk) -> Option<NodeRef> {
        if r.is_none() {
            for c in &self.roots {
                if self.trie_nodes[*c].input == input {
                    return Some(*c);
                }
            }
        } else {
            for c in &self.get_node(r.unwrap()).children {
                if self.trie_nodes[*c].input == input {
                    return Some(*c);
                }
            }
        }
        return None;
    }

    /// Panics if ref invalid
    pub fn get_cmd(&self, r: NodeRef) -> Option<&Command> {
        self.trie_nodes[r].command.as_ref()
    }

    /// Convenience method to map a string to a command. Will panic if string contains a
    /// non-alphabetic char.
    pub fn add_cmd_str(&mut self, cmd_str: &str, cmd: Command) -> Result<(), AddCommandError> {
        // Loop over all the chars and convert into input chunks
        let input_chunks: Vec<InputChunk> = cmd_str
            .chars()
            .map(|c| {
                InputChunk(
                    char_to_keycode(c).unwrap(),
                    if c.is_uppercase() { 0b1000 } else { 0b0000 },
                )
            })
            .collect();

        if input_chunks.len() == 0 {
            return Err(AddCommandError::CommandEmpty);
        }

        // Special case for root nodes
        let mut curr_node = None;
        for c in &self.roots {
            let n = &mut self.trie_nodes[*c];
            if n.input == input_chunks[0] {
                curr_node = Some(*c);
                break;
            }
        }
        if curr_node.is_none() {
            self.trie_nodes.push(TrieNode {
                input: input_chunks[0],
                display_name: SmallVec::new(),
                children: SmallVec::new(),
                command: None,
            });
            self.roots.push(self.trie_nodes.len() - 1);
            curr_node = Some(self.trie_nodes.len() - 1);
        }

        // Trace down the trie until we get to a leaf, or until we need to create a new node.
        let mut consumed = 1;
        let mut curr_node = curr_node.unwrap();
        'outer: for i in &input_chunks[1..] {
            for c in &self.trie_nodes[curr_node].children {
                if self.trie_nodes[*c].input == *i {
                    curr_node = *c;
                    consumed += 1;
                    continue 'outer;
                }
            }
            // If we're here, there are no valid children - if this is a leaf, throw an error,
            // otherwise break & we can add child nodes
            if self.get_node(curr_node).command.is_some() {
                return Err(AddCommandError::CommandAlreadyPrefixed);
            }
            break;
        }

        // Loop until 1 before the end, which will be a leaf
        for i in &input_chunks[consumed..input_chunks.len() - 1] {
            self.trie_nodes.push(TrieNode {
                input: *i,
                display_name: SmallVec::new(),
                children: SmallVec::new(),
                command: None,
            });
            let last_ix = self.trie_nodes.len() - 1;
            self.trie_nodes[curr_node].children.push(last_ix);
            curr_node = self.trie_nodes.len() - 1;
        }

        // At the end now, add the leaf
        self.trie_nodes.push(TrieNode {
            input: input_chunks[input_chunks.len() - 1],
            display_name: SmallVec::new(),
            children: SmallVec::new(),
            command: Some(cmd),
        });
        let last_ix = self.trie_nodes.len() - 1;
        self.trie_nodes[curr_node].children.push(last_ix);

        return Ok(());
    }
}

#[derive(Debug)]
pub enum AddCommandError {
    CommandAlreadyPrefixed,
    CommandEmpty,
}

impl fmt::Display for AddCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AddCommandError {
    fn description(&self) -> &str {
        "Failed to add command"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use command::{Command, CreateCommand, CreateObject};

    #[test]
    pub fn test_str_cmd_insertion() {
        let mut input_trie = InputTrie::new();
        input_trie
            .add_cmd_str("cc", Command::Create(CreateCommand(CreateObject::Class)))
            .unwrap();
        input_trie
            .add_cmd_str("cp", Command::Create(CreateCommand(CreateObject::Package)))
            .unwrap();
        assert_eq!(input_trie.roots.len(), 1);
        assert_eq!(input_trie.trie_nodes.len(), 3);
    }

    #[test]
    pub fn test_advance_node_ref() {
        let mut input_trie = InputTrie::new();
        input_trie
            .add_cmd_str("cc", Command::Create(CreateCommand(CreateObject::Class)))
            .unwrap();
        input_trie
            .add_cmd_str("cp", Command::Create(CreateCommand(CreateObject::Package)))
            .unwrap();
        assert_eq!(input_trie.advance_node_ref(None, InputChunk(VKC::C, 0)), Some(0));
        assert_eq!(input_trie.advance_node_ref(Some(0), InputChunk(VKC::C, 0)), Some(1));
        assert_eq!(input_trie.advance_node_ref(Some(0), InputChunk(VKC::P, 0)), Some(2));
        assert_eq!(input_trie.advance_node_ref(Some(1), InputChunk(VKC::P, 0)), None);
        assert_eq!(input_trie.advance_node_ref(Some(2), InputChunk(VKC::P, 0)), None);
    }
}
