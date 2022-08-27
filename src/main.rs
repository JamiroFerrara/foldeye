use clap::Parser;

mod directory;

///Argument parser for the start of the program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Path to search
    n: String
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let dir = directory::Directory::new(&args.n)?;
    let files = dir.all_dirs();
    println!("{:?}", files);

    Ok(())
}
