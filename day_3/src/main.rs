use std::result;
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

    /**
     * Constructor function for creating a new row
    */

    fn new(rows: &'a Vec<String>) -> Self{
        let row_length = rows.len() as u32;
        let col_length = rows.get(0).unwrap().len().clone() as u32;

        Self { rows, col_length, row_length }
    }

    /**
     * FUNCTIONS FOR CHECKING IF THERE ARE ANY NUMBERS 
    */

    pub fn is_part_top_left(row_index:usize, char_index:usize) -> bool{
        todo!();
    }   

    pub fn is_part_top_right(row_index:usize, char_index:usize) -> bool{
        todo!();
    }   

    pub fn is_part_top(row_index:usize, char_index:usize) -> bool{
        todo!();
    }   

    pub fn is_part_left(row_index:usize, char_index:usize) -> bool{
        todo!();
    }   

    pub fn is_part_right(row_index:usize, char_index:usize) -> bool{
        todo!();
    }   

    pub fn is_part_bottom_left(row_index:usize, char_index:usize) -> bool{
        todo!();
    }   

    pub fn is_part_bottom_right(row_index:usize, char_index:usize) -> bool{
        todo!();
    }   

    pub fn is_part_bottom(row_index:usize, char_index:usize) -> bool{
        todo!();
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
                    result += Self::get_part_sum(row_index, char_index);
                }
            }
        }

        result
    }





    
}

/**
 * Method that gets the whole number given a index 
 * - Takes the row that the string is at
 * - Takes the index the number is found at 
 * 
 * Returns the number parsed
 * 
 */
pub fn get_number(number_string: &String, number_index:usize) -> u32{
    // Create a new string

    let char_vector: Vec<_> = number_string.chars().collect();
    

    // Check if the start index is numeric, else print the error and return 0 
    if !char_vector[number_index].is_numeric(){
        eprintln!("ERROR: get_number start index was not numeric, returned 0");
        return 0 as u32;
    }
    
    // Save the central char as a variable 
    let central_char = char_vector[number_index];
    println!("Central char: {}", central_char);

    //We add each numeric char that is to the left
    let mut right_index = number_index + 1;
    let mut right_number = String::new();

    while char_vector[right_index].is_numeric() && char_vector.len() > right_index  {
        right_number.push(char_vector[right_index]);
        right_index += 1;
    };

    println!("RIGHT: '{right_number}'");

    // Then we get all the chars from the front 
    let mut left_index = number_index - 1;
    let mut left_number = String::new();
    
    while char_vector[left_index].is_numeric() && left_index > 0 {
        left_number.push(char_vector[left_index]);
        left_index-=1;
    };

    println!("LEFT: '{left_number}'");


    // Format the result as a string 
    let result_string = if right_number.trim().is_empty() {
        format!("{}{}",left_number.chars().rev().collect::<String>(), central_char)
    } else if left_number.trim().is_empty() {
        format!("{}{}",central_char, right_number)
    } else {
        format!("{}{}{}", left_number.chars().rev().collect::<String>(),central_char,right_number)
    };

    println!("RESULT STING: {result_string}");

    match (result_string).parse(){
        Ok(number) => number,
        Err(err) => {
            eprintln!("ERROR: during parsing of get_number : {err}");
            panic!();
        }

    }
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
