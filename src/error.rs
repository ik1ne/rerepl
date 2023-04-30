use std::ffi::OsString;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid command")]
    InvalidCommand(OsString),
}
