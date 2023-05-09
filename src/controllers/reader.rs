use std::fs::File;
use std::io::{self, Read};

fn Reader(file_path: String) -> io::Result<()> {
    // Open the RXDATA file
    let mut file = File::open(file_path)?;

    // Read the first 4 bytes to determine the file type
    let mut magic_bytes = [0; 4];
    file.read_exact(&mut magic_bytes)?;

    println!("{:?}", magic_bytes);

    Ok(())
}