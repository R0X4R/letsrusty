use crate::{ Config, should_skip_line, normalize_line };
use ahash::AHashSet;
use std::{ fs::File, io::{ BufRead, BufReader }, path::Path };

use crate::error::Result;

pub struct FileLoader {
    config: Config,
}

impl FileLoader {
    pub fn new(config: Config) -> Self {
        FileLoader { config }
    }

    pub fn load_existing_lines(&self, file_path: &str) -> Result<AHashSet<String>> {
        let mut known = AHashSet::new();

        if Path::new(file_path).exists() {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let raw = line?;

                if should_skip_line(&raw, self.config.skip_blanks) {
                    continue;
                }

                let key = normalize_line(&raw, self.config.normalize, self.config.ignore_case);
                known.insert(key);
            }
        }

        Ok(known)
    }
}
