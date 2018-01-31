extern crate rusttype;
extern crate cgmath;
#[macro_use]
extern crate glium;

mod renderer;
mod common;

fn main() {
    // Setup glium stuff
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Create renderer & painter
    let mut renderer = renderer::Renderer::new(&display);

    let mut closed = false;

    while !closed {
        use glium::{glutin, Surface};

        {
            let mut painter = renderer.get_painter();
            painter.fill_rect(&common::Rect::new(0.0, 0.0, 0.1, 0.1), &[255, 0, 0, 255]);
        }

        let mut surface = display.draw();
        surface.clear_color(0.1, 0.1, 0.2, 1.0);
        renderer.render(&mut surface);
        surface.finish().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                }
            }
            _ => (),
        });
    }
}
