
#![allow(unused)]

use itertools::Itertools;

use crate::{
    indexation::WordsMap,
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
    map: WordsMap,
}

pub struct Unzip {
    compressed: String,
    map: WordsMap,
}

pub struct ToSave {
    content: String,
    map: Option<WordsMap>
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
    fn from(original: String) -> Zip {
        Zip {
            original,
            map: WordsMap::new(),
        }
    }

    #[inline]
    pub fn proc(self) -> ToSave {
    
        use crate::indexation::{
            words::Words,
            WordsMap,
        };

        let s = Self::split(&self.original);

        let mut words = Words::new();

        s.iter().map(|w| words.insert(w, false)).collect_vec();
        for i in 2..=5 {
            s.windows(i).map(|w| words.insert(&w.join(" "), true)).collect_vec();
        }
        words.clean();

        let (w, c) = words.into_vecs();
        let m = WordsMap::from(w, c);

        for (ch, word) in m.iter() {
            self.original.replace(word, ch.as_str());
        }

        ToSave {
            content: self.original,
            map: Some(m),
        }
    }

    #[inline]
    fn split(s: &str) -> Vec<String> {

        use ch::Ch;

        let mut vec: Vec<String> = Vec::with_capacity(s.len()/8);
        let mut buf = String::with_capacity(15);
        
        let mut chars = s.chars().peekable();
        let mut prev = Ch::from(*chars.peek().expect("Specify non-empty file!"));


        for c in chars {
            let current = Ch::from(c);
            if current != prev && !buf.is_empty() {
                vec.push(buf.to_owned());
                buf.clear()
            } 
            buf.push(c);
            prev = current
        }
        if !buf.is_empty() {
            vec.push(buf.to_owned())
        }

        vec
    }

}

impl Unzip {

    #[inline]
    fn from(original: String) -> Unzip {
        Unzip {
            compressed: original,
            map: WordsMap::new(),
        }
    }

    #[inline]
    pub fn proc(self) -> ToSave {
        unimplemented!()
    }

}

pub(self) mod ch {

    #[derive(PartialEq, Eq)]
    pub enum Ch {
        WhiteSpace,
        Alpha,
        PuncSym,
    }

    impl Ch {
        #[inline]
        pub fn from(c: char) -> Self {
            if char::is_whitespace(c) {
                Self::WhiteSpace
            } else if char::is_alphabetic(c) {
                Self::Alpha
            } else {
                Self::PuncSym
            }
        }

    }
}

#[test]
fn split() {
    use self::Zip;
    let s = "some, string.. with !some \n symbols \t, need to separate this ";
    let rus = "некая русская строка \n, с \u{2223} ,, ,as to";

    assert_eq!(Zip::split(s), vec!["some", ",", " ", "string", "..", " ", "with", " ", "!", "some", " \n ", "symbols", " \t", ",", " ", "need", " ", "to", " ", "separate", " ", "this", " "]);
    assert_eq!(Zip::split(rus), vec![ "некая", " ", "русская", " ", "строка", " \n", ",", " ","с", " ", "\u{2223}", " ", ",,", " ", ",", "as", " ", "to" ]);
}
