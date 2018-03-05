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

/// The height of an item in the package list
const ITEM_HEIGHT: f32 = 16.0;
/// The width of rht package list
const ITEM_WIDTH: f32 = 200.0;

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
        text_indent: f32,
        prefix: &mut String,
        sel: &Option<state::Selection>,
    ) -> Rect {
        let mut pos = offset;
        for d in decl_list {
            let orig_prefix_len = prefix.len();
            prefix.push_str(d.name().as_ref());
            if sel.is_some() && sel.as_ref().unwrap().is_decl(prefix.as_str()) {
                g.rect(
                    &[pos.x, pos.y, ITEM_WIDTH, ITEM_HEIGHT],
                    &[0.2, 0.5, 0.2, 1.0],
                );
            }
            g.text(
                d.name().as_ref(),
                &[pos.x + text_indent + 4.0, pos.y + ITEM_HEIGHT / 2.0 + 4.0],
                self.font,
                &[1.0, 1.0, 1.0, 1.0],
            );
            pos.y += ITEM_HEIGHT;
            prefix.truncate(orig_prefix_len);
        }
        return Rect::new(
            offset.x,
            offset.y,
            ITEM_WIDTH,
            ITEM_HEIGHT * decl_list.len() as f32,
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
        text_indent: f32,
        pkg: &Package,
        prefix: &mut String,
        sel: &Option<state::Selection>,
    ) -> Rect {
        let orig_prefix_len = prefix.len();
        prefix.push_str(pkg.name.as_ref());
        if sel.is_some() && sel.as_ref().unwrap().is_package(prefix.as_str()) {
            g.rect(
                &[pos.x, pos.y, ITEM_WIDTH, ITEM_HEIGHT],
                &[0.2, 0.5, 0.2, 1.0],
            );
        }
        // render this package - choose a highlighted colour if this prefix is the selected one
        g.text(
            prefix,
            &[pos.x + text_indent + 4.0, pos.y + ITEM_HEIGHT / 2.0 + 4.0],
            self.font,
            &[1.0, 1.0, 1.0, 1.0],
        );


        // Add '.' to our pkg name so we can use it as a prefix
        prefix.push_str(".");
        let mut indented = cgmath::Vector2 {
            x: pos.x,
            y: pos.y + ITEM_HEIGHT,
        };
        for p in &pkg.package_list {
            let rect = self.render_pkg(g, indented, text_indent + ITEM_HEIGHT, p, prefix, sel);
            indented.y += rect.size.y;
        }


        let rect = self.render_decl_list(
            g,
            &pkg.decl_list[..],
            indented,
            text_indent + ITEM_HEIGHT,
            prefix,
            sel,
        );

        prefix.truncate(orig_prefix_len);
        return Rect::new(pos.x, pos.y, ITEM_WIDTH, indented.y - pos.y + rect.size.y);
    }

    /// Renders a list of packages.
    pub fn render(&self, g: &mut RendererController, screen_size: cgmath::Vector2<f32>) {
        let mut pos = cgmath::Vector2 { x: 0.0, y: 0.0 };
        let package_list = &*self.state.project.package_list.lock().unwrap();
        let selected_pkg_name = &*self.state.project.curr_sel.lock().unwrap();

        // Render background
        g.rect(
            &[pos.x, pos.y, ITEM_WIDTH, screen_size.y],
            &[0.1, 0.1, 0.1, 1.0],
        );

        // Render items
        for p in package_list {
            let rect = self.render_pkg(g, pos, 0.0, p, &mut "".to_owned(), selected_pkg_name);
            pos.y += rect.size.y;
        }
    }
}
