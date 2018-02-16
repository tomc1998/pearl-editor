//! Module pertaining to application state. All views will keep a reference to this state.

use java_model::*;
use std::sync::{Arc, Mutex};

pub struct Project {
    pub package_list: Arc<Mutex<Vec<Package>>>
}

pub struct State {
    pub project: Project,
}

impl Project {
    pub fn new() -> Project {
        Project {
            package_list: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl State {
    pub fn new() -> State {
        State {
            project: Project::new(),
        }
    }
}
