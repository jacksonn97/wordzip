
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

pub fn print(mut f: Files) -> Result<()> {
    let mut s = String::new();
    f.r#if.read_to_string(&mut s)?;
    for w in s.split(' ') {
        if !w.is_empty() {
            println!("{w}")
        }
    }
    Ok(())
}

pub fn proc(mut f: Files) -> Result<()> {
    let mut s = String::new();
    f.r#if.read_to_string(&mut s)?;
    let mut words = Words::new();
    let _ = s.split_whitespace().map(|w| words.insert(w)).collect_vec();
    words.clean();

    println!("Words to cut: {:?}", words.total());
    // for w in words {
    //     println!("{:?}", w);
    // }
    Ok(())
}

struct Zip {
    of: File,
    dict: HashMap<String, char>,
    words: Vec<Word>,
    chars: Vec<char>,
}

impl Zip {

    #[inline]
    pub fn new(of: File, words: Vec<String>, chars: Vec<char>) -> Self {
        todo!()
    }

    #[inline]
    fn create_map(&mut self) {
        if self.words.len() > self.chars.len() {
            
            
            

        } else {
            self.words.iter().zip(self.chars.iter())
                .map(|(w, ch)| self.dict.insert(w.to_owned().into_str(), ch.to_owned()))
                .collect_vec();
        }
    }
}
