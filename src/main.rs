use std::{
    env::Args,
    fmt::Display,
    fs::{self},
    io::{BufReader, Read},
};

#[derive(Debug)]
struct Configuration {
    filepaths: Vec<String>,
    count_characters: bool,
    count_lines: bool,
    count_words: bool,
    count_bytes: bool,
}

fn parse_config<'a>(_args: Args) -> Configuration {
    let mut args = _args.skip(1).peekable();
    if args.len() < 2 {
        panic!("wc-rust usage: <filepaths []> <options>");
    }
    let mut cfg = Configuration {
        filepaths: Vec::new(),
        count_characters: false,
        count_lines: false,
        count_words: false,
        count_bytes: false,
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-m" | "--chars" => cfg.count_characters = true,
            "-l" | "--lines" => cfg.count_lines = true,
            "-c" | "--bytes" => cfg.count_bytes = true,
            "-f" | "--files" => {
                while let Some(value) = args.peek() {
                    if value.starts_with("-") {
                        break;
                    }

                    let value = args.next().unwrap();
                    cfg.filepaths.push(value);
                }
            }

            "-w" | "--words" => cfg.count_words = true,

            _ if !arg.starts_with("-") => {
                cfg.filepaths.push(arg);
            }

            unknown => {
                panic!("Unknow argument, {unknown}")
            }
        }
    }

    return cfg;
}

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
    let config = parse_config(args);

    let mut filepaths = config.filepaths.iter();
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
        if config.count_lines {
            stats.lines = contents.lines().count().into();
        }
        if config.count_characters {
            stats.chars = contents.chars().count().into();
        }

        if config.count_words {
            stats.words = contents.split_whitespace().count().into();
        }

        if config.count_bytes {
            stats.bytes = contents.bytes().count().into();
        }

        println!("{} {}", stats, filepath);
    }
}
