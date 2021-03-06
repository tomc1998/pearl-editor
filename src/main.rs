// Setup benching (as long as bench feature enabled)
#![cfg_attr(feature = "bench", feature(test))]
#[cfg(feature = "bench")]
extern crate test;

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

/// Poll the command buffer & execute the command
fn poll_cmd_buffer(state: std::sync::Arc<state::State>) {
    // Poll command buffer & execute command
    use command::*;
    match state.command_buffer.lock().unwrap().poll_cmd() {
        Some(Command::Create(CreateCommand(CreateObject::Class))) => {
            command::create_class(state.clone());
        }
        Some(Command::Create(CreateCommand(CreateObject::Field))) => {
            command::create_field(state.clone());
        }
        Some(Command::Create(CreateCommand(CreateObject::Package))) => {
            command::create_package(state.clone());
        }
        Some(Command::Select(SelectCommand(SelectObject::Package))) => {
            command::select_package(state.clone());
        }
        Some(Command::Select(SelectCommand(SelectObject::Class))) => {
            command::select_decl(state.clone());
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
        12.0,
        &qgfx::gen_charset(&charsets)[..],
    ).unwrap();

    let mut closed = false;

    for jj in 0..3 {
        for ii in 0..3 {
            let class = Class::new_with_name(&format!("MyClass{}", ii));
            state.project.add_decl(&format!("com.tom.package{}", jj), Declaration::Class(class));
        }
    }
    state.project.regen_decl_completion_list();
    state.project.regen_pkg_completion_list();

    // Create views
    let package_view = view::PackageListView::new(state.clone(), fh);
    let command_buffer_view = view::CommandBufferView::new(state.clone(), fh);
    let prompt_input_view = view::PromptInputView::new(state.clone(), fh);

    while !closed {
        {
            let (display_w, display_h) = g.get_display_size();
            let screen_size = cgmath::Vector2::new(display_w as f32, display_h as f32);
            let mut controller = g.get_renderer_controller();
            package_view.render(&mut controller, screen_size.clone());
            command_buffer_view.render(&mut controller, screen_size.clone());
            prompt_input_view.render(&mut controller, screen_size.clone());
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
