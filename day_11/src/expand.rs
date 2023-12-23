/// Returns true if any horizontal lines does not contain a galaxy ("#")
fn should_expand_horizontal(content: &String) -> bool{
    for line in content.lines(){
        if !line.contains("#"){
            return true;
        }
    }

    false
}

/// Gets the line width for a &String. 
/// Returns a result (error if no line was found)
pub fn get_line_length(content: &String) -> Result<usize, String>{
    content.lines().next().map_or_else(|| {
        eprintln!("ERROR: No lines found");
        Err("No lines found".to_string())
    }, |line| Ok(line.len()))
}


/// Checks if there is a column without a galaxy ("#").
/// Returns a true and vector of all the column indexes that are empty 
fn should_expand_vertical(content: &str) -> (bool, Vec<usize>) {
    // Split the input content into lines and collect them into a vector of string slices.
    let lines: Vec<&str> = content.lines().map(|line| line.trim()).collect();

    // Get the width of the map by taking the length of the first line (assuming all lines have the same length).
    let width = lines.first().map_or(0, |line| line.len());

    // Initialize a vector to store the indices of empty columns in each line.
    let mut empty_columns: Vec<usize> = Vec::new();

    // Get all of the vertical lines in the 
    let vertical_lines: Vec<String> = (0..width).map(|col| {
        lines.iter().map(|line| line.chars().nth(col).unwrap_or(' ')).collect()
    }).collect();


    for (index, v_line) in vertical_lines.iter().enumerate(){
        if !v_line.contains("#"){
            empty_columns.push(index);
        }
    }


    (empty_columns.len() > 0, empty_columns)
}

/// Takes a galaxy map and expands it horizontally  
/// Returns the new expanded map or an error if map could not be expanded
pub fn expand_horizontal(expanded_map: &String) -> Result<String, String>{
    let mut result = String::new();

    let length = match get_line_length(&expanded_map){
        Err(err) => return Err(err),
        Ok(val) => val, 
    };

    if should_expand_horizontal(&expanded_map){
        let mut new_lines = String::new();

        for line in expanded_map.lines() {
            new_lines.push_str(line);
            new_lines.push('\n');
            if !line.contains("#"){
                new_lines.push_str(&".".repeat(length));
                new_lines.push('\n');
            }
            
        }

        result.push_str(&new_lines);
    }else{
        result = expanded_map.clone();
    }

    Ok(result.to_string())
}


/// Expands the map vertically. 
/// Finds all the columns that should be expanded adds a new column to the left of empty columns
pub fn expand_vertically(expanded_map: &String) -> Result<String, String>{

    let mut result = String::new();

    let (expand_vertically, indexes) = should_expand_vertical(&expanded_map);

    if expand_vertically {
        let mut new_lines = String::new();
    
        for line in expanded_map.lines() {
            let mut modified_line = String::new();
    
            for (col, char) in line.chars().enumerate() {
                modified_line.push(char);
    
                // Check if the current column index is in the list of empty columns
                if indexes.contains(&col) {
                    modified_line.push('.');
                }
            }
    
            new_lines.push_str(&modified_line);
            new_lines.push('\n');
        }
    
        result.push_str(&new_lines);
    }else{
        result = expanded_map.clone();
    }


    Ok(result)

}