use std::{path::Path, fs::File, io::{BufReader, Read}, process::exit};

use crate::galaxy_map::{GalaxyMap, GalaxyMethods};

mod galaxy_map;
mod expand;
mod test;
mod node;

fn main() -> std::io::Result<()> {
    println!("--- Day 11: Cosmic Expansion ---");

    // Save the content of a file to a string; 
    let mut content = String::new();
    
    // Open the file of input relative to the folder
    let path = Path::new("input.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file. 
    // A buffer reader is a way to optimize reads, by making a single sys call 
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;

    let galaxy = match GalaxyMap::from_file(content.clone(), 2){
        Ok(g) => g,
        Err(_) => exit(1),
    };

    let sum = match galaxy.find_sum_of_shortest_distances() {
        Err(_) => exit(1),
        Ok(val) => val
    };


    // Answer part 1 => 9805264
    println!("Sum of shortest paths (part 1): {}", sum);

    // Part 2 => to slow
    let _large_galaxy = match GalaxyMap::from_file(content, 1_000_000){
        Ok(g) => g,
        Err(_) => exit(1),
    };



    Ok(())
}
