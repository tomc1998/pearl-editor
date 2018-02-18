use super::Prompt;
use state::State;
use std::sync::Arc;
use java_model::*;
use std::ptr::null;

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
                let package_list = &*state.project.package_list.lock().unwrap();
                let splits = input.split(".");
                let mut curr_pkg: *const Package = null();
                'outer: for s in splits {
                    let pkg_list = if curr_pkg == null() {
                        package_list
                    } else {
                        unsafe { &(*curr_pkg).package_list }
                    };

                    for p in pkg_list {
                        if p.name == s {
                            curr_pkg = p;
                            continue 'outer;
                        }
                    }

                    // If we're here, then the package wasn't found. Now we can generate
                    // completions.
                    let mut completions = Vec::new();
                    for p in pkg_list {
                        if p.name.starts_with(s) {
                            completions.push(p.name.clone());
                        }
                    }
                    return completions;
                }
                return vec![]; // We must have completed the package name completely - return no completions.
            }
        }
    }
}
