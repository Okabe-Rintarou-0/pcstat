
use std::{error::Error, fmt};
use std::fmt::Formatter;

#[derive(Debug)]
pub struct MapError {}

#[derive(Debug)]
pub struct MincoreError {}

impl MapError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl MincoreError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Error for MapError {}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("mmap error")
    }
}

impl Error for MincoreError {}

impl fmt::Display for MincoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("mincore error")
    }
}
