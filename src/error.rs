use std::{io as stdio, result};

use failure::Fail;
use zydis::ZydisError;

pub(crate) type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub(crate) enum Error {
    #[fail(display = "Zydis error: {}", _0)]
    Zydis(#[cause] ZydisError),

    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] stdio::Error),

    #[fail(display = "Application error: {}", _0)]
    Application(String),
}

macro_rules! application_error {
    ($msg:expr) => {
        Error::Application(String::from($msg))
    };
}
