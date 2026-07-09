mod config;

use std::{
    fmt::Display,
    fs::{self},
    io::{BufReader, Read},
};

use crate::config::parse_config;

struct FileStats {
    lines: Option<usize>,
    words: Option<usize>,
    chars: Option<usize>,
    bytes: Option<usize>,
}

impl Display for FileStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(lines) = self.lines {
            write!(f, "{} ", lines)?;
        }

        if let Some(words) = self.words {
            write!(f, "{} ", words)?;
        }

        if let Some(chars) = self.chars {
            write!(f, "{} ", chars)?;
        }
        if let Some(bytes) = self.bytes {
            write!(f, "{} ", bytes)?;
        }
        Ok(())
    }
}

fn main() {
    let args = std::env::args();
    let cfg = parse_config(args);

    let mut filepaths = cfg.filepaths.iter();
    while let Some(filepath) = filepaths.next() {
        let file = match fs::File::open(filepath) {
            Ok(file) => file,
            Err(error) => panic!("Could not open file: {filepath}, error: {error:?}"),
        };
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        match buf_reader.read_to_string(&mut contents) {
            // using read_to_string because it automatically converts bytes to utf8
            Ok(read_count) => read_count,
            Err(error) => panic!("Error reading bytes: {error:?}"),
        };

        let mut stats = FileStats {
            bytes: None,
            chars: None,
            lines: None,
            words: None,
        };
        if cfg.count_lines {
            stats.lines = contents.lines().count().into();
        }
        if cfg.count_characters {
            stats.chars = contents.chars().count().into();
        }

        if cfg.count_words {
            stats.words = contents.split_whitespace().count().into();
        }

        if cfg.count_bytes {
            stats.bytes = contents.bytes().count().into();
        }

        println!("{} {}", stats, filepath);
    }
}
