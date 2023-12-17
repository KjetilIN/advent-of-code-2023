use std::collections::HashMap;

use crate::lcm::find_lcm;

/// Network of nodes
///  - Has a root node name
///  - Has a end node name
///  - Contains a map to a new direction
#[derive(Clone)]
pub struct Network{
    root_node_name: String,
    end_node_name: String,
    map: HashMap<String, Direction>
}


/// Represents a direction of a node
///  - Only two directions, left and right
#[derive(Clone)]
pub struct Direction{
    left: String,
    right: String
}

impl Direction {
    /// Create a new direction object
    pub fn new(left: String, right: String) -> Self{
        Self{
            left,
            right
        }
    }
}

pub trait NetworkMethods{
    fn with_map(map: HashMap<String, Direction>) -> Self;
    fn walk_steps(&self, steps: &String)-> Result<String, String>;   
    fn count_steps(&self, steps: &String)-> Result<u64, String>;
    fn steps_to_escape_all(&self, steps: &String)-> Result<u64, String>;
    fn get_z_in_path(&self, steps: &String, start:String) -> Result<u64, String>;
}

impl NetworkMethods for Network {
    /// Create a new network with the given map
    /// - Sets start node to be **AAA**
    /// - Sets end node to be **ZZZ**
    /// Returns a new network 
    fn with_map(map: HashMap<String, Direction>) -> Self {
        Self{
            root_node_name: "AAA".to_string(),
            end_node_name: "ZZZ".to_string(),
            map
        }
    }

    /// Count how many steps it takes to get to the end node
    /// - Uses the given steps, which must be a string of _L_ and _R_
    /// - Loops over each char in the streps until the end node has been found
    /// Returns how many steps it took. 
    fn count_steps(&self, steps: &String)-> Result<u64, String> {
        let mut current_node: String = String::from(&self.root_node_name);
        let mut steps_count: u64 = 0; 

        loop{
            for direction in steps.chars(){
                // If we are at the end node, we are done
                if current_node == self.end_node_name{
                    return Ok(steps_count);
                }

                // Increment steps
                steps_count += 1;

                // Get the directions dor left and right node
                let dir_options: &Direction = match self.map.get(&current_node){
                    Some(dir) => dir,
                    None => {
                        eprintln!("ERROR: could not find left or right node:{current_node}");
                        return Err("Node not in map".to_string());
                    },
                };

                // If we are asked to go left
                if direction == 'L'{
                    current_node = dir_options.left.clone();
                    continue;
                } 
                
                // If we are asked to go left
                if direction == 'R'{
                    current_node = dir_options.right.clone();
                    continue;
                } 
            }
        }

       
    }

    /// Walk the given steps
    /// Returns the current node
    fn walk_steps(&self, steps: &String)-> Result<String, String> {
        let mut current_node: String = String::from(&self.root_node_name);

        for direction in steps.chars(){

            let (left, right) = match self.map.get(&current_node){
                Some(dir) =>(dir.left.clone(), dir.right.clone()),
                None => {
                    eprintln!("ERROR: could not find left or right node:{current_node}");
                    return Err("Node not in map".to_string());
                },
            };

            if direction == 'L'{
               current_node = left.clone();
               continue;
            }

            if direction == 'R'{
                current_node = right.clone();
                continue;
             }


        }


        Ok(current_node)
       
    }

    /// Finds all nodes that end with A. 
    /// Returns the total of steps needed for all to simultaneously end up on a node that ends with Z
    fn steps_to_escape_all(&self, steps: &String)-> Result<u64, String> {
        // Retrieve all nodes that end with Z in our current map 
        let current_nodes: Vec<String> = get_all_nodes_with_end_a(&self.map);
        let mut z_encounters: Vec<u64> = Vec::new();

        // For each node 
        for node in current_nodes{
            // Find the first node that ends with Z
            let z_count = match self.get_z_in_path(steps, node){
                Ok(val) => val,
                Err(er) => return Err(er),
            };

            // Push it to the vector of encounters. 
            z_encounters.push(z_count);
        }

        // Calculate the least common multiple between all encounters and return them
        Ok(find_lcm(z_encounters))
    }

    /// Given a starting node, count how many steps with the following steps was needed to find the first node ending with Z
    /// - Takes both the steps (of R and Ls) and staring node name
    fn get_z_in_path(&self, steps: &String, start:String) -> Result<u64, String> {
        let mut current_node = start.clone();
        let mut steps_count: u64 = 0; 
        loop{
            for direction in steps.chars(){
                // For each node, change the current node to left or right 
                let directions_options: &Direction = match self.map.get(&current_node) {
                    Some(dir) => dir,
                    None => {
                        eprintln!("ERROR: could not find left or right node: {}", current_node);
                        return Err("Node not in map".to_string());
                    }
                };
    
                if direction == 'L' {
                    current_node = directions_options.left.clone();
                } else{
                    current_node = directions_options.right.clone();
                }

                                
                // Increment the step
                steps_count +=1;

                // Check if we found the node that ends with Z
                if current_node.ends_with("Z"){
                    return Ok(steps_count);
                }
            }
        }
    }
}

/// Gets all nodes ending with A
/// - Takes a map of string as key and direction as value
/// Returns a vector of all nodes that has an ending with **A**
fn get_all_nodes_with_end_a(map: &HashMap<String, Direction>) -> Vec<String> {
    map.keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect()
}