## FOLDEYE
A simple rust crate built for watching changes in a folder.

# Usage
```
use foldeye::*;

fn main() -> Result<(), std::io::Error> {
    let path = "./".to_string();
    let directory = Directory::new(&path.clone())?;
    let chron_timing = "1/1 * * * * *".to_string(); // Every second

    let chron = Chron::new(path.clone(), chron_timing, directory);
    chron.watch_folder(&path, &|comparison| {
        let add = comparison.inserted;
        let rem = comparison.removed;
        let rep = comparison.replaced;
    })?;

    Ok(())
}
```

https:://crates.io/crates/foldeye
