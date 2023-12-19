use crate::pipe::{Pipe, get_pipe};

struct MazeMap{
    map: Vec<char>,
    start_index: u32,
    map_width: u32,
    map_length: u32,
}


trait MazeMapMethods {
    fn from_file(content: &String) -> Self;
    fn count_half_circle(&self) -> u32;
}

fn find_start_index(map: Vec<char>) -> Option<u32>{
   for (index, pipe_char) in map.iter().enumerate(){
        if pipe_char == &'S'{
            return Some(index as u32);
        }
   }
   None
}