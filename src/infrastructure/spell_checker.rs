use std::path::Path;

use spell_checker::load_custom_dict;

pub struct SpellChecker {
    novig_spell_checker: spell_checker::NorvigSpellChecker,
}

impl SpellChecker {
    pub fn new() -> Self {
        let custom_dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tnc_freq.txt"));
        let custom_dict = load_custom_dict(custom_dict_path);
        Self {
            novig_spell_checker: spell_checker::NorvigSpellChecker::new_with_custom_dict(
                custom_dict,
            ),
        }
    }
    pub fn spell(&self, text: &str) -> Vec<String> {
        self.novig_spell_checker.spell(text)
    }
}

impl Default for SpellChecker {
    fn default() -> Self {
        Self::new()
    }
}
