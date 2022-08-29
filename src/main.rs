use std::env::*;
use foldeye::directory::*;
use foldeye::chron::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = args();
    let paths = get_paths(args);
    println!("Starting folder watcher..!");

    for path in paths {
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

fn get_paths(args: std::env::Args) -> Vec<String> {
    let mut paths = Vec::new();
    for arg in args {
        paths.push(arg);
    }

    //Remove first dummy argument
    paths.remove(0);
    paths
}
