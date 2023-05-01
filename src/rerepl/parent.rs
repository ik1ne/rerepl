use std::env;
use std::io::{stdout, Read, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;

use crate::rerepl::{Rerepl, IS_CHILD_ENVVAR};

impl Rerepl {
    pub(crate) fn run_as_parent(&self) -> Result<(), anyhow::Error> {
        // Relaunch the program as a child

        // ArcMutex buffer of prompt + user input

        // setup stream of stdin(with raw terminal input)
        // - get write lock on buffer
        // -

        // setup stream of child stdout as external stream handler
        // setup stream of child stderr as external stream handler

        // external stream handler: when an stream outputs something,
        // - get read lock on buffer
        // - set cursor back to beginning of buffer
        // - clear terminal from cursor to end of screen
        // - write stream output to terminal
        // - write buffer to terminal


        todo!()
    }
}
