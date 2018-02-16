//! A module which encompasses generating vertex data to render.

use java_model::*;
use qgfx::{RendererController, FontHandle};
use common::Rect;
use cgmath;
use std;
use state;

pub struct PackageListView {
    pub state: std::sync::Arc<state::State>,

    /// The width of this view
    pub width: f32,

    pub font: FontHandle,
}

impl PackageListView {
    pub fn new(state: std::sync::Arc<state::State>, font: FontHandle) -> PackageListView {
        PackageListView {
            state: state,
            width: 128.0,
            font: font,
        }
    }

    /// Helper to render a list of decls. Returns a rect which indicates the size used up by the
    /// classes rendered.
    fn render_decl_list(
        &self,
        g: &RendererController,
        decl_list: &[Declaration],
        offset: cgmath::Vector2<f32>,
        decl_height: f32,
        decl_width: f32,
    ) -> Rect {
        let mut pos = offset;
        for d in decl_list {
            g.rect(
                &[pos.x, pos.y, decl_width, decl_height],
                &[0.1, 0.1, 0.1, 1.0],
            );
            g.text(
                d.name(),
                &[pos.x, pos.y + decl_height / 2.0],
                self.font,
                &[1.0, 1.0, 1.0, 1.0],
            ).unwrap();
            pos.y += decl_height;
        }
        return Rect::new(
            offset.x,
            offset.y,
            decl_width,
            decl_height * decl_list.len() as f32,
        );

    }

    /// Renders a list of packages.
    /// Returns the bounding box used up from rendering.
    pub fn render(
        &self,
        g: &RendererController,
        offset: cgmath::Vector2<f32>,
        packages: &[Package],
    ) {
        let mut pos = offset;
        for p in packages {
            g.rect(
                &[pos.x, pos.y, self.width - 16.0, 32.0],
                &[0.1, 0.1, 0.1, 1.0],
            );
            g.text(
                p.name.as_ref(),
                &[pos.x, pos.y + 32.0 / 2.0],
                self.font,
                &[1.0, 1.0, 1.0, 1.0],
            ).unwrap();
            pos.y += 32.0;

            pos.x += 16.0;
            let rect = self.render_decl_list(g, &p.decl_list[..], pos, 32.0, self.width - 16.0);
            pos.x -= 16.0;
            pos.y += rect.size.y;
        }
    }
}
