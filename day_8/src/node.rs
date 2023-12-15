use std::{cell::RefCell, rc::Rc};

/// Struct that represents a new node 
#[derive(Debug)]
pub struct Node{
    pub name: String,
    pub left: Option<NodeReference>,
    pub right: Option<NodeReference>,
}

pub type NodeReference = Rc<RefCell<Node>>;

pub trait NodeMethods {
    fn empty_node(name: String) -> Self;
    fn with_line(node_string: String) -> Result<Self, String> where Self: Sized;
    fn with_left_right(name: String, left: String, right: String) -> Result<Self, String> where Self: Sized;
    fn set_left(&mut self, left: Rc<RefCell<Node>>);
    fn set_right(&mut self, left: Rc<RefCell<Node>>);
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
    fn with_line(node_string: String) -> Result<Self, String> where Self: Sized {
        if node_string.len() != 16{
            eprintln!("ERROR: node length must be 16, was {}", node_string.len());
            return Err("Illegal node constructor".to_string());
        }

        let main_node_name = &node_string[0..3]; 
        let right_node_name = &node_string[7..10]; 
        let left_node_name = &node_string[12..15]; 

        let left_node = Node::empty_node(left_node_name.to_string());
        let right_node = Node::empty_node(right_node_name.to_string());

        Ok(Self{
            name: main_node_name.to_string(),
            left: Some(Rc::new(RefCell::new(left_node))),
            right: Some(Rc::new(RefCell::new(right_node)))
        })
    }

    fn with_left_right(name: String, left: String, right: String) -> Result<Self, String> where Self: Sized{
        let left_node = Node::empty_node(left);
        let right_node = Node::empty_node(right);

        Ok(Self{
            name,
            left: Some(Rc::new(RefCell::new(left_node))),
            right: Some(Rc::new(RefCell::new(right_node)))
        })
    }

    fn set_left(&mut self, left: Rc<RefCell<Node>>) {
        self.left = Some(left);
    }

    fn set_right(&mut self, right: Rc<RefCell<Node>>) {
        self.right = Some(right);
    }

}

    
