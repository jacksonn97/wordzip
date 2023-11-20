
use crate::{
    args::*,
    Result,
};
use std::fs::File;

pub struct Files {
    mode: Mode,
    pub(crate) r#if: File,
    pub(crate) r#of: File,
}

impl Files {

    #[inline]
    pub fn open(a: Args) -> Result<Self> {
        Ok(
        Files {
            mode: *a.mode(),
            r#if: File::open(a.r#if())?,
            r#of: File::create(a.r#of())?,
        })
    }

    #[inline]
    pub fn save(&mut self, of: String) {
        unimplemented!()
    }

}
