use crate::Result;
use std::{fmt::Display, fs::File, io::Write, path::PathBuf as Path};

#[derive(Debug)]
pub struct ToSave {
    content: String,
}

impl ToSave {
    #[inline]
    pub fn new(content: String) -> Self {
        Self { content }
    }

    #[inline]
    pub fn save(self, path: Path) -> Result<()> {
        let mut f = File::create(path)?;
        f.write_all(self.content.as_bytes())?;
        Ok(())
    }
}

impl Display for ToSave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
