## FOLDEYE
A simple rust crate built for watching changes in a folder.

# Usage
```
let directory = Directory::new(&path.clone())?;
let chron_timing = "1/1 * * * * *".to_string();

Chron::new(path.clone(), chron_timing, directory);
```

https:://crates.io/crates/foldeye
