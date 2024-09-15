use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use wordcut_engine::{load_dict, Wordcut};

pub struct WordcutEngineSdk {
    wordcut: Wordcut,
}

pub trait WordcutEngine {
    fn cut_with_delimiters(&self, text: &str, delimiter: &str) -> String;

    fn cut(&self, text: &str) -> Vec<String>;

    // เพิ่มคำใหม่ลงใน dict.txt และโหลด dict ใหม่
    fn add_word(&mut self, word: &str);

    // ลบคำใน dict.txt
    fn remove_word(&mut self, word: &str);

    // ตรวจสอบว่าคำมีใน dict.txt แล้วหรือยัง
    fn word_exists_in_dict(&self, word: &str, dict_path: &Path) -> bool;

    // เพิ่มคำใหม่ลงใน dict.txt
    fn append_word_to_dict(&self, word: &str, dict_path: &Path);

    // ลบคำใน dict.txt
    fn remove_word_from_dict(&self, word: &str, dict_path: &Path);

    // โหลด dict ใหม่และอัปเดต wordcut
    fn reload_dict(&mut self, dict_path: &Path);
}

impl WordcutEngine for WordcutEngineSdk {
    fn cut_with_delimiters(&self, text: &str, delimiter: &str) -> String {
        self.wordcut.put_delimiters(text, delimiter)
    }
    fn cut(&self, text: &str) -> Vec<String> {
        self.wordcut.segment_into_strings(text)
    }

    // เพิ่มคำใหม่ลงใน dict.txt และโหลด dict ใหม่
    fn add_word(&mut self, word: &str) {
        let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));

        // ตรวจสอบว่าคำนี้มีใน dict.txt แล้วหรือยัง
        if !self.word_exists_in_dict(word, dict_path) {
            // ถ้าไม่มีคำนี้ ให้เขียนคำใหม่ลงไปในไฟล์
            self.append_word_to_dict(word, dict_path);
            // โหลด dict ใหม่หลังจากเพิ่มคำ
            self.reload_dict(dict_path);
        }
    }

    // ลบคำใน dict.txt
    fn remove_word(&mut self, word: &str) {
        let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));
        self.remove_word_from_dict(word, dict_path);
        self.reload_dict(dict_path);
    }

    // ตรวจสอบว่าคำมีใน dict.txt แล้วหรือยัง
    fn word_exists_in_dict(&self, word: &str, dict_path: &Path) -> bool {
        let file = File::open(dict_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines().map_while(Result::ok) {
            if line == word {
                return true;
            }
        }
        false
    }

    // เพิ่มคำใหม่ลงใน dict.txt
    fn append_word_to_dict(&self, word: &str, dict_path: &Path) {
        let mut file = OpenOptions::new().append(true).open(dict_path).unwrap();

        // เขียนคำใหม่ลงไปในไฟล์ dict.txt
        writeln!(file, "{}", word).unwrap();
    }

    // ลบคำใน dict.txt
    fn remove_word_from_dict(&self, word: &str, dict_path: &Path) {
        let file = File::open(dict_path).unwrap();
        let reader = BufReader::new(file);

        // อ่านไฟล์ทีละบรรทัดและเก็บคำที่ไม่ตรงกับคำที่จะลบ
        let lines: Vec<String> = reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.trim().eq(word)) // ค้นหาและลบคำ
            .collect();

        // เขียนไฟล์ใหม่โดยไม่มีคำที่ลบ
        let mut file = File::create(dict_path).unwrap();
        for line in lines {
            writeln!(file, "{}", line).unwrap();
        }
    }

    // โหลด dict ใหม่และอัปเดต wordcut
    fn reload_dict(&mut self, dict_path: &Path) {
        let dict = load_dict(dict_path).unwrap();
        self.wordcut = Wordcut::new(dict); // อัปเดต wordcut ด้วย dict ใหม่
    }
}

pub fn new() -> WordcutEngineSdk {
    let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));
    let dict = load_dict(dict_path).unwrap();
    let wordcut = Wordcut::new(dict);
    WordcutEngineSdk { wordcut }
}
