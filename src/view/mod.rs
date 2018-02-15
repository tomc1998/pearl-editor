//! A module which encompasses generating vertex data to render.

use java_model::*;
use qgfx::{RendererController, FontHandle};
use cgmath;

pub struct ClassListView {
    pub offset: cgmath::Vector2<f32>,

    /// The width of this view
    pub width: f32,

    /// The height of the classes when rendered in a list.
    pub class_height: f32,

    pub font: FontHandle,
}

impl ClassListView {
    pub fn new(font: FontHandle) -> ClassListView {
        ClassListView {
            offset: cgmath::Vector2 { x: 0.0, y: 0.0 },
            width: 128.0,
            class_height: 32.0,
            font: font,
        }
    }

    /// Given a renderer controller, render a class list.
    pub fn render(&self, g: &RendererController, class_list: &[Class]) {
        let mut pos = self.offset;
        for c in class_list {
            g.rect(
                &[pos.x, pos.y, self.width, self.class_height],
                &[0.1, 0.1, 0.1, 1.0],
            );
            g.text(&c.name, &[pos.x, pos.y + self.class_height/2.0], self.font, &[1.0, 1.0, 1.0, 1.0]).unwrap();
            pos.y += self.class_height;
        }
    }
}
