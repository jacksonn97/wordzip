
#![allow(dead_code)]

pub mod proc;
mod indexation;
pub mod fs;
pub mod args;

#[cfg(test)]
mod tests;

pub(crate) type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;

