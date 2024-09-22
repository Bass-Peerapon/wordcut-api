use crate::infrastructure::spell_checker::SpellChecker;

pub struct SpellCheckerUsecase {
    spell_checker: SpellChecker,
}

impl SpellCheckerUsecase {
    pub fn new(spell_checker: SpellChecker) -> Self {
        Self { spell_checker }
    }

    pub fn check(&self, text: &str) -> Vec<String> {
        self.spell_checker.check(text).unwrap()
    }
}
