use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use wordcut_engine::{load_cluster_rules, load_dict, Wordcut};

pub struct WordcutEngine {
    wordcut: Wordcut,
}
impl WordcutEngine {
    pub fn new() -> Self {
        let cluster_path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/thai_cluster_rules.txt"
        ));
        let cluster_re = load_cluster_rules(cluster_path).unwrap();
        let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));
        let dict = load_dict(dict_path).unwrap();
        let wordcut = Wordcut::new_with_cluster_re(dict, cluster_re);

        Self { wordcut }
    }

    pub fn cut_with_delimiters(&self, text: &str, delimiter: &str) -> String {
        self.wordcut.put_delimiters(text, delimiter)
    }
    pub fn cut(&self, text: &str) -> Vec<String> {
        self.wordcut.segment_into_strings(text)
    }

    // เพิ่มคำใหม่ลงใน dict.txt และโหลด dict ใหม่
    pub fn add_word(&mut self, word: &str) {
        let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));

        // ตรวจสอบว่าคำนี้มีใน dict.txt แล้วหรือยัง
        if !word_exists_in_dict(word, dict_path) {
            // ถ้าไม่มีคำนี้ ให้เขียนคำใหม่ลงไปในไฟล์
            append_word_to_dict(word, dict_path);
            // โหลด dict ใหม่หลังจากเพิ่มคำ
            self.reload_dict(dict_path);
        }
    }

    // ลบคำใน dict.txt
    pub fn remove_word(&mut self, word: &str) {
        let dict_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dict.txt"));
        remove_word_from_dict(word, dict_path);
        self.reload_dict(dict_path);
    }

    // โหลด dict ใหม่และอัปเดต wordcut
    pub fn reload_dict(&mut self, dict_path: &Path) {
        let dict = load_dict(dict_path).unwrap();
        self.wordcut = Wordcut::new(dict); // อัปเดต wordcut ด้วย dict ใหม่
    }
}

impl Default for WordcutEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ตรวจสอบว่าคำมีใน dict.txt แล้วหรือยัง
fn word_exists_in_dict(word: &str, dict_path: &Path) -> bool {
    let file = File::open(dict_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        if line.eq(word) {
            return true;
        }
    }
    false
}

// เพิ่มคำใหม่ลงใน dict.txt
fn append_word_to_dict(word: &str, dict_path: &Path) {
    let mut file = OpenOptions::new().append(true).open(dict_path).unwrap();

    // เขียนคำใหม่ลงไปในไฟล์ dict.txt
    writeln!(file, "{}", word).unwrap();
}

// ลบคำใน dict.txt
fn remove_word_from_dict(word: &str, dict_path: &Path) {
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
