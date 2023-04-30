use std::collections::HashMap;

use crate::error::Error;

mod child;
mod parent;

const RERUN_ENVVAR: &str = "REREPL_RERUN";

pub type Handler = Box<dyn Fn(&str)>;

pub struct Rerepl {
    prompt: String,
    handlers: HashMap<String, Handler>,
}

impl Rerepl {
    pub fn init(prompt: String) -> Self {
        if Self::is_parent() {
            Self::init_as_parent();
        }

        Self {
            prompt,
            handlers: Default::default(),
        }
    }

    pub fn add_handler(&mut self, cmd: &str, handler: Handler) {
        self.handlers.insert(String::from(cmd), handler);
    }

    pub fn is_parent() -> bool {
        std::env::var(RERUN_ENVVAR).is_ok()
    }

    pub fn run(&self) {
        if Self::is_parent() {
            self.run_as_parent();
        } else {
            self.run_as_child();
        }
    }
}
