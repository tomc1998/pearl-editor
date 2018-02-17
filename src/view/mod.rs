//! A module which encompasses generating vertex data to render.

mod command_buffer;

pub use self::command_buffer::*;

use java_model::*;
use qgfx::{RendererController, FontHandle};
use common::Rect;
use cgmath;
use std;
use state;

pub struct PackageListView {
    pub state: std::sync::Arc<state::State>,

    pub font: FontHandle,
}

impl PackageListView {
    pub fn new(state: std::sync::Arc<state::State>, font: FontHandle) -> PackageListView {
        PackageListView {
            state: state,
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

    /// Function to render a package recursively. Returns a rect which contains the space used up.
    fn render_pkg(
        &self,
        g: &RendererController,
        pos: cgmath::Vector2<f32>,
        pkg: &Package,
    ) -> Rect {
        // render this package
        g.rect(&[pos.x, pos.y, 128.0 - 16.0, 32.0], &[0.1, 0.1, 0.1, 1.0]);
        g.text(
            pkg.name.as_ref(),
            &[pos.x, pos.y + 32.0 / 2.0],
            self.font,
            &[1.0, 1.0, 1.0, 1.0],
        ).unwrap();

        let mut indented = cgmath::Vector2 {
            x: pos.x + 16.0,
            y: pos.y + 32.0,
        };
        for p in &pkg.package_list {
            let rect = self.render_pkg(g, indented, p);
            indented.y += rect.size.y;
        }

        let rect = self.render_decl_list(g, &pkg.decl_list[..], indented, 32.0, 128.0 - 16.0);
        return Rect::new(pos.x, pos.y, 128.0, indented.y - pos.y + rect.size.y);
    }

    /// Renders a list of packages.
    /// Returns the bounding box used up from rendering.
    pub fn render(&self, g: &RendererController, offset: cgmath::Vector2<f32>) {
        let mut pos = offset;
        let package_list = &*self.state.project.package_list.lock().unwrap();
        for p in package_list {
            let rect = self.render_pkg(g, pos, p);
            pos.y += rect.size.y;
        }
    }
}
