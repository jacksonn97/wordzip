
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub struct Words {
    words: Vec<Word>,
    n: usize,
    unused: Vec<char>,
}

impl Words {

    #[inline]
    pub fn new() -> Self {
        Words {
            words: Vec::new(),
            n: 0,
            unused: (0..u8::MAX).map(|u| u as char).collect()
        }
    }
    
    #[inline]
    pub fn insert(&mut self, k: &str) {

        if let Some(w) = self.words.iter_mut().find(|w| *w.str() == k) {
            w.add()
        } else if Self::word_check(k) {
            let first_char = k.chars().next().unwrap_or(b'\0' as char);
            if self.unused.contains(&first_char) && first_char != b'\0' as char {
                self.unused[first_char as u8 as usize] = b'\0' as char;
            }
            self.words.push(Word::new(k.to_string()));
        }
    }
    
    #[inline]
    pub fn sort(&mut self) {
        self.words.sort_by_key(|w| w.len() * w.amount());
        self.words.reverse();

    }

    #[inline]
    pub fn clean(&mut self) {
        self.sort();

        self.words = self.words.to_owned().into_iter()
            .filter(|w| 
                w.word_type().is_short() && w.amount() >= 15 ||
                w.word_type().is_long() && w.amount() >= 10 ||
                w.word_type().is_verylong() && w.amount() >= 5
            ).collect_vec();

        self.unused = self.unused.to_owned()
            .into_iter()
            .filter(|c| *c != b'\0' as char)
            .collect_vec()[32..]
            .to_vec();
    }

    #[inline]
    pub fn total(&self) -> usize {
        self.words.len()
    }

    #[inline]
    pub fn unused_chars(&self) -> Vec<char> {
        self.unused.to_owned()
    }

    #[inline]
    pub fn word_check(s: &str) -> bool {
        s.len() >= 3
    }

    #[inline]
    pub fn into_vecs(self) -> (Vec<String>, Vec<char>) {
        (self.words.into_iter().map(|w| w.into_str()).collect(), self.unused)
    }
}

impl Iterator for Words {

    type Item = Word;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.words.len() {
            self.n += 1;
            Some(self.words[self.n-1].to_owned())
        } else {
            None
        }
    }


}

// Incapsulation tu-tu-tu
pub use word::Word;
pub(self) mod word {

    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Word {
        s: String,
        count: usize,
        word_type: WordType,
    }

    impl Word {
        
        #[inline]
        pub fn str(&self) -> &String {
            &self.s
        }

        #[inline]
        pub fn into_str(self) -> String {
            self.s
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.word_type.unwrap()
        }

        #[inline]
        pub fn amount(&self) -> usize {
            self.count
        }

        #[inline]
        pub fn add(&mut self) {
            self.count += 1
        }

        #[inline]
        pub fn new(s: String) -> Self {
            let len = s.len();

            Word {
                s,
                count: 1,
                word_type: WordType::from(len)
            }
        }

        #[inline]
        pub fn word_type(&self) -> WordType {
            self.word_type
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
    pub enum WordType {
        VeryLong(usize),
        Long(usize),
        Short(usize),
    }

    impl WordType {

        #[inline]
        pub fn from(l: usize) -> Self {
            if l > 15 {
                Self::VeryLong(l)
            } else if l > 6 {
                Self::Long(l)
            } else {
                Self::Short(l)
            }
        }

        #[inline]
        pub fn unwrap(&self) -> usize {
            match self {
                Self::VeryLong(u)   => *u,
                Self::Long(u)       => *u,
                Self::Short(u)      => *u,
            }
        }

        #[inline]
        pub fn is_verylong(&self) -> bool {
            if let WordType::VeryLong(_) = self {
                true
            } else {
                false
            }
        }

        #[inline]
        pub fn is_long(&self) -> bool {
            if let WordType::Long(_) = self {
                true
            } else {
                false
            }
        }

        #[inline]
        pub fn is_short(&self) -> bool {
            if let WordType::Short(_) = self {
                true
            } else {
                false
            }
        }
    }
}

