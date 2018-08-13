use std::{result, io as stdio};

use capstone::Error as CsError;
use failure::Fail;

pub(crate) type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub(crate) enum Error {
    #[fail(display = "Capstone error: {}", _0)]
    Capstone(#[cause] CsError),

    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] stdio::Error),
}