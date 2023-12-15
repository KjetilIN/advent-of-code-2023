use std::{collections::HashMap, rc::Rc, cell::RefCell, borrow::BorrowMut};

use crate::node::{Node, NodeMethods, NodeReference};

pub fn build_tree_from_map(root: &mut Rc<RefCell<Node>>, nodes: &mut HashMap<String, (String, String)>) {
    todo!()
}