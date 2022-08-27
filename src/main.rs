use clap::Parser;

mod directory;
mod chron;


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
    chron::start(&args.n);
    Ok(())
}
