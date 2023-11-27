
use std::collections::{ HashMap, VecDeque };
use itertools::Itertools;
use self::words::Word;

#[derive(Debug)]
pub struct WordsMap {
    map: HashMap<CharSet, String>
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct CharSet {
    set: String
}

impl WordsMap {

    #[inline]
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    #[inline]
    pub fn from(w: Vec<Word>, c: Vec<char>) -> Self {
        let mut map: HashMap<CharSet, String> = HashMap::with_capacity(w.len());

        if w.len() > c.len() {
            w.into_iter().zip(c.into_iter())
                .map(|(w, c)| map.insert(c.into(), w.into_str())).collect_vec();
            Self {
                map
            }
        } else {
            let (wl, cl) = (w.len(), c.len());
            let mut c = Self::product(c, Self::amount_digraphs(wl, cl));

            for w in w {
                if w.word_type().is_verylong() {
                    map.insert(c.pop_front().expect("Fail on pop_front!"), w.into_str());
                } else {
                    map.insert(c.pop_back().expect("Fail on pop_back!"), w.into_str());
                }
            }

            Self {
                map
            }
        }
    }

    #[inline]
    pub fn create_from(&mut self, w: Vec<Word>, c: Vec<char>) {

        if w.len() > c.len() {
            w.into_iter().zip(c.into_iter())
                .map(|(w, c)| self.map.insert(c.into(), w.into_str())).collect_vec();
        } else {
            let (wl, cl) = (w.len(), c.len());
            let mut c = Self::product(c, Self::amount_digraphs(wl, cl));

            for w in w {
                if w.word_type().is_verylong() {
                    self.map.insert(c.pop_front().expect("Fail on pop_front!"), w.into_str());
                } else {
                    self.map.insert(c.pop_back().expect("Fail on pop_back!"), w.into_str());
                }
            }

        }
    }

    #[inline]
    fn amount_digraphs(words: usize, mut singles: usize) -> usize {
        let mut needed = 0usize;

        while words > singles {
            singles += singles - 1;
            needed += 1;
        }
        needed
    }

    #[inline]
    pub fn product(mut c: Vec<char>, amount: usize) -> VecDeque<CharSet> {

        let basic = {
            let mut bas = 0;
            while amount > bas {
                bas += 1;
                bas = bas * c.len();
            }
            bas
        };
        let def = c.split_off(basic);

        let mut r = def.iter()
            .cartesian_product(&c)
            .take(amount)
            .map(|c| c.into())
            .collect::<Vec<CharSet>>();

        r.append(&mut def.iter().map(|c| c.into()).collect::<Vec<CharSet>>());
        r.into()

    }

}

impl From<String> for CharSet {
    fn from(set: String) -> Self {
        assert!(set.len() > 2);
        Self { set }
    }
}

impl From<char> for CharSet {
    fn from(set: char) -> Self {
        Self { set: set.to_string() }
    }
}

impl From<&char> for CharSet {
    fn from(set: &char) -> Self {
        Self { set: set.to_string() }
    }
}

impl From<(&char, &char)> for CharSet {
    fn from(set: (&char, &char)) -> Self {
        Self { set: format!("{}{}", set.0, set.1) }
    }
}

pub mod words {

    use itertools::Itertools;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Words {
        words: Vec<Word>,
        unused: Vec<char>,

        n: usize,           // for Iterator implementation
    }

    impl Words {

        #[inline]
        pub fn new() -> Self {
            Words {
                words: Vec::new(),
                n: 0,
                unused: ('\u{00ff}'..='\u{0fff}').collect()
            }
        }
        
        #[inline]
        pub fn insert(&mut self, k: &str) {

            if let Some(w) = self.words.iter_mut().find(|w| *w.str() == k) {
                w.add()
            } else if Self::word_check(k) {
                if k.chars().any(|c| self.unused.contains(&c)) {
                    todo!("try hard")
                }
                self.words.push(Word::new(k.to_string()));
            }
        }

        #[inline]
        pub fn word_check(s: &str) -> bool {
            s.len() >= 3
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
        pub fn total_unused(&self) -> usize {
            self.unused.len()
        }

        #[inline]
        pub fn unused(&self) -> Vec<char> {
            self.unused.to_owned()
        }

        #[inline]
        pub fn into_vecs(self) -> (Vec<Word>, Vec<char>) {
            (self.words, self.unused)
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



    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Word {
        s: String,
        count: usize,
        word_type: WordType,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
    pub enum WordType {
        VeryLong(usize),
        Long(usize),
        Short(usize),
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

#[test]
fn digraphs() {

    use itertools::Itertools;
    use crate::tests::WoCh;

    let chars: Vec<char> = ('a'..='z').collect();
    let words = vec![ "asda", "asdf", "fjjfj", "adfjkj", "sdffjjf", "asdfasdf", "asdf", "asdf", "fdf", "dafasd", "dfd", "dfda", "asd" ]
        .into_iter()
        .map(|w| w.to_string())
        .collect_vec();


    // 1 iteration
    let zero = WoCh { words: words.clone(), chars: chars.clone() };
    let one = WoCh { words: words.clone(), chars: chars[..=6].to_vec() };
    let two = WoCh { words: words.clone(), chars: chars[..=5].to_vec() };

    dbg!(&zero.len());
    dbg!(&one.len());
    dbg!(&two.len());

    // right value - right side
    assert_eq!(WordsMap::amount_digraphs(zero.words.len(), zero.chars.len()), 0);
    assert_eq!(WordsMap::amount_digraphs(one.words.len(), one.chars.len()), 1);
    assert_eq!(WordsMap::amount_digraphs(two.words.len(), two.chars.len()), 2);

}
