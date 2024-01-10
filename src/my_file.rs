use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Read, Write};
pub struct MyFile_Data {
    path: String,
    data: String,
    mbsize: i32,
    kbsize: i32,
    bsize: i32,
}
pub struct MyFile {
    file_data: MyFile_Data,
}

impl MyFile {
    pub fn new(path: &str) -> MyFile {
        return MyFile {
            file_data: MyFile_Data {
                path: path.to_string(),
                data: String::new(),
                mbsize: 0,
                kbsize: 0,
                bsize: 0,
            },
        };
    }
    pub fn file_write_all(&mut self, buf: String) -> bool {
        let mut file = File::create(&self.file_data.path).unwrap();
        file.write_all(&buf.as_bytes()).unwrap();
        file.flush();
        true
    }
    pub fn file_write_append(&mut self, buf: String) -> bool {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file_data.path)
            .unwrap();
        file.write_all(&buf.as_bytes());
        true
    }
    pub fn create_files(&mut self) -> &MyFile {
        let mut file = File::create(&self.file_data.path);
        self
    }
    pub fn file_read_gets(&self, target_line: usize, buf: &mut String) -> io::Result<()> {
        let mut file = File::open(&self.file_data.path)?;
        let mut reader = io::BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = line?;

            if index == target_line {
                *buf = line;
                break;
            }
        }

        Ok(())
    }
    pub fn file_read_all_in_vec(&self) -> io::Result<Vec<String>> {
        let file = File::open(&self.file_data.path)?;
        let reader = BufReader::new(file);

        let mut lines = Vec::new();

        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(lines)
    }
}
