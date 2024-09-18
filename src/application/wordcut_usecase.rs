use crate::infrastructure::wordcut_engine::WordcutEngine;
use std::io::Result;
pub struct WordcutUsecase {
    wordcut_engine: WordcutEngine,
}

impl WordcutUsecase {
    pub fn new(wordcut_engine: WordcutEngine) -> Self {
        Self { wordcut_engine }
    }

    pub fn cut(&self, text: &str) -> Vec<String> {
        self.wordcut_engine.cut(text)
    }

    pub fn add_word(&mut self, word: &str) -> Result<()> {
        self.wordcut_engine.add_word(word)
    }

    pub fn remove_word(&mut self, word: &str) -> Result<()> {
        self.wordcut_engine.remove_word(word)
    }
}
