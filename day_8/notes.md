# Day 8 - Description

Given the input (explained in this readme):
1. Find the amount of steps needed to be taken between AAA and BBB, given the steps as instructions. 
2. Find the amount of steps needed to take a all nodes that end with A, so that they all end up on a node that ends with Z - simultaneously.  

Link to complete description: https://adventofcode.com/2023/day/8

## Input Description

Steps are defines as Left - L or Right R. 
A string of steps could be  => `RLLRRRLRLRLRRLLRRRR`

Then you are given a list of nodes, where each line represents a node: `11A = (11B, XXX)`.
The first node is the name of the node, then the left node and the right node are children of the node. 

A complete input could look like this:
```text
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
```

## Approach/Algorithm

This was a hard challenge, but I ended up with a good solution.
To define out network of nodes, we could create a Node struct, and create a binary three. That was my first attempt and it looked good, until I tried to parse the tree (it was not easy).

After thinking about this I found out that you actually do not need to make them children of each other. If you can access each node and find out what nodes they have as children, that is the same as a three!

With this in mind I implemented the following:
```rust 
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
```

The `Network` struct has a default root node and an end node. 
Is also has a hashmap - where each key is the node name and the value is a `Direction`.
The `Direction` struct was created for better readability and references the names of the nodes children. 
We could easily add a parent to the direction to get the ability to backtrace.

For the functions, I created: 

```rust 
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
```

The algorithm for part 1: 
1. Create a counter that is incremented for each step iteration
2. Iterate over each char in the steps
    1. This iteration is repeated infinite times.
3. For each step we iterate over:
    1. Check if our current node in the network is the end node, then we return the counter
    2. Else we change our current node to be the left or right node (based on current step)

The algorithm for part 2:
1. Find all the nodes that end with A and collect them in a vector
2. Count how many steps is needed of each of the nodes collected to get to the first node that ends with Z
3. Use LCM to calculate on what iteration all nodes will end on Z

## Learnings

Tried first using RC and RefCell but found out that we can just use a hashmap to explain the 
This article was useful. It showed me new datatype
    - `Rc<T>` => provides shared ownership of a value of type T, allocated in the heap. (My last attempt got stack overflow error, so this was very much needed)
    - `RefCell<T>` => is a type that allows for interior mutability, which means that it allows you to mutate the contents of value even if you only have an immutable reference
Source: https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/ 

For part 2, I learned that we could use LCM, instead of trying each step over and over again. 
If we know how many steps is needed to find the first node that ends with Z - we also know that the number of steps needed to be taken for all nodes simultaneously - is a multiple of each of the numbers for encountering the first z.
To be more specific, the lowest number that is also a multiple. 

To find that number I used LCM:
Read more here: https://en.wikipedia.org/wiki/Least_common_multiple


## Code Snippets

The code snippet for part 2:

```rust
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
```

**NOTE:** my lcm implementation is found in `src/lcm.rs`

