use std::{path::Path, fs::File, io::{BufReader, Read}};

use crate::mazemap::{MazeMap, MazeMapMethods};

mod pipe;
mod mazemap;
mod direction;
mod test;

fn main() -> std::io::Result<()> {
    println!("--- Day 10: Pipe Maze ---");

    // Save the content of a file to a string; 
    let mut content = String::new();
    
    // Open the file of input relative to the folder
    let path = Path::new("map.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file. 
    // A buffer reader is a way to optimize reads, by making a single sys call 
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;

    let map: MazeMap = match MazeMap::from_file(&content){
        Ok(m) => m,
        Err(_) => {
            eprintln!("ERROR: creating the Mazemap");
            panic!();
        }
    };

    println!("{:#?}", map);


    Ok(())
}
