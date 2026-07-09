use std::env::Args;

pub struct Configuration {
    pub filepaths: Vec<String>,
    pub count_characters: bool,
    pub count_lines: bool,
    pub count_words: bool,
    pub count_bytes: bool,
}

pub fn parse_config<'a>(_args: Args) -> Configuration {
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
