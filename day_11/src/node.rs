use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Node {
    pub index: u32,
    pub distance: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}