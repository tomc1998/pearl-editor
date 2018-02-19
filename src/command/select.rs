/// Handles processing of user select commands

use std::sync::Arc;
use state;
use prompt::{PromptType as PT, Prompt as P};
use state::Selection;

/// Called when the user issues a create class command. Creates a create class prompt & updates
/// state when prompt is executed.
pub fn select_package(state: Arc<state::State>) {
    let state_clone = state.clone();
    state::State::prompt(
        state.clone(),
        vec![PT::Package(P::new_empty_allowed("Package Name"))],
        Box::new(move |data| {
            // If didn't find a completion, then just assume this is a bad pkg name
            let (name, completion_match) = (&data[0].val, data[0].completion_match);
            if completion_match {
                *state_clone.project.curr_sel.lock().unwrap() =
                    Some(Selection::Package(name.clone()));
            } else {
                *state_clone.project.curr_sel.lock().unwrap() = None;
            }
        }),
    );
}

/// Called when the user issues a select decl command.
pub fn select_decl(state: Arc<state::State>) {
    let state_clone = state.clone();
    state::State::prompt(
        state.clone(),
        vec![PT::Decl(P::new_empty_allowed("Name"))],
        Box::new(move |data| {
            // If didn't find a completion, then just assume this is a bad class name
            let (name, completion_match) = (&data[0].val, data[0].completion_match);
            if completion_match {
                *state_clone.project.curr_sel.lock().unwrap() = Some(Selection::Decl(name.clone()));
            } else {
                *state_clone.project.curr_sel.lock().unwrap() = None;
            }
        }),
    );
}
