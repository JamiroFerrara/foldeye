use rdrive::directory::Directory;

mod directory;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let dir = directory::Directory::new(&args[1])?;

    Ok(())
}
