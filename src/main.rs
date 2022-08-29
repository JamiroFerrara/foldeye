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
        chron.watch_folder(&path, &|inserted, deleted| {
            println!("Inserted: {:?}", inserted);
            println!("Deleted: {:?}", deleted);
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
