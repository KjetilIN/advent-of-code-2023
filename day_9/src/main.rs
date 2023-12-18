use std::{path::Path, fs::File, io::{BufReader, Read}};

use crate::prediction::{Prediction, PredictionMethods};

mod prediction;
mod test;

fn main() -> std::io::Result<()>{
    println!("--- Day 9: Mirage Maintenance ---");

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

    let mut part_one_sum = 0;

    for line in content.lines(){
        let prediction = match Prediction::with_numbers(line){
            Ok(val) => val,
            Err(_) =>{
                panic!("Failed test: Constructor was illegal ");
            }
        };

        part_one_sum += prediction.predict_next_number();
    }

    // Answers for part 1 => 1953784198
    println!("Extrapolated values: {}", part_one_sum);

    Ok(())

}
