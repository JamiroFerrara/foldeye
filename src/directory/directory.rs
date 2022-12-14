use crate::directory::*;
use rayon::prelude::*;
use std::hash::{Hash, Hasher};
use differ::{Differ, Tag, Span};
use std::fs;

#[derive(Debug, Clone)]
pub struct CompAction {
    pub action: Vec<ComparisonActionEnum>,
}

#[derive(Debug, Clone)]
pub enum ComparisonActionEnum {
    Inserted(Vec<String>),
    Removed(Vec<String>),
    Replaced(Vec<String>)
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

    pub fn all_files(&self) -> Vec<&String> {
        self.entries
            .par_iter()
            .flat_map(|entry| match entry {
                Entry::File(file) => vec![&file.name],
                Entry::Directory(dir) => dir.all_files(),
            })
            .collect()
    }

    pub fn compare(&self, other: &Directory) -> CompAction {
        let self_files = self.all_files();
        let other_files = other.all_files();
        let differ = Differ::new(&self_files, &other_files);

        let ins = other.get_inserted_paths(differ.spans().to_vec());
        let del = self.get_deleted_paths(differ.spans().to_vec());
        let rep = self.get_replaced_paths(differ.spans().to_vec());

        CompAction {
            action: vec![
                ComparisonActionEnum::Inserted(ins),
                ComparisonActionEnum::Removed(del),
                ComparisonActionEnum::Replaced(rep),
            ],
        }
    } 

    pub fn get_deleted_paths(&self, spans: Vec<Span>) -> Vec<String> {
        let mut deleted_paths: Vec<String> = Vec::new();
        let paths = self.all_files();
        
        for span in spans {
            match span.tag {
                Tag::Equal => {}
                Tag::Insert => {},
                Tag::Delete => {
                    let range = &paths[span.a_start..span.a_end];
                    range.iter().for_each(|path| {
                        deleted_paths.push(path.to_owned().to_string());
                    });
                }
                Tag::Replace => {} ,
            }
        }

        deleted_paths
    }

    pub fn get_replaced_paths(&self, spans: Vec<Span>) -> Vec<String> {
        let mut replaced_paths: Vec<String> = Vec::new();
        let paths = self.all_files();
        
        for span in spans {
            match span.tag {
                Tag::Equal => {}
                Tag::Insert => {},
                Tag::Delete => {} ,
                Tag::Replace => {
                    let range = &paths[span.a_start..span.a_end];
                    range.iter().for_each(|path| {
                        replaced_paths.push(path.to_owned().to_string());
                    });
                }
            }
        }

        replaced_paths
    }

    pub fn get_inserted_paths(&self, spans: Vec<Span>) -> Vec<String> {
        let mut added_paths: Vec<String> = Vec::new();
        let paths = self.all_files();
        
        for span in spans {
            match span.tag {
                Tag::Equal => {}
                Tag::Insert => {
                    let range = &paths[span.b_start..span.b_end];
                    range.iter().for_each(|path| {
                        added_paths.push(path.to_owned().to_string());
                    });
                }
                Tag::Delete => {} ,
                Tag::Replace => {} ,
            }
        }

        added_paths
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
            entries.push(Entry::File(File::new(&entry.path().to_str().unwrap().replace("\\", "/"), metadata.len() as i32)));
        }
    }

    Ok(entries)
}

impl<'a> Hash for Directory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Directory) -> bool {
        self.name == other.name // MUST use same data as Hash
    }
}
impl Eq for Directory {}
