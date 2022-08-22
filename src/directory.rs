#[derive(Debug)]
enum Entry {
    File(File),
    Directory(Directory),
}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
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
struct Directory {
    name: String,
    entries: Vec<Entry>,
}

impl Directory {
    fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            entries: Vec::new(),
        }
    }

    fn walk_directories(&mut self) -> Vec<Entry>{
        let mut entries = Vec<Entry>::new();
        for entry in &self.entries {
            match entry {
                Entry::Directory(dir) => {
                    dir.walk_directories();
                }
                Entry::File(file) => {
                    entries.push(file);
                }
            }
        }
    }

    fn add_file(&mut self, file: File) {
        self.entries.push(Entry(file));
    }

    fn add_directory(&mut self, directory: Directory) {
        self.entries.push(Entry(directory));
    }
}
