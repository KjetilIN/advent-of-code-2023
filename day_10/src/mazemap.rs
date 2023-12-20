use crate::{direction::Direction, pipe::{get_pipe, Pipe}};

#[derive(Debug)]
pub struct MazeMap {
    pub map: Vec<char>,
    pub start_index: u32,
    pub map_width: u32,
    pub map_height: u32,
}

pub trait MazeMapMethods {
    fn from_file(content: &String) -> Result<Self, String>
    where
        Self: Sized;
    fn count_half_circle(&self) -> u32;
    fn find_starting_pipe(&self) -> Option<(u32, Direction)>;
    fn get_north(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_east(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_west(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_south(&self, index:u32) -> Option<(u32, Direction)>;
}

impl MazeMapMethods for MazeMap {
    fn from_file(content: &String) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut map: Vec<char> = Vec::new();
        let mut map_height: u32 = 0;
        let mut map_width: u32 = 0;
        for line in content.lines() {
            if map_width == 0 {
                map_width = line.len() as u32;
            }

            line.chars().into_iter().for_each(|ch| map.push(ch));

            println!("Line: {line}");
            map_height += 1;
        }

        let start_index = match find_start_index(&map) {
            Some(value) => value,
            None => {
                eprintln!("ERROR: did not find start index for map");
                return Err("Illegal constructor parameters".to_string());
            }
        };

        Ok(Self {
            map,
            map_height,
            map_width,
            start_index,
        })
    }

    fn count_half_circle(&self) -> u32 {
        todo!()
    }

    fn find_starting_pipe(&self) -> Option<(u32, Direction)>{
        todo!()
    }

    fn get_north(&self, index:u32) -> Option<(u32, Direction)> {
        // Check that top boundary is okay
        if index < self.map_width{
            return None;
        }

        let new_index = index - self.map_width;
        let char = self.map[new_index as usize];
        let pipe: Pipe = get_pipe(char);

        match pipe{
            Pipe::Vertical => return Some((new_index, Direction::North)),
            Pipe::NorthEast => return Some((new_index, Direction::East)),
            Pipe::NorthWest => return Some((new_index, Direction::West)),
            _ => return  None,
           
        }
    }

    fn get_east(&self, index:u32) -> Option<(u32, Direction)> {
        // Check that top boundary is okay
        if index + 1 > self.map_width{
            return None;
        }

        let new_index = index + 1;
        let char = self.map[new_index as usize];
        let pipe: Pipe = get_pipe(char);

        match pipe{
            Pipe::Horizontal => return Some((new_index, Direction::East)),
            Pipe::NorthEast => return Some((new_index, Direction::East)),
            Pipe::NorthWest => return Some((new_index, Direction::West)),
            _ => return  None,
           
        }
    }

    fn get_west(&self, index:u32) -> Option<(u32, Direction)> {
        todo!()
    }

    fn get_south(&self, index:u32) -> Option<(u32, Direction)> {
        todo!()
    }
}

fn find_start_index(map: &Vec<char>) -> Option<u32> {
    for (index, pipe_char) in map.iter().enumerate() {
        if pipe_char == &'S' {
            return Some(index as u32);
        }
    }
    None
}


