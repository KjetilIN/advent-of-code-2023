use std::{fs::File, io::{Read, BufRead, BufReader}, path::Path};


fn number_from_string(input: &str) -> u32{
    let mut result: u32 = 0;

    for char in input.chars(){
        
    }


    // Return the result 
    result
}



fn main() -> std::io::Result<()>{
    // Print context line
    println!("--- Day 1 ---");

    // We store the total sum of the file to a variable 
    let mut total_sum_from_file: u32 = 0;

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

    // For each line we need to fine the number
    for line in content.lines(){
        println!("{}", line);
    }

    


    Ok(())
}
