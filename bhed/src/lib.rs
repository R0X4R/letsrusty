use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub silent: bool,
    pub preview: bool,
    pub normalize: bool,
    pub ignore_case: bool,
    pub count_only: bool,
    pub skip_blanks: bool,
    pub separator: String,
    pub output: Option<String>,
}

pub struct Processor {
    known: HashSet<String>,
    writer: Option<BufWriter<File>>,
}

impl Processor {
    pub fn new(config: &Config) -> io::Result<Self> {
        let mut known: HashSet<String> = HashSet::new();

        if let Some(ref file_path) = config.output {
            if Path::new(file_path).exists() {
                let file = File::open(file_path)?;
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let raw = line?;
                    let key = Self::normalize_line(&raw, config.normalize, config.ignore_case);
                    known.insert(key);
                }
            }
        }

        let writer = if !config.preview {
            config.output.as_ref().map(|file_path| {
                let file = OpenOptions::new().append(true).create(true).open(file_path).unwrap();
                BufWriter::new(file)
            })
        } else {
            None
        };

        Ok(Self { known, writer })
    }

    pub fn process_stdin(&mut self, config: &Config) -> io::Result<usize> {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let mut new_count = 0usize;

        for line in reader.lines() {
            let raw = line?;

            if config.skip_blanks && raw.trim().is_empty() {
                continue;
            }

            let key = Self::normalize_line(&raw, config.normalize, config.ignore_case);

            if self.known.contains(&key) {
                continue;
            }

            self.known.insert(key);
            new_count += 1;

            if !config.silent && !config.count_only {
                println!("{}", raw);
            }

            if let Some(ref mut w) = self.writer {
                write!(w, "{}{}", raw, config.separator)?;
            }
        }

        if let Some(ref mut w) = self.writer {
            w.flush()?;
        }

        Ok(new_count)
    }

    fn normalize_line(line: &str, normalize: bool, ignore_case: bool) -> String {
        let mut value = if normalize {
            line.trim().to_string()
        } else {
            line.to_string()
        };

        if ignore_case {
            value = value.to_lowercase();
        }

        value
    }
}