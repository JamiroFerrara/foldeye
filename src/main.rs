use std::env::*;
use fwatcher::directory::*;
use fwatcher::chron::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = args();
    let paths = get_paths(args);
    println!("Starting folder watcher..!");

    for path in paths {
        let directory = Directory::new(&path.clone())?;
        let chron_timing = "1/1 * * * * *".to_string();

        println!("Starting ChronJob in path -> {}", path);
            let chron = Chron::new(path.clone(), chron_timing, directory);
            match chron.watch_folder(&path) {
                Ok(_) => println!("Folder watcher ended!"),
                Err(e) => println!("Error: {}", e),
        }
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
