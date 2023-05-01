use std::collections::HashMap;

mod child;
mod parent;

const IS_CHILD_ENVVAR: &str = "REREPL_IS_CHILD";

pub type Handler = Box<dyn Fn(&str)>;

pub struct Rerepl {
    prompt: String,
    handlers: HashMap<String, Handler>,
}

impl Rerepl {
    pub fn init(prompt: String) -> Self {
        let rerepl = Self {
            prompt,
            handlers: Default::default(),
        };

        rerepl
    }

    pub fn add_handler(&mut self, cmd: &str, handler: Handler) {
        self.handlers.insert(String::from(cmd), handler);
    }

    pub fn is_parent() -> bool {
        std::env::var(IS_CHILD_ENVVAR).is_err()
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        if Self::is_parent() {
            self.run_as_parent()
        } else {
            self.run_as_child()
        }
    }
}
