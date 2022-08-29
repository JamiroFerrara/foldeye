## FOLDEYE
A simple rust crate built for watching changes in a folder.

# Usage
```
use foldeye::*;

fn main() -> Result<(), std::io::Error> {
    let path = "./".to_string();
        let directory = Directory::new(&path.clone())?;
        let chron_timing = "1/1 * * * * *".to_string();

        let chron = Chron::new(path.clone(), chron_timing, directory);
        chron.watch_folder(&path, &|comparison| {
            for action in comparison.action {
                match action {
                    ComparisonActionEnum::Inserted(files) => { println!("Inserted: {:?}", files); }
                    ComparisonActionEnum::Removed(files) => { println!("Removed: {:?}", files); }
                    ComparisonActionEnum::Replaced(files) => { println!("Replaced: {:?}", files); }
                }
            }
        })?;
    }

    Ok(())
}
```

https:://crates.io/crates/foldeye
