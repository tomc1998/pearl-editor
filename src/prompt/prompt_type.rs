use super::Prompt;
use state::State;
use std::sync::Arc;

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

    /// Suggest completionsof a given string based on the type of this type and the current
    /// program state. Returns vec of len 0 if no completion available.
    pub fn complete(&self, state: Arc<State>, input: &str) -> Vec<String> {
        match *self {
            PromptType::String(_) => Vec::new(),
            PromptType::Package(_) => {
                state
                    .project
                    .pkg_completion_list
                    .lock()
                    .unwrap()
                    .find_all_subsequences(input)
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect()
            }
        }
    }
}
