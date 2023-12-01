use std::{fs::File, path::Path, io::{BufReader, Read}};




/**
 * Calculate the number for a given string with random chars
 *  - it will always take the first number and add the last number
 *  - safg2dsfs2 => 22 
 *  - Will return a number that is x digits long, given x digits in the string
 *  - takes a string slice and returns a u32
 */
fn number_from_string(input: &str) -> u32{
    // Save the result in a variable 
    let mut first_digit:u32 = 0;
    let mut last_digit:u32 = 0;

    // Iterate over the chars 
    for ch in input.chars(){
        // Check if the char is numeric
        if ch.is_numeric(){
            // Set the first digit if it has not been set
            if first_digit == 0{
                first_digit = ch.to_digit(10).unwrap()
            }
            // Reset the last number 
            last_digit = ch.to_digit(10).unwrap();
        }
    }
    // Return the result as a two digit number
    first_digit*10 + last_digit
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
        // Add the number from the file to the total sum
        total_sum_from_file += number_from_string(line.trim());

    }

    // Print the result
    println!("Result: {}", total_sum_from_file);
    
    // Return OK
    Ok(())
}
