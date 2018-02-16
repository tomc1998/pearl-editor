extern crate quick_gfx as qgfx;
extern crate cgmath;
extern crate winit;
extern crate smallvec;

mod common;
mod command;
mod view;
mod java_model;
mod state;

use std::collections::HashSet;
use java_model::*;

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
                match ev {
                    qgfx::WindowEvent::Closed => closed = true,
                    qgfx::WindowEvent::KeyboardInput {
                        device_id: _,
                        input: k,
                    } => {
                        if k.virtual_keycode.is_some() && k.state == winit::ElementState::Pressed {
                            // Special case, clear the command buffer on C-g
                            if common::mods_to_bitflags(k.modifiers) == 0b0100 &&
                                k.virtual_keycode.unwrap() == winit::VirtualKeyCode::G
                            {
                                state.command_buffer.lock().unwrap().reset_input();
                            } else {
                                (*state.command_buffer.lock().unwrap()).add_key(command::InputChunk(
                                    k.virtual_keycode
                                        .unwrap(),
                                    0,
                                ));
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        });
    }
}
