use std::sync::Arc;
use java_model::*;
use state;
use prompt::{PromptType as PT, Prompt as P};
use state::Selection;

/// Called when the user issues a create class command. Creates a create class prompt & updates
/// state when prompt is executed.
pub fn create_class(state: Arc<state::State>) {
    let state_clone = state.clone();
    // If curr package is some, then fill that in
    let curr_sel = match *state.project.curr_sel.lock().unwrap() {
        Some(Selection::Package(ref val)) => Some(val.clone()),
        _ => None,
    };
    state::State::prompt(
        state.clone(),
        vec![
            PT::Package(P::new_exact("Package Name", true, curr_sel)),
            PT::String(P::new("Class Name")),
        ],
        Box::new(move |data| {
            let mut class = Class::new_empty();
            class.name = data[1].val.clone();
            let pkg_name = &data[0].val;
            let pkg = state_clone.project.add_package(&pkg_name);
            unsafe {
                (*pkg).decl_list.push(Declaration::Class(class));
            }
            state_clone.project.regen_decl_completion_list();
        }),
    );
}

/// Called when the user issues a create class command. Creates a create class prompt & updates
/// state when prompt is executed.
pub fn create_package(state: Arc<state::State>) {
    let state_clone = state.clone();
    state::State::prompt(
        state.clone(),
        vec![PT::Package(P::new("Package Name"))],
        Box::new(move |data| {
            let pkg_name = &data[0].val;
            state_clone.project.add_package(&pkg_name);
        }),
    );
}


/// Called when the user issues a create field command. Creates a create field prompt & updates
/// state when prompt is executed.
pub fn create_field(_state: Arc<state::State>) {
    println!("Hello");
    //let state_clone = state.clone();
    //// If curr package is some, then fill that in
    //let curr_sel = match *state.project.curr_sel.lock().unwrap() {
    //    Some(Selection::Package(ref val)) => Some(val.clone()),
    //    _ => None,
    //};
    //state::State::prompt(
    //    state.clone(),
    //    vec![
    //        PT::Package(P::new_exact("Package Name", true, curr_sel)),
    //        PT::String(P::new("Class Name")),
    //    ],
    //    Box::new(move |data| {
    //        let mut class = Class::new_empty();
    //        class.name = data[1].val.clone();
    //        let pkg_name = &data[0].val;
    //        let pkg = state_clone.project.add_package(&pkg_name);
    //        unsafe {
    //            (*pkg).decl_list.push(Declaration::Class(class));
    //        }
    //        state_clone.project.regen_decl_completion_list();
    //    }),
    //);
}