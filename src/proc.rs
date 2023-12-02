const SEP: &'static str = "\u{2ffff}";

use itertools::Itertools;
use sha256::digest;

use crate::{
    err::Error,
    fs::ToSave,
    indexation::{words::Words, WordsMap},
    Result,
};

pub enum Do {
    Zip(Zip),     // struct inside
    Unzip(Unzip), // struct inside
}

pub struct Zip {
    original: String,
}

pub struct Unzip {
    compressed: String,
}

impl Do {
    #[inline]
    pub fn proc(self) -> Result<ToSave> {
        match self {
            Self::Zip(z) => z.proc(),
            Self::Unzip(u) => u.proc(),
        }
    }
}

impl Zip {
    #[inline]
    pub fn from(original: String) -> Zip {
        Zip { original }
    }

    #[inline]
    pub fn proc(self) -> Result<ToSave> {
        let mut s = split(&self.original);

        let mut words = Words::new();

        s.windows(3).map(|w| words.insert(&w.join(""), true)).collect_vec();
        s.iter().map(|w| words.insert(w, false)).collect_vec();
        words.clear();

        if words.is_empty() {
            return Err(Box::new(Error::new("file", "Nothing to compress!")));
        }

        let (w, c) = words.into_vecs();
        let m = WordsMap::from(w, c);

        let mut table = String::new();

        for (ch, word) in m.iter() {
            table.push_str(&format!("{}:{}\n", ch, word));
            s.iter_mut()
                .map(|w| {
                    if w == word {
                        *w = ch.as_str().to_owned()
                    }
                })
                .for_each(drop);
        }

        let content = s.join("");
        let hash = digest(format!("{table}{content}"));

        Ok(ToSave::new(format!("{hash}{SEP}{table}{SEP}{content}")))
    }
}

impl Unzip {
    #[inline]
    pub fn from(original: String) -> Unzip {
        Unzip {
            compressed: original,
        }
    }

    #[inline]
    pub fn proc(self) -> Result<ToSave> {
        // return Err(Box::new(Error::new("file", "File is corrupted!")))

        let l: Option<(&str, &str, &str)> = self.compressed.split(SEP).collect_tuple();

        let (plain, content): (String, String);

        if let Some((hash, imap, icontent)) = l {
            plain = imap.to_string();
            content = icontent.to_string();

            if digest(format!("{}{}", &plain, &content)) != hash {
                return Err(Box::new(Error::new("file", "File is corrupted!")));
            }
        } else {
            return Err(Box::new(Error::new("file", "File is corrupted!")));
        }

        let map = WordsMap::from_plain(plain);

        let mut words = split(&content);

        for (ch, word) in map.iter() {
            words
                .iter_mut()
                .map(|w| {
                    if w == ch.as_str() {
                        *w = word.as_str().to_owned()
                    }
                })
                .for_each(drop);
        }

        Ok(ToSave::new(words.join("")))
    }
}

pub(self) mod ch {

    #[derive(PartialEq, Eq)]
    pub enum Ch {
        Alphabetic,
        WhiteSpace,
        Other,
    }

    impl Ch {
        #[inline]
        pub fn from(c: char) -> Self {
            if char::is_alphabetic(c) {
                Self::Alphabetic
            } else if char::is_whitespace(c) {
                Self::WhiteSpace
            } else {
                Self::Other
            }
        }
    }
}

#[inline]
fn split(s: &str) -> Vec<String> {
    use ch::Ch;

    let mut vec: Vec<String> = Vec::with_capacity(s.len() / 8);
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

#[test]
fn split_cases() {
    let s = "some, string.. with !some \n symbols \t, need to separate this ";
    let rus = "некая русская строка \n, с \u{2223} ,, ,as to";

    assert_eq!(
        split(s),
        vec![
            "some", ",", " ", "string", "..", " ", "with", " ", "!", "some", " \n ", "symbols",
            " \t", ",", " ", "need", " ", "to", " ", "separate", " ", "this", " "
        ]
    );
    assert_eq!(
        split(rus),
        vec![
            "некая",
            " ",
            "русская",
            " ",
            "строка",
            " \n",
            ",",
            " ",
            "с",
            " ",
            "\u{2223}",
            " ",
            ",,",
            " ",
            ",",
            "as",
            " ",
            "to"
        ]
    );
}
