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

use std::collections::HashSet;
use java_model::*;
use std::boxed::Box;

/// Poll the command buffer & execute the command
fn poll_cmd_buffer(state: std::sync::Arc<state::State>) {
    // Poll command buffer & execute command
    use command::*;
    match state.command_buffer.lock().unwrap().poll_cmd() {
        Some(Command::Create(CreateCommand(CreateObject::Class))) => {
            state::State::prompt(state.clone(),
                vec!["Package Name".to_owned(), "Class Name".to_owned()],
                Box::new(|data| {
                    println!("PROMPTED: {:?}", data);
                }),
            );
        }
        Some(Command::Create(CreateCommand(CreateObject::Package))) => {
            println!("Creating package");
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
        let mut p = Package::new(format!("com.tom.package{}", jj));
        p.decl_list = declarations;
        state.project.package_list.lock().unwrap().push(p);
    }

    // Create views
    let package_view = view::PackageListView::new(state.clone(), fh);
    let command_buffer_view = view::CommandBufferView::new(state.clone(), fh);

    while !closed {
        {
            let (display_w, display_h) = g.get_display_size();
            let controller = g.get_renderer_controller();
            package_view.render(&controller, cgmath::Vector2::new(0.0, 0.0));
            command_buffer_view.render(
                &controller,
                cgmath::Vector2::new(display_w as f32, display_h as f32),
            );
        }

        g.recv_data();
        g.render();

        g.poll_events(|ev| match ev {
            qgfx::Event::WindowEvent {
                event: ev,
                window_id: _,
            } => {
                if !state.process_input(&ev) {
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
