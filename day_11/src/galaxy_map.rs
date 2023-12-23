use crate::expand::{expand_horizontal, expand_vertically, get_line_length};

pub struct GalaxyMap{
    pub map: Vec<char>,
    pub galaxy_indexes: Vec<u32>,
    pub width: u32,
}

pub trait GalaxyMethods {
    fn from_file(content: String) -> Result<Self, String> where Self: Sized;
}

impl GalaxyMethods for GalaxyMap {
    fn from_file(content: String) -> Result<Self, String> where Self: Sized {
        let mut expanded_map = content;

        // Expand the map horizontally 
        expanded_map = match expand_horizontal(&expanded_map){
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        
        // Expand the map vertically 
        expanded_map = match expand_vertically(&expanded_map){
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        let width = match get_line_length(&expanded_map){
            Ok(val) => val as u32,
            Err(err) => return Err(err),
        };
        let map:Vec<char> = expanded_map.chars().filter(|&c| c != '\n').collect();
        let mut galaxy_indexes: Vec<u32> = Vec::new();

        for (index, char) in map.iter().enumerate(){
            if char == &'#'{
                galaxy_indexes.push(index as u32)
            }
        }
        

        Ok(Self{
            map,
            galaxy_indexes,
            width
        })

    }
}