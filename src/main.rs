extern crate quick_gfx as qgfx;
extern crate cgmath;

mod common;

use std::collections::HashSet;

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

    while !closed {
        {
            let controller = g.get_renderer_controller();
            controller
                .text(
                    "The quick brown fox jumps over the lazy dog!",
                    &[128.0, 128.0],
                    fh,
                    &[1.0, 1.0, 1.0, 1.0],
                )
                .unwrap();
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
