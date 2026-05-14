use std::{ fs::{ File, OpenOptions }, io::{ BufWriter, Write }, path::Path };

use crate::error::Result;

pub struct FileWriter {
    writer: Option<BufWriter<File>>,
}

impl FileWriter {
    pub fn new(file_path: Option<&str>, preview: bool, backup: bool) -> Result<Self> {
        let writer = if !preview {
            if let Some(path) = file_path {
                // Create backup if requested
                if backup {
                    FileWriter::backup_file(path)?;
                }

                let file = OpenOptions::new().append(true).create(true).open(path)?;

                Some(BufWriter::new(file))
            } else {
                None
            }
        } else {
            None
        };

        Ok(FileWriter { writer })
    }

    pub fn write(&mut self, line: &str, separator: &str) -> Result<()> {
        if let Some(ref mut w) = self.writer {
            write!(w, "{}{}", line, separator)?;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        if let Some(ref mut w) = self.writer {
            w.flush()?;
        }
        Ok(())
    }

    fn backup_file(file_path: &str) -> Result<()> {
        let path = Path::new(file_path);
        if path.exists() {
            let backup_path = format!("{}.bak", file_path);
            std::fs::copy(file_path, &backup_path)?;
        }
        Ok(())
    }
}
