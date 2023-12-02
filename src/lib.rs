mod fs;
mod indexation;
pub mod args;
pub mod proc;
pub(crate) mod err;

#[cfg(test)]
mod tests;

pub(crate) type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;
