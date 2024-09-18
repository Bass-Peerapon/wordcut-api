use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Result, Write};
use std::path::Path;
use wordcut_engine::{load_cluster_rules, load_dict, Wordcut};

pub struct WordcutEngine {
    wordcut: Wordcut,
}

impl WordcutEngine {
    pub fn new() -> Result<Self> {
        let cluster_path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/thai_cluster_rules.txt"
        ));
        let cluster_re = match load_cluster_rules(cluster_path) {
            Ok(cluster_re) => cluster_re,
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Failed to load cluster rules: {}", e),
                ));
            }
        };
        let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));
        let dict = load_dict(dict_path)?;
        let wordcut = Wordcut::new_with_cluster_re(dict, cluster_re);

        Ok(Self { wordcut })
    }

    pub fn cut_with_delimiters(&self, text: &str, delimiter: &str) -> String {
        self.wordcut.put_delimiters(text, delimiter)
    }

    pub fn cut(&self, text: &str) -> Vec<String> {
        self.wordcut.segment_into_strings(text)
    }

    // เพิ่มคำใหม่ลงใน dict.txt และโหลด dict ใหม่
    pub fn add_word(&mut self, word: &str) -> Result<()> {
        let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));

        // ตรวจสอบว่าคำนี้มีใน dict.txt แล้วหรือยัง
        if !word_exists_in_dict(word, dict_path)? {
            // ถ้าไม่มีคำนี้ ให้เขียนคำใหม่ลงไปในไฟล์
            append_word_to_dict(word, dict_path)?;
            // โหลด dict ใหม่หลังจากเพิ่มคำ
            self.reload_dict(dict_path)?;
        }
        Ok(())
    }

    // ลบคำใน dict.txt
    pub fn remove_word(&mut self, word: &str) -> Result<()> {
        let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));
        remove_word_from_dict(word, dict_path)?;
        self.reload_dict(dict_path)?;
        Ok(())
    }

    // โหลด dict ใหม่และอัปเดต wordcut
    pub fn reload_dict(&mut self, dict_path: &Path) -> Result<()> {
        let dict = load_dict(dict_path)?;
        self.wordcut = Wordcut::new(dict); // อัปเดต wordcut ด้วย dict ใหม่
        Ok(())
    }
}

impl Default for WordcutEngine {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

// ตรวจสอบว่าคำมีใน dict.txt แล้วหรือยัง
fn word_exists_in_dict(word: &str, dict_path: &Path) -> Result<bool> {
    let file = File::open(dict_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line?.trim() == word {
            return Ok(true);
        }
    }
    Ok(false)
}

// เพิ่มคำใหม่ลงใน dict.txt
fn append_word_to_dict(word: &str, dict_path: &Path) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(dict_path)?;

    // เขียนคำใหม่ลงไปในไฟล์ dict.txt
    writeln!(file, "{}", word)?;
    Ok(())
}

// ลบคำใน dict.txt
fn remove_word_from_dict(word: &str, dict_path: &Path) -> Result<()> {
    let file = File::open(dict_path)?;
    let reader = BufReader::new(file);

    // อ่านไฟล์ทีละบรรทัดและเก็บคำที่ไม่ตรงกับคำที่จะลบ
    let lines: Vec<String> = reader
        .lines()
        .map_while(|line| line.ok())
        .filter(|line| line.trim() != word)
        .collect();

    // เขียนไฟล์ใหม่โดยไม่มีคำที่ลบ
    let mut file = File::create(dict_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
