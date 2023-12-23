#[cfg(test)]
mod tests{
    use crate::galaxy_map::{GalaxyMap, GalaxyMethods};


    #[test]
    fn test_galaxy_count(){
        let content = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let galaxy = match GalaxyMap::from_file((&content).to_string()){
            Ok(g) => g,
            Err(_) => panic!("Error during creating the galaxy map"),
        };

        assert!(galaxy.galaxy_indexes.len() == 9);

        for index in galaxy.galaxy_indexes{
            assert_eq!(galaxy.map.get(index as usize).unwrap(), &'#')
        }
    }

    #[test]
    fn test_expansion_of_galaxy(){
        let content = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let galaxy = match GalaxyMap::from_file((&content).to_string()){
            Ok(g) => g,
            Err(_) => panic!("Error during creating the galaxy map"),
        };

        let result_map = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";


        let result_map_chars: Vec<_> = result_map.chars().filter(|&c| c != '\n').collect();
        
        assert_eq!(result_map_chars.len(), galaxy.map.len());


    }

}
