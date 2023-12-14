use std::{path::Path, fs::File, io::{BufReader, Read}, process::exit, collections::{HashMap, HashSet}};

use crate::{node::{Node, NodeMethods}, tree_methods::add_node};

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

    let mut root = Node::empty_node((&"e").to_string());
    let mut nodes_not_added: HashMap<String, Node> = HashMap::new();

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
        if root.name == "e"{
            root = match Node::create_nodes(line.to_owned()){
                Ok(node) => node,
                Err(_) => exit(1),
            } ;
        }

        let mut node_to_add = match Node::create_nodes(line.to_owned()) {
            Ok(node) => node,
            Err(_) => exit(1),
        };

        // Find the node 
        match add_node(&mut root, &mut node_to_add){
            Ok(_) => {},
            Err(_) => {
                //eprintln!("ERROR: could not add node: {:#?}", node_to_add);
                nodes_not_added.insert(node_to_add.name.clone(), node_to_add);
            },
        }
    }


    Ok(())
}
