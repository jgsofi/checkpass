use std::collections::HashSet;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::io::BufReader;
use std::fs::File;

pub struct Checker {
    lookup: HashSet<String>,
}

impl Checker {
    pub fn new() -> Checker {
        Checker { lookup: HashSet::new() }
    }

    pub fn contains(&self, password: &str) -> bool {
        self.lookup.contains(password)
    }

    // Gets passwords from all files listed as arguments
    pub fn load_passwords(&mut self, files: Vec<String>) {
        for filename in files {
            match self.injest_password_file(&filename) {
                Ok(_) => println!("Completed file {}... passwords so far: {}", filename, self.lookup.len()),
                Err(error) => println!("Skipping file {}: read canceled because {}", filename, error),
            }
        }
    }

    // Reads passwords from a file into a BTreeSet, halting on and returning any errors
    fn injest_password_file(&mut self, filename: &str) -> Result<(), IOError> {
        let file = File::open(filename)?;
        for line in BufReader::new(file).lines() {
            let line = line?;
            if line.len() >= 8 {
                self.lookup.insert(line);
            }
        }
        Ok(())
    }
}