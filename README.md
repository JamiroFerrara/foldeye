## FOLDEYE
A simple rust crate built for watching changes in a folder.

# Usage
```
let directory = Directory::new(&path.clone())?;
let chron_timing = "1/1 * * * * *".to_string();

let chron = Chron::new(path.clone(), chron_timing, directory);
chron.watch_folder(&path, &|comparison| {
    let add = comparison.inserted;
    let rem = comparison.removed;
    let rep = comparison.replaced;

})?;
```

https:://crates.io/crates/foldeye
