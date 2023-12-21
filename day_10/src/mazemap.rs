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
    fn count_half_circle(&self) -> Result<u32, String> ;
    fn find_starting_pipe(&self) -> Option<(u32, Direction)>;
    fn get_north(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_east(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_west(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_south(&self, index:u32) -> Option<(u32, Direction)>;
}

impl MazeMapMethods for MazeMap {
    /// Creates a new mazemap from the given content of a file 
    /// - Takes a reference of the content string. 
    /// 
    /// Returns a new instance of self 
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

    /// Function that counts how many pipes away the farthest point is from the start
    /// - Counts the total amount of pipes used to create the whole circle and returns the count divided by two
    /// - Iterates through the pipe loop
    /// 
    /// Returns the count or the error
    fn count_half_circle(&self) -> Result<u32, String> {
        let mut counter: u32 = 1;

        let (start, direction) = match self.find_starting_pipe(){
            None => {
                eprintln!("ERROR: did not find starting pipe");
                return Err("No pipe connected to start".to_string());
            }
            Some((index, dir)) => (index, dir)
        };

        let mut current_dir = direction;
        let mut current_index = start;

        while current_index != self.start_index{
            counter+= 1;
            match current_dir{
                Direction::North => {
                    let (new_index, new_dir) = match self.get_north(current_index){
                        Some((ind, dir)) => (ind, dir),
                        None => {
                            eprintln!("ERROR: did not find north pipe");
                            return Err("Not valid north".to_string());
                        },
                    };

                    current_dir = new_dir;
                    current_index = new_index;
                },
                Direction::South => {
                    let (new_index, new_dir) = match self.get_south(current_index){
                        Some((ind, dir)) => (ind, dir),
                        None => {
                            eprintln!("ERROR: did not find south pipe");
                            return Err("Not valid south".to_string());
                        },
                    };

                    current_dir = new_dir;
                    current_index = new_index;
                },
                Direction::West => {
                    let (new_index, new_dir) = match self.get_west(current_index){
                        Some((ind, dir)) => (ind, dir),
                        None => {
                            eprintln!("ERROR: did not find west pipe");
                            return Err("Not valid west".to_string());
                        },
                    };

                    current_dir = new_dir;
                    current_index = new_index;
                },
                Direction::East => {
                    let (new_index, new_dir) = match self.get_east(current_index){
                        Some((ind, dir)) => (ind, dir),
                        None => {
                            eprintln!("ERROR: did not find east pipe");
                            return Err("Not valid east".to_string());
                        },
                    };

                    current_dir = new_dir;
                    current_index = new_index;
                },
                Direction::None => {
                    eprintln!("Found pipe none");
                    return Err("Not a pipe".to_string());
                },
            }
        }


        Ok(counter/2)
    }

    /// Finds the first pipe to follow.
    /// Looks clockwise starting with North
    /// Returns None if no valid pipe is connected to the starting index
    /// Returns the index of the next pipe from the start with the direction to go
    fn find_starting_pipe(&self) -> Option<(u32, Direction)>{
        match self.get_north(self.start_index){
            Some((index,dir)) => {
                if dir != Direction::None{
                    return Some((index, dir));
                }
            },
            None => {},
        }

        match self.get_west(self.start_index) {
            Some((index,dir)) => {
                if dir != Direction::None{
                    return Some((index, dir));
                }
            },
            None => {},
        }

        match self.get_east(self.start_index) {
            Some((index,dir)) => {
                if dir != Direction::None{
                    return Some((index, dir));
                }
            },
            None => {},
        }

        match self.get_west(self.start_index) {
            Some((index,dir)) => {
                if dir != Direction::None{
                    return Some((index, dir));
                }
            },
            None => {},
        }

        None


    }

    /// Given the index, get the north pipe
    /// Returns None, if pipe is invalid to be a connected pipe
    /// Else returns the north index and the direction of the north pipe 
    fn get_north(&self, index:u32) -> Option<(u32, Direction)> {
        // Check that top boundary is okay
        if index - self.map_width <= 0 {
            return None;
        }

        let new_index = index - self.map_width;
        let char = self.map[new_index as usize];
        let pipe: Pipe = get_pipe(char);

        match pipe{
            Pipe::Vertical => return Some((new_index, Direction::North)),
            Pipe::SouthEast => return Some((new_index, Direction::East)),
            Pipe::SouthWest => return Some((new_index, Direction::West)),
            Pipe::Start => return Some((new_index, Direction::None)),
            _ => return  None,
           
        }
    }

    /// Given the index, get the east pipe
    /// Returns None, if pipe is invalid to be a connected pipe
    /// Else returns the east index and the direction of the east pipe 
    fn get_east(&self, index:u32) -> Option<(u32, Direction)> {
        // Check that top boundary is okay
        if (index % self.map_width) + 1 > self.map_width{
            return None;
        }

        let new_index = index + 1;
        let char = self.map[new_index as usize];
        let pipe: Pipe = get_pipe(char);

        match pipe{
            Pipe::Horizontal => return Some((new_index, Direction::East)),
            Pipe::SouthWest => return Some((new_index, Direction::South)),
            Pipe::NorthWest => return Some((new_index, Direction::North)),
            Pipe::Start => return Some((new_index, Direction::None)),
            _ => return  None,
           
        }
    }

    /// Given the index, get the west pipe
    /// Returns None, if pipe is invalid to be a connected pipe
    /// Else returns the west index and the direction of the west pipe 
    fn get_west(&self, index:u32) -> Option<(u32, Direction)> {
        // Check that top boundary is okay
        if  index % self.map_width == 0{
            return None;
        }

        let new_index = index - 1;
        let char = self.map[new_index as usize];
        let pipe: Pipe = get_pipe(char);

        match pipe{
            Pipe::Horizontal => return Some((new_index, Direction::West)),
            Pipe::SouthEast => return Some((new_index, Direction::South)),
            Pipe::NorthEast => return Some((new_index, Direction::North)),
            Pipe::Start => return Some((new_index, Direction::None)),
            _ => return  None,
           
        }
    }

    /// Given the index, get the south pipe
    /// Returns None, if pipe is invalid to be a connected pipe
    /// Else returns the south index and the direction of the south pipe 
    fn get_south(&self, index:u32) -> Option<(u32, Direction)> {
        // Check that top boundary is okay
        if index + self.map_width > self.map_width*self.map_height {
            return None;
        }

        let new_index = index + self.map_height;
        let char = self.map[new_index as usize];
        let pipe: Pipe = get_pipe(char);

        match pipe{
            Pipe::Vertical => return Some((new_index, Direction::South)),
            Pipe::NorthEast => return Some((new_index, Direction::East)),
            Pipe::NorthWest => return Some((new_index, Direction::West)),
            Pipe::Start => return Some((new_index, Direction::None)),
            _ => return  None,
        }
    }
}

/// Given a vector of chars, finds the index of 'S' => starting pipe
/// Returns none if there are no pipe that fit 
fn find_start_index(map: &Vec<char>) -> Option<u32> {
    for (index, pipe_char) in map.iter().enumerate() {
        if pipe_char == &'S' {
            return Some(index as u32);
        }
    }
    None
}