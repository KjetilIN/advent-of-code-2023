use std::{path::Path, fs::File, io::{BufReader, Read}};

mod pipe;
mod mazemap;

fn main() -> std::io::Result<()> {
    println!("--- Day 10: Pipe Maze ---");

    // Save the content of a file to a string; 
    let mut content = String::new();
    
    // Open the file of input relative to the folder
    let path = Path::new("oasis.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file. 
    // A buffer reader is a way to optimize reads, by making a single sys call 
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;

    Ok(())
}
