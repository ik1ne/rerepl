use anyhow::anyhow;
use std::io::prelude::*;
use std::io::{BufReader, Read, Stdout, Write};
use std::sync::{Arc, Mutex};
use std::{io, thread};

use crate::rerepl::Rerepl;

mod process;
mod terminal;

#[derive(Clone)]
pub(crate) struct SharedData {
    pub(crate) parent_stdout: Arc<Mutex<Stdout>>,
    pub(crate) parent_stdin_buffer: Arc<Mutex<String>>,
    pub(crate) prompt: String,
    pub(crate) cursor_idx: usize,
}

impl Rerepl {
    pub(crate) fn run_as_parent(&self) -> Result<(), anyhow::Error> {
        let stdout_lock = Arc::new(Mutex::new(io::stdout()));
        let input_buffer = Arc::new(Mutex::new(String::new()));

        let mut shared_data = SharedData {
            parent_stdout: stdout_lock,
            parent_stdin_buffer: input_buffer,
            prompt: self.prompt.clone(),
            cursor_idx: 0,
        };

        // Relaunch the program as a child
        let (mut child_stdin, child_stdout) = process::spawn_child()?;

        let mut shared_data_clone = shared_data.clone();
        let child_stdout_thread = thread::spawn(move || {
            let reader = BufReader::new(child_stdout);
            for line in reader.lines().flatten() {
                terminal::handle_child_stdout(&mut shared_data_clone, line);
            }
        });

        terminal::start_handling_terminal_input(&shared_data)?;
        loop {
            if !terminal::handle_parent_stdin(&mut shared_data, &mut child_stdin) {
                break;
            }
        }

        terminal::stop_handling_terminal_input()?;

        child_stdout_thread
            .join()
            .map_err(|e| anyhow!("child thread panicked: {:?}", e))
    }
}
