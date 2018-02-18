extern crate quick_gfx as qgfx;
extern crate cgmath;
extern crate winit;
extern crate smallvec;

mod common;
mod command;
mod prompt;
mod view;
mod java_model;
mod state;
mod input;
mod search;

use std::collections::HashSet;
use java_model::*;
use std::boxed::Box;

/// Poll the command buffer & execute the command
fn poll_cmd_buffer(state: std::sync::Arc<state::State>) {
    // Poll command buffer & execute command
    use command::*;
    use prompt::{PromptType as PT, Prompt as P};
    use state::Selection;
    match state.command_buffer.lock().unwrap().poll_cmd() {
        Some(Command::Create(CreateCommand(CreateObject::Class))) => {
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
                    class.name = data[1].clone();
                    let pkg_name = &data[0];
                    let pkg = state_clone.project.add_package(&pkg_name);
                    unsafe {
                        (*pkg).decl_list.push(Declaration::Class(class));
                    }
                }),
            );
        }
        Some(Command::Create(CreateCommand(CreateObject::Package))) => {
            let state_clone = state.clone();
            state::State::prompt(
                state.clone(),
                vec![PT::Package(P::new("Package Name"))],
                Box::new(move |data| {
                    let pkg_name = &data[0];
                    state_clone.project.add_package(&pkg_name);
                }),
            );
        }
        Some(Command::Select(SelectCommand(SelectObject::Package))) => {
            let state_clone = state.clone();
            state::State::prompt(
                state.clone(),
                vec![PT::Package(P::new_empty_allowed("Package Name"))],
                Box::new(move |data| {
                    let pkg_name = &data[0];
                    // If default package, just set to none
                    if state_clone.project.package_exists(pkg_name) && pkg_name.len() > 0 {
                        *state_clone.project.curr_sel.lock().unwrap() =
                            Some(Selection::Package(pkg_name.clone()));
                    } else {
                        *state_clone.project.curr_sel.lock().unwrap() = None;
                    }
                }),
            );
        }
        None => (),
    }
}

fn main() {
    // Initialise state
    let state = std::sync::Arc::new(state::State::new());

    // Initialise window
    let mut g = qgfx::QGFX::new();

    // Load font
    let mut charsets = HashSet::new();
    charsets.insert(qgfx::Charset::Lowercase);
    charsets.insert(qgfx::Charset::Uppercase);
    charsets.insert(qgfx::Charset::Numbers);
    charsets.insert(qgfx::Charset::Punctuation);
    let fh = g.cache_glyphs(
        "assets/FreeMonoBold.ttf",
        16.0,
        &qgfx::gen_charset(&charsets)[..],
    ).unwrap();

    let mut closed = false;

    for jj in 0..3 {
        let mut declarations = Vec::new();
        for ii in 0..3 {
            let mut class = Class::new_empty();
            class.name = format!("MyClass{}", ii);
            declarations.push(Declaration::Class(class));
        }
        let p = state.project.add_package(&format!("com.tom.package{}", jj));
        unsafe {
            (*p).decl_list = declarations;
        }
    }

    // Create views
    let package_view = view::PackageListView::new(state.clone(), fh);
    let command_buffer_view = view::CommandBufferView::new(state.clone(), fh);
    let prompt_input_view = view::PromptInputView::new(state.clone(), fh);

    while !closed {
        {
            let (display_w, display_h) = g.get_display_size();
            let mut controller = g.get_renderer_controller();
            package_view.render(&mut controller, cgmath::Vector2::new(0.0, 0.0));
            command_buffer_view.render(
                &mut controller,
                cgmath::Vector2::new(display_w as f32, display_h as f32),
            );
            prompt_input_view.render(
                &mut controller,
                cgmath::Vector2::new(display_w as f32, display_h as f32),
            );
            controller.flush();
        }

        g.recv_data();
        g.render();

        g.poll_events(|ev| match ev {
            qgfx::Event::WindowEvent {
                event: ev,
                window_id: _,
            } => {
                if !state::State::process_input(state.clone(), &ev) {
                    match ev {
                        qgfx::WindowEvent::Closed => closed = true,
                        _ => (),
                    }
                }
            }
            _ => (),
        });

        poll_cmd_buffer(state.clone());
    }
}
