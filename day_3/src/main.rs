use std::{fs::File, io::BufReader};
use std::io::Read;

/**
 * The struct that represent an engine schema 
 */
struct EngineSchema<'a>{
    rows: &'a Vec<String>,
    col_length: u32,
    row_length: u32
}

impl<'a> EngineSchema<'a>{

    fn new(rows: &'a Vec<String>) -> Self{
        let row_length = rows.len() as u32;
        let col_length = rows.get(0).unwrap().len().clone() as u32;

        Self { rows, col_length, row_length }
    }
    /**
     * Method that gets the total part sum for all of the parts in the constructed map
     * Returns the result of all the parts 
     */
    fn get_total_part_sum(&self) -> u32{
        let mut result: u32 = 0;

        for (row_index, row) in self.rows.iter().enumerate(){
            for (char_index, ch) in row.chars().enumerate(){
                // If it is a special char, we have found a special char
                if !ch.is_alphanumeric() || ch != '.'{
                    result += get_part_sum(row_index, char_index);
                }
            }
        }

        result
    }


    
}

/**
 * Function that takes finds the sum part for a single part
 * Takes the row index and char index of the given part 
 * Returns the sum or 0 if no parts for that sum is available 
 */
pub fn get_part_sum(row_index:usize, char_index:usize) -> u32{
    let mut sum = 0;



    sum
}


fn main() -> std::io::Result<()> {
    println!("--- Day 3: Gear Ratios ---");

    let mut content = String::new();

    let file = File::open("map.txt")?;
    let mut buf_reader = BufReader::new(file);

    buf_reader.read_to_string(&mut content).unwrap_or_else(|err|{
        eprintln!("Error reading the map file : {err}");
        panic!();
    });

    let mut parts_vector = Vec::<String>::new();
    for line in content.lines(){
        parts_vector.push(line.to_string().clone());
    }

    let parts_map = EngineSchema::new(&parts_vector);
   

    println!("Parts Map size: {}x{}", parts_map.col_length, parts_map.row_length);


    Ok(())
}
