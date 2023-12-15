use std::{path::Path, fs::File, io::{BufReader, Read}, process::exit, collections::HashMap, rc::Rc, cell::RefCell};

use crate::{node::{Node, NodeMethods}, tree_methods::build_tree_from_map};

mod node;
mod tree_methods;

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

    // Create variables 
    let mut time_vec: Vec<u64> = Vec::new();
    let mut record_vec: Vec<u64> = Vec::new();

    let mut has_steps = false;

    let mut steps = String::new();

    let mut root = Rc::new(RefCell::new(Node::empty_node((&"root").to_string())));
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    // For each line we need to fine the number
    for line in content.lines(){
        // Jump over empty line
        if line.is_empty(){
            continue;
        }

        // Collect the steps 
        if !has_steps{
            steps = line.trim().to_string();
            has_steps = true;
            continue;
        }

        // If the root node is not set
        if root.borrow().name == "root"{
            root = match Node::with_line(line.to_string()){
                Ok(node) => Rc::new(RefCell::new(node)),
                Err(_) => exit(1),
            } ;
        }

        let main_node_name = &line[0..3]; 
        let right_node_name = &line[7..10]; 
        let left_node_name = &line[12..15]; 

        nodes.insert(main_node_name.to_string(), (left_node_name.to_string(), right_node_name.to_string()));        
    }


    //println!("Hashmap: {:#?}", nodes);
    println!("Root: {:#?}", root);


    Ok(())
}
