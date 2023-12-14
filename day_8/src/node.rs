/// Struct that represents a new node 
#[derive(Debug)]
pub struct Node{
    pub name: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub trait NodeMethods {
    fn empty_node(name: String) -> Self;
    fn create_nodes(node_string: String) -> Result<Self, String> where Self: Sized;
    fn set_left_node(&mut self, node: Box<Node>);
    fn set_right_node(&mut self, node: Box<Node>);
    fn swap_self(&mut self, node: Box<Node>);
}

impl NodeMethods for Node {
    fn empty_node(name: String) -> Self {
        Self{
            name,
            left: None,
            right: None,
        }
    }

    /// Creates a node from the given node string
    /// Example:
    ///     - NQT = (TXC, RVJ)
    fn create_nodes(node_string: String) -> Result<Self, String> where Self: Sized {
        if node_string.len() != 16{
            eprintln!("ERROR: node length must be 16, was {}", node_string.len());
            return Err("Illegal node constructor".to_string());
        }

        let main_node_name = &node_string[0..3]; 
        let right_node_name = &node_string[7..10]; 
        let left_node_name = &node_string[12..15]; 

        println!("NODE names: {}, {}, {}", main_node_name, right_node_name, left_node_name);

        let left_node = Node::empty_node(left_node_name.to_string());
        let right_node = Node::empty_node(right_node_name.to_string());

        Ok(Self{
            name: main_node_name.to_string(),
            left: Some(Box::new(left_node)),
            right: Some(Box::new(right_node))
        })
    }

    fn set_left_node(&mut self, node: Box<Node>) {
        self.left = Some(node);
    }

    fn set_right_node(&mut self, node: Box<Node>) {
        self.right = Some(node);
    }

    fn swap_self(&mut self, node: Box<Node>) {
        self.name = node.name.clone();
        self.left = node.left;
        self.right = node.right;
    }

}

    
