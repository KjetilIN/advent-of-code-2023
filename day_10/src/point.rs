/// Struct that represents a point on a 2D plane
/// Has a x and y coordinate 
pub struct Point{
    pub x: i32,
    pub y: i32
}

impl Point {
    /// Creates a new point from the vector index 
    /// Takes the vector index and the width of the vector 
    /// Returns a new instance of self 
    pub fn from_vector_index(index: &u32, width: &u32) ->Self{
        let x = (index % width) as i32;
        let y = (index/ width) as i32;

        Self{
            x,
            y
        }

    }
}

