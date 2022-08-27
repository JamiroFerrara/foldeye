use std::fs;
use rayon::prelude::*;

#[derive(Debug)]
pub enum Entry {
    File(File),
    Directory(Directory),
}


#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: i32,
}

impl File {
    fn new(name: &str, size: i32) -> File {
        File {
            name: name.to_string(),
            size: size,
        }
    }
}


#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub entries: Vec<Entry>,
}

impl Directory {
    pub fn new(path: &str) -> Result<Directory, std::io::Error> {
        let entries = get_entries(path)?;

        Ok(Directory {
            name: path.to_string(),
            entries: entries,
        })
    }

    pub fn count(&self) -> i32 {
        self.entries
            .par_iter()
            .map(|entry| match entry {
                Entry::File(_) => 1,
                Entry::Directory(dir) => dir.count(),
            })
            .sum()
    }

    pub fn dirs(&self) -> Vec<&String> {
        self.entries
            .par_iter()
            .filter_map(|entry| match entry {
                Entry::File(_) => None,
                Entry::Directory(dir) => Some(&dir.name),
            })
            .collect()
    }

    pub fn files(&self) -> Vec<&String> {
        self.entries
            .par_iter()
            .filter_map(|entry| match entry {
                Entry::File(file) => Some(&file.name),
                Entry::Directory(_) => None,
            })
            .collect()
    }

    pub fn all_files(&self) -> Vec<&String> {
        self.entries
            .par_iter()
            .flat_map(|entry| match entry {
                Entry::File(file) => vec![&file.name],
                Entry::Directory(dir) => dir.all_files(),
            })
            .collect()
    }

    pub fn all_dirs(&self) -> Vec<&String> {
        self.entries
            .par_iter()
            .flat_map(|entry| match entry {
                Entry::File(_) => vec![],
                Entry::Directory(dir) => {
                    let mut dirs = vec![&dir.name];
                    dirs.extend(dir.all_dirs());
                    dirs
                }
            })
            .collect()
    }
}

fn get_entries(path: &str) -> Result<Vec<Entry>, std::io::Error> {
    let read_dir = fs::read_dir(path)?;
    let mut entries = Vec::new();
    for dir in read_dir {
        let entry = dir?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            entries.push(Entry::Directory(Directory::new(&entry.path().to_str().unwrap())?));
        } else {
            entries.push(Entry::File(File::new(&entry.path().to_str().unwrap(), metadata.len() as i32)));
        }
    }

    Ok(entries)
}
