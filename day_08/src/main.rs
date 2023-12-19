mod network;
mod test;
mod lcm;

use std::{path::Path, fs::File, io::{BufReader, Read}, collections::HashMap, process::exit};

use crate::network::{Network, NetworkMethods, Direction};


fn main() -> std::io::Result<()> {
    println!("--- Day 8: Haunted Wasteland ---");

    // Save the content of a file to a string; 
    let mut content = String::new();
    
    // Open the file of input relative to the folder
    let path = Path::new("nodes.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file. 
    // A buffer reader is a way to optimize reads, by making a single sys call 
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;

    // Store the steps in a string variable 
    let mut steps = String::new();

    // Store each node
    let mut nodes: HashMap<String, Direction> = HashMap::new();

    // For each line we need to fine the number
    for line in content.lines(){
        // Jump over empty line
        if line.is_empty(){
            continue;
        }

        // Collect the steps, if not collected before
        if steps.is_empty(){
            steps = line.trim().to_string();
            continue;
        }

        // Get the name of the 
        let main_node_name = &line[0..3]; 
        let left_node_name = &line[7..10]; 
        let right_node_name = &line[12..15]; 

        // Inset the new node
        nodes.insert(main_node_name.to_string(), Direction::new(left_node_name.to_string(), right_node_name.to_string()));    
    }

    // Create a new network 
    let network: Network = Network::with_map(nodes);

    // Count each steps from the start node to the end node
    let end_node = match network.count_steps(&steps){
        Ok(end) => end,
        Err(_) => exit(1),
    };

    // Count the escape steps for all the starting Z
    let escape_steps = match network.steps_to_escape_all(&steps){
        Ok(end) => end,
        Err(_) => exit(1),
    };

    // Answer => 13771
    println!("End node steps (part 1): {end_node}");

    // Answer => 13129439557681
    println!("End node escape steps (part 2): {escape_steps}");


    Ok(())
}
