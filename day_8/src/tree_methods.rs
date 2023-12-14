use std::collections::HashMap;

use crate::node::{Node, NodeMethods};

pub fn add_node(root: &mut Node, node_to_add: &mut Node) -> Result<(), String>{

    let mut node_to_replace = match traverse_level_order(root, node_to_add.name.clone()) {
        Some(node) => node,
        None => {
            eprintln!("ERROR: did not find the node with the name '{}'", node_to_add.name);
            return Err("Did not find node".to_string());
        }
    };

   node_to_replace = node_to_add;

    Ok(())
}


fn traverse_level_order(node: &mut Node, node_to_find_name: String) -> Option<&mut Node>{
    if node.name == node_to_find_name{
        return Some(node);
    }

    if let Some(left_node) = &mut node.left{
        if left_node.name == node_to_find_name{
            return Some(left_node);
        }else{
            return traverse_level_order(left_node, node_to_find_name)
        }
    }

    if let Some(right_node) = &mut node.right{
        if right_node.name == node_to_find_name{
            return Some(right_node);
        }else{
            return traverse_level_order(right_node, node_to_find_name)
        }
    }

    None
}


fn build_tree_from_map(root: &'a mut Node, nodes: &mut HashMap<String, Node>) -> &'a mut Node{

    while !nodes.is_empty(){
        

    }


    root
} 