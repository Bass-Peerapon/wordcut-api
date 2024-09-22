use pyo3::prelude::*;

pub struct SpellChecker;

impl SpellChecker {
    pub fn new() -> Self {
        Self
    }
    pub fn check(&self, text: &str) -> PyResult<Vec<String>> {
        Python::with_gil(|py| {
            let pythainlp = py.import_bound("pythainlp")?;
            let spell = pythainlp.getattr("spell")?;
            let suggestions: Vec<String> = spell.call1((text,))?.extract()?;
            Ok(suggestions)
        })
    }
}

impl Default for SpellChecker {
    fn default() -> Self {
        Self::new()
    }
}
