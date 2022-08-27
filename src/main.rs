use clap::Parser;

mod directory;
mod chron;

///Argument parser for the start of the program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Path to search
    path: String,
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let path = args.path.clone();
    let directory = directory::Directory::new(&path.clone())?;
    let chron_timing = "1/1 * * * * *".to_string();

    chron::Chron::new(path.clone(), chron_timing, directory).watch_folder(&path)?;

    Ok(())
}
