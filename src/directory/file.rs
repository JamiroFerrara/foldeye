use crate::directory::*;

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
    pub fn new(name: &str, size: i32) -> File {
        File {
            name: name.to_string(),
            size: size,
        }
    }
}


