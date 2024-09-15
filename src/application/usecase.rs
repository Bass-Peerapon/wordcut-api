pub trait WordcutUsecase {
    fn cut(&self, text: &str) -> Vec<String>;

    fn add_word(&mut self, word: &str);

    fn remove_word(&mut self, word: &str);
}
