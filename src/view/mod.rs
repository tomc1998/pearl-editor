//! A module which encompasses generating vertex data to render.

mod command_buffer;
mod prompt;

pub use self::command_buffer::*;
pub use self::prompt::*;

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
    /// # Params
    /// * `prefix` - The prefix to this decl, i.e. all the parents. e.g - "com.tom."
    /// * `sel` - The current selection. Will tell us whether or not to highlight.
    fn render_decl_list(
        &self,
        g: &mut RendererController,
        decl_list: &[Declaration],
        offset: cgmath::Vector2<f32>,
        decl_height: f32,
        decl_width: f32,
        prefix: &mut String,
        sel: &Option<state::Selection>,
    ) -> Rect {
        let mut pos = offset;
        for d in decl_list {
            let orig_prefix_len = prefix.len();
            prefix.push_str(d.name().as_ref());
            let col = if sel.is_none() || !sel.as_ref().unwrap().is_decl(prefix.as_str()) {
                [0.1, 0.1, 0.1, 1.0]
            } else {
                [0.2, 0.5, 0.2, 1.0]
            };
            g.rect(&[pos.x, pos.y, decl_width, decl_height], &col);
            g.text(
                prefix.as_str(),
                &[pos.x, pos.y + decl_height / 2.0],
                self.font,
                &[1.0, 1.0, 1.0, 1.0],
            );
            pos.y += decl_height;
            prefix.truncate(orig_prefix_len);
        }
        return Rect::new(
            offset.x,
            offset.y,
            decl_width,
            decl_height * decl_list.len() as f32,
        );

    }

    /// Function to render a package recursively. Returns a rect which contains the space used up.
    ///
    /// # Params
    /// * `prefix` - The prefix to this package, i.e. all the parents. e.g - "com.tom."
    /// * `sel` - The current selection. Will tell us whether or not to highlight.
    fn render_pkg(
        &self,
        g: &mut RendererController,
        pos: cgmath::Vector2<f32>,
        pkg: &Package,
        prefix: &mut String,
        sel: &Option<state::Selection>,
    ) -> Rect {
        let orig_prefix_len = prefix.len();
        prefix.push_str(pkg.name.as_ref());
        let col = if sel.is_none() || !sel.as_ref().unwrap().is_package(prefix.as_str()) {
            [0.1, 0.1, 0.1, 1.0]
        } else {
            [0.2, 0.5, 0.2, 1.0]
        };
        // render this package - choose a highlighted colour if this prefix is the selected one
        g.rect(&[pos.x, pos.y, 128.0 - 16.0, 32.0], &col);
        g.text(
            prefix,
            &[pos.x, pos.y + 32.0 / 2.0],
            self.font,
            &[1.0, 1.0, 1.0, 1.0],
        );


        // Add '.' to our pkg name so we can use it as a prefix
        prefix.push_str(".");
        let mut indented = cgmath::Vector2 {
            x: pos.x + 16.0,
            y: pos.y + 32.0,
        };
        for p in &pkg.package_list {
            let rect = self.render_pkg(g, indented, p, prefix, sel);
            indented.y += rect.size.y;
        }


        let rect = self.render_decl_list(g, &pkg.decl_list[..], indented, 32.0, 128.0 - 16.0, prefix, sel);

        prefix.truncate(orig_prefix_len);
        return Rect::new(pos.x, pos.y, 128.0, indented.y - pos.y + rect.size.y);
    }

    /// Renders a list of packages.
    /// Returns the bounding box used up from rendering.
    pub fn render(&self, g: &mut RendererController, offset: cgmath::Vector2<f32>) {
        let mut pos = offset;
        let package_list = &*self.state.project.package_list.lock().unwrap();
        let selected_pkg_name = &*self.state.project.curr_sel.lock().unwrap();
        for p in package_list {
            let rect = self.render_pkg(g, pos, p, &mut "".to_owned(), selected_pkg_name);
            pos.y += rect.size.y;
        }
    }
}
