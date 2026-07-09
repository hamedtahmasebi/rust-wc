use std::{
    env::Args,
    fs,
    io::{self, BufReader, Read, Write},
};

#[derive(Debug)]
struct Configuration {
    filepaths: Vec<String>,
    count_characters: bool,
    count_lines: bool,
    count_words: bool,
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
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-m" | "--chars" => cfg.count_characters = true,
            "-l" | "--lines" => cfg.count_lines = true,

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

        let mut out_lines: Vec<String> = Vec::new();
        if config.count_lines {
            out_lines.push(format!(
                "Lines: {} \n",
                contents.lines().count().to_string()
            ));
        }
        if config.count_characters {
            out_lines.push(format!(
                "Characters: {} \n",
                contents.chars().count().to_string()
            ));
        }

        if config.count_words {
            out_lines.push(format!(
                "Words: {}",
                contents.split_whitespace().count().to_string()
            ));
        }

        for line in out_lines {
            let formatted_string = format!("{}: {}", filepath, line);
            match io::stdout().write(formatted_string.as_bytes()) {
                Ok(_) => (),
                Err(error) => panic!("Error writing to stdout: {error:?}"),
            }
        }
    }
}
