#[cfg(test)]
mod tests {
    use std::{collections::HashMap, process::exit};

    use crate::network::{Network, NetworkMethods, Direction};


    #[test]
    fn test_walk_z(){
        let content = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        let mut steps = String::new();
        let mut has_steps = false;

        let mut nodes: HashMap<String, Direction> = HashMap::new();

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

            let main_node_name = &line[0..3]; 
            let left_node_name = &line[7..10]; 
            let right_node_name = &line[12..15]; 

            nodes.insert(main_node_name.to_string(), Direction::new(left_node_name.to_string(), right_node_name.to_string()) );        
        }

        let network: Network = Network::with_map(nodes);

        let end_node = match network.count_steps(&steps){
            Ok(end) => end,
            Err(_) => exit(1),
        };

        assert_eq!(end_node,2);

    }

    #[test]
    fn test_steps_to_escape(){
        let content = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let mut steps = String::new();
        let mut has_steps = false;

        let mut nodes: HashMap<String, Direction> = HashMap::new();

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

            let main_node_name = &line[0..3]; 
            let left_node_name = &line[7..10]; 
            let right_node_name = &line[12..15]; 

            nodes.insert(main_node_name.to_string(), Direction::new(left_node_name.to_string(), right_node_name.to_string()));        
        }

        let network: Network = Network::with_map(nodes);

        let steps_to_escape = match network.steps_to_escape_all(&steps){
            Ok(end) => end,
            Err(_) => exit(1),
        };

        assert_eq!(steps_to_escape,6);

    }

    
}