
#![allow(unused)]

use itertools::Itertools;

use crate::{
    indexation::Word,
    fs::Files,
    indexation::*,
    Result,
};

use std::{
    fs::File,
    collections::HashMap,
    io::Read,
};

pub enum Do {
    Zip(Zip),       // struct inside
    Unzip(Unzip),   // struct inside
}

pub struct Zip {
    original: String,
    compressed: String,
    map: Map,
}

pub struct Unzip {
    compressed: String,
    original: String,
    map: Map,
}

struct CharSet {
    set: [char; 2]
}

struct Map {
    map: HashMap<CharSet, String>
}

pub struct ToSave {
    content: String,
    map: Option<Map>
}

impl Do {

    #[inline]
    pub fn proc(self) -> ToSave {
        match self {
            Self::Zip(z)    => z.proc(),
            Self::Unzip(u)  => u.proc(),
        }
    }
}

impl Zip {

    #[inline]
    pub fn new(original: String) -> Self {
        unimplemented!()
    }

    #[inline]
    pub fn proc(self) -> ToSave {
        unimplemented!()
    }
}

impl Unzip {

    #[inline]
    pub fn new(original: String) -> Self {
        unimplemented!()
    }

    #[inline]
    pub fn proc(self) -> ToSave {
        unimplemented!()
    }
}




impl From<[char; 2]> for CharSet {
    fn from(set: [char; 2]) -> Self {
        Self { set }
    }
}
