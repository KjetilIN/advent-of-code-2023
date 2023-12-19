struct MazeMap {
    map: Vec<char>,
    start_index: u32,
    map_width: u32,
    map_height: u32,
}

trait MazeMapMethods {
    fn from_file(content: &String) -> Result<Self, String>
    where
        Self: Sized;
    fn count_half_circle(&self) -> u32;
    
    fn find_starting_pipe(&self) -> u32;
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

    fn find_starting_pipe(&self) -> u32 {
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


