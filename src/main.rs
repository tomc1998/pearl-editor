extern crate quick_gfx as qgfx;
extern crate cgmath;

mod common;
mod view;
mod java_model;

use std::collections::HashSet;
use java_model::*;

fn main() {
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
    let class_view = view::ClassListView::new(fh);

    let mut classes = Vec::new();
    for ii in 0..3 {
        let mut class = Class::new_empty();
        class.name = format!("Class {}", ii);
        classes.push(class);
    }

    while !closed {
        {
            let controller = g.get_renderer_controller();
            class_view.render(&controller, &classes[..]);
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
                    _ => (),
                }
            }
            _ => (),
        });
    }
}
