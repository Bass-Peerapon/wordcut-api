use crate::application::usecase::WordcutUsecase;
use crate::infrastructure::wordcut_engine::WordcutEngine;
pub struct WordcutApplication<T: WordcutEngine> {
    wordcut_engine: T,
}

impl<T: WordcutEngine> WordcutUsecase for WordcutApplication<T> {
    fn cut(&self, text: &str) -> Vec<String> {
        self.wordcut_engine.cut(text)
    }

    fn add_word(&mut self, word: &str) {
        self.wordcut_engine.add_word(word)
    }

    fn remove_word(&mut self, word: &str) {
        self.wordcut_engine.remove_word(word)
    }
}

pub fn new<T: WordcutEngine>(wordcut_engine: T) -> WordcutApplication<T> {
    WordcutApplication { wordcut_engine }
}
