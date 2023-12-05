use std::{fs::File, io::BufReader};
use std::io::Read;

/**
    The struct that represent an engine schema 
    - Has the vectors of all the rows
    - Has the length of columns and rows
 */
pub struct EngineSchema<'a>{
    pub rows: &'a Vec<String>,
    pub col_length: u32,
    pub row_length: u32
}

impl<'a> EngineSchema<'a>{  

    /**
        Constructor function for creating a new Engine Schema Struct
        - Takes a vector of strings
        - The life time of the struct is the same as the Vector given
    */

    pub fn new(rows: &'a Vec<String>) -> Self{
        let row_length = rows.len().clone() as u32;
        let col_length = rows.get(0).unwrap().len().clone() as u32;

        Self { rows, col_length, row_length }
    }


    /**
      Method that gets the char from the map vector.
      - Gets the indexes 
      
      Returns an option with the char or none if illegal indexes where given 
    */
    pub fn get_char_from_vector(&self, row_index:usize, char_index:usize) -> Option<char>{

        // Validating input or panics 
        if row_index > self.row_length.try_into().unwrap() {
            eprintln!("ERROR: illegal row index given: {row_index}");
            return None;
        }else if char_index > self.col_length.try_into().unwrap(){
            eprintln!("ERROR: illegal char index given: {char_index}");
            return None;
        }

        // Get row from the option
        let binding = self.rows.get(row_index);
        let row = match &binding{
            Some(row) => row,
            None => {
                eprintln!("ERROR: illegal was not able to get row: {row_index}");
                return None},
        };
        let row_chars: Vec<_> = row.chars().collect();

        // return the option
        Some(row_chars[char_index])
    }


    /**
      Method that checks if a given index is a part number / aka a digit
      - Takes the row index and char index of the char we want to check
      - Checks that the given indexes could be an item on the map
      - (Uses get_char_from_vector method for validating)
      
      Returns true if the item is a number
    */
    pub fn is_part_number(&self, row_index:usize, char_index:usize) -> bool{
        match self.get_char_from_vector(row_index, char_index){
            Some(item) => item.is_numeric(),
            None => false
        }
    }   

    /**
      Function that takes finds the sum part for a single part
      - Takes the row index and char index of the given part 
      
      Returns the sum or 0 if no parts for that sum is available 
    */
    pub fn get_part_sum(&self, row_index:usize, char_index:usize) -> u32{
        let mut sum = 0;

        // Declare variables for the indexes
        let top_row_index = row_index - 1;
        let bottom_row_index = row_index + 1;
        let left_char_index = char_index - 1;
        let right_char_index = char_index + 1;

        // Check all sides for the number
        let number_top = self.is_part_number(top_row_index, char_index);
        let number_left = self.is_part_number(row_index, left_char_index);
        let number_right = self.is_part_number(row_index, right_char_index);
        let number_bottom = self.is_part_number(bottom_row_index, char_index);

        // Check the top
        if number_top{
            let top_number = get_number(self.rows.get(top_row_index).unwrap(), char_index);
            sum += top_number;

        }else{
            let number_top_left = self.is_part_number(top_row_index, left_char_index);
            let number_top_right = self.is_part_number(top_row_index, right_char_index);
            // There is no number on the top, 
            // and if there are numbers to the left and right, we know that they are not the same numbers 
            // There is a space between the numbers
            if number_top_left {
                let top_left_number = get_number(self.rows.get(top_row_index).unwrap(), left_char_index);
                sum += top_left_number;
            }

            if number_top_right {
                let top_right_number= get_number(self.rows.get(top_row_index).unwrap(), right_char_index);
                sum += top_right_number;
            }
        }

        // Check both sides 
        if number_left{
            let left_number = get_number(self.rows.get(row_index).unwrap(), left_char_index);
            sum += left_number;
        }

        if number_right{
            let right_number = get_number(self.rows.get(row_index).unwrap(), right_char_index);
            sum += right_number;
        }


        // Check the bottom
        if number_bottom{
            let bottom_number = get_number(self.rows.get(bottom_row_index).unwrap(), char_index);
            sum += bottom_number;

        }else{
            // There is no number on the bottom, 
            // and if there are numbers to the left and right diagonal, we know that they are not the same numbers 
            // There is a space between the numbers

            
            let number_bottom_right = self.is_part_number(bottom_row_index, right_char_index);
            let number_bottom_left = self.is_part_number(bottom_row_index, left_char_index);
            
            if number_bottom_left {
                let bottom_left = get_number(self.rows.get(bottom_row_index).unwrap(), left_char_index);
                sum += bottom_left;
            }

            if number_bottom_right {
                let bottom_right = get_number(self.rows.get(bottom_row_index).unwrap(), right_char_index);
                sum += bottom_right;
            }
        }



        sum
    }


    /**
      Method that gets the total part sum for all of the parts in the constructed map
      - Uses the self keyword 
      Returns the result of all the parts 
     */
    pub fn get_total_part_sum(&self) -> u32{
        let mut result: u32 = 0;

        for (row_index, row) in self.rows.iter().enumerate(){
            for (char_index, ch) in row.chars().enumerate(){
                // If it is a special char, we have found a special char
                if !ch.is_numeric() && ch != '.'{
                    result += Self::get_part_sum(self, row_index, char_index);
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
pub fn get_number(row: &String, number_index:usize) -> u32{
    // Create a new vector of chars from the string row
    let char_vector: Vec<_> = row.chars().collect();

    // Check if the start index is numeric, else print the error and return 0 
    if !char_vector[number_index].is_numeric(){
        eprintln!("ERROR: get_number start index was not numeric, returned 0");
        return 0 as u32;
    }
    
    // Save the central char as a variable 
    let central_char = char_vector[number_index];

    //We add each numeric char that is to the left
    let mut right_index = number_index + 1;
    let mut right_number = String::new();

    while char_vector[right_index].is_numeric() && char_vector.len() > right_index  {
        right_number.push(char_vector[right_index]);

        if right_index == char_vector.len() - 1{
            break;
        }
        right_index += 1;
    };

    // Then we get all the chars from the front 
    let mut left_index = number_index - 1;
    let mut left_number = String::new();
    
    while char_vector[left_index].is_numeric() {
        left_number.push(char_vector[left_index]);
        
        if left_index == 0{
            break;
        }
        left_index-=1;
    };


    // Format the result as a string 
    let result_string = if right_number.trim().is_empty() {
        format!("{}{}",left_number.chars().rev().collect::<String>(), central_char)
    } else if left_number.trim().is_empty() {
        format!("{}{}",central_char, right_number)
    } else {
        format!("{}{}{}", left_number.chars().rev().collect::<String>(),central_char,right_number)
    };

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

    let engine = EngineSchema::new(&parts_vector);
   

    println!("Total part sum: {}", engine.get_total_part_sum());


    Ok(())
}
