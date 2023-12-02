pub struct WoCh {
    pub words: Vec<String>,
    pub chars: Vec<char>,
}

impl WoCh {
    #[inline]
    pub fn len(&self) -> (usize, usize) {
        (self.words.len(), self.chars.len())
    }
}
