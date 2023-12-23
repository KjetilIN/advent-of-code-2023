use std::collections::{BinaryHeap, HashMap};

use crate::{expand::{expand_horizontal, expand_vertically, get_line_length, self}, node::Node};

pub struct GalaxyMap{
    pub map: Vec<char>,
    pub galaxy_indexes: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

pub trait GalaxyMethods {
    fn from_file(content: String, expand: usize) -> Result<Self, String> where Self: Sized;
    fn get_neighbors(&self, index: u32) -> Vec<u32>;
    fn find_shortest_distance(&self, start: u32, end: u32) -> Option<u32>;
    fn find_sum_of_shortest_distances(&self) -> Result<u32, String>;
}

impl GalaxyMethods for GalaxyMap {
    fn from_file(content: String, expand: usize) -> Result<Self, String> where Self: Sized {
        let mut expanded_map = content;

        // Expand the map horizontally 
        expanded_map = match expand_horizontal(&expanded_map, expand){
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        
        // Expand the map vertically 
        expanded_map = match expand_vertically(&expanded_map, expand){
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        let width = match get_line_length(&expanded_map){
            Ok(val) => val as u32,
            Err(err) => return Err(err),
        };

        let height = expanded_map.lines().collect::<Vec<_>>().len() as u32;

        let map:Vec<char> = expanded_map.chars().filter(|&c| c != '\n').collect();
        let mut galaxy_indexes: Vec<u32> = Vec::new();

        // Iterate over each char in the map and add the index as a record in galaxy_indexes
        for (index, char) in map.iter().enumerate(){
            if char == &'#'{
                galaxy_indexes.push(index as u32)
            }
        }
        

        Ok(Self{
            map,
            galaxy_indexes,
            width,
            height
        })

    }

    /// Given a index in the map, finds all the neighbors.
    /// Returns a vector of the neighbor indexes
    fn get_neighbors(&self, index: u32) -> Vec<u32> {
        // Calculate the x and y coordinates 
        let x = (index % self.width) as i32;
        let y = (index / self.width) as i32;

        // Create a new vector of neighbors 
        let mut neighbors = Vec::new();

        // Check left
        if x > 0 {
            neighbors.push(index - 1);
        }
        // Check right
        if x < (self.width - 1) as i32 {
            neighbors.push(index + 1);
        }
        // Check up
        if y > 0 {
            neighbors.push(index - self.width);
        }
        // Check down
        if y < (self.height - 1) as i32 {
            neighbors.push(index + self.width);
        }

        neighbors
    }

    /// Finds the shortest distance between two indexes in the map of the galaxy.
    ///     - Uses Dijkstra's algorithm to calculate the shortest distances
    ///     - Takes a start and end index 
    /// 
    /// Returns an option of the shortest distance. 
    fn find_shortest_distance(&self, start_index: u32, end_index: u32) -> Option<u32> {
        // We store each distances for each node from the staring node in a hashmap 
        let mut distances: HashMap<u32, u32> = HashMap::new();

        // A priority queue that prioritizes nodes with the shortest distance
        let mut priority_queue: BinaryHeap<Node> = BinaryHeap::new();

        // Insert the starting node 
        distances.insert(start_index, 0);
        priority_queue.push(Node {
            index: start_index,
            distance: 0,
        });


        // Dijkstra's algorithm - while we have a element in the priority queue
        while let Some(Node { index, distance }) = priority_queue.pop() {
            // If the index is the same as the end index, we move 
            if index == end_index {
                return Some(distance);
            }
            // Get all the neighbors fot the node 
            for &neighbor in &self.get_neighbors(index) {
                // Each neighbor is a one char away from the current node
                let new_distance = distance + 1; 


                // Check if the old distance is larger then the new or that there are no record for the neighbor.
                // Then we add the node to the hashmap and push it to the queue
                if distances.get(&neighbor).map_or(true, |&old_distance| new_distance < old_distance) {
                    distances.insert(neighbor, new_distance);
                    priority_queue.push(Node { index: neighbor, distance: new_distance });
                }
            }
        }

        // No path has been found 
        None 
    }

    fn find_sum_of_shortest_distances(&self) -> Result<u32, String> {
        let mut result = 0;
        let total_pairs = self.galaxy_indexes.len() * (self.galaxy_indexes.len() - 1) / 2; // Total number of distinct pairs
        let mut completed_pairs = 0;

        // Iterate over each pair of galaxies (distinct pairs)
        for i in 0..self.galaxy_indexes.len() {
            for j in i + 1..self.galaxy_indexes.len() {
                let start_index = self.galaxy_indexes[i];
                let end_index = self.galaxy_indexes[j];

                // Find the shortest distance between the current pair of galaxies
                if let Some(distance) = self.find_shortest_distance(start_index, end_index) {
                    result += distance;
                } else {
                    return Err("No path found between galaxies".to_string());
                }

                // Increment the completed pairs counter
                completed_pairs += 1;

                // Print the percentage of completion
                let percentage = (completed_pairs as f64 / total_pairs as f64) * 100.0;
                println!("Progress: {:.2}%", percentage);
            }
        }

        Ok(result)
    }
}