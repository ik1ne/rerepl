use crate::rerepl::IS_CHILD_ENVVAR;
use anyhow::anyhow;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub fn spawn_child() -> Result<(impl Write, impl Read), anyhow::Error> {
    let current_exe = std::env::current_exe().unwrap();
    let mut child = Command::new(current_exe)
        .env(IS_CHILD_ENVVAR, "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let child_stdin = child
        .stdin
        .take()
        .ok_or(anyhow!("failed to take child stdin"))?;
    let child_stdout = child
        .stdout
        .take()
        .ok_or(anyhow!("failed to take child stdout"))?;

    Ok((child_stdin, child_stdout))
}
