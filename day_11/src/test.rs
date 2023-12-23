#[cfg(test)]
mod tests{
    use crate::{galaxy_map::{GalaxyMap, GalaxyMethods}, expand};


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

        let galaxy = match GalaxyMap::from_file((&content).to_string(), 2){
            Ok(g) => g,
            Err(_) => panic!("Error during creating the galaxy map"),
        };

        assert_eq!(galaxy.galaxy_indexes.len(),9);

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

        let galaxy = match GalaxyMap::from_file((&content).to_string(), 2){
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

    #[test]
    fn test_shortest_path_1(){
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

        let galaxy = match GalaxyMap::from_file((&content).to_string(), 2){
            Ok(g) => g,
            Err(_) => panic!("Error during creating the galaxy map"),
        };

        
        // Test from 5th to 9th => 9 steps
        let start_index = galaxy.galaxy_indexes.get(4).unwrap();
        let end_index = galaxy.galaxy_indexes.get(8).unwrap();

        match galaxy.find_shortest_distance(*start_index, *end_index){
            None => panic!("No path found"),
            Some(distance) => assert_eq!(distance, 9)
        }

    }

    #[test]
    fn test_shortest_path_2(){
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

        let galaxy = match GalaxyMap::from_file((&content).to_string(), 2){
            Ok(g) => g,
            Err(_) => panic!("Error during creating the galaxy map"),
        };

        
        // Test from 1th to 7th => 15 steps
        let start_index = galaxy.galaxy_indexes.get(0).unwrap();
        let end_index = galaxy.galaxy_indexes.get(6).unwrap();

        match galaxy.find_shortest_distance(*start_index, *end_index){
            None => panic!("No path found"),
            Some(distance) => assert_eq!(distance, 15)
        }

    }

    #[test]
    fn test_shortest_path_3(){
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

        let galaxy = match GalaxyMap::from_file((&content).to_string(), 2){
            Ok(g) => g,
            Err(_) => panic!("Error during creating the galaxy map"),
        };

        
        // Test from 3th to 6th => 17 steps
        let start_index = galaxy.galaxy_indexes.get(2).unwrap();
        let end_index = galaxy.galaxy_indexes.get(5).unwrap();

        match galaxy.find_shortest_distance(*start_index, *end_index){
            None => panic!("No path found"),
            Some(distance) => assert_eq!(distance, 17)
        }

    }

    #[test]
    fn test_shortest_path_4(){
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

        let galaxy = match GalaxyMap::from_file((&content).to_string(), 2){
            Ok(g) => g,
            Err(_) => panic!("Error during creating the galaxy map"),
        };

        
        // Test from 8th to 9th => 5 steps
        let start_index = galaxy.galaxy_indexes.get(7).unwrap();
        let end_index = galaxy.galaxy_indexes.get(8).unwrap();

        match galaxy.find_shortest_distance(*start_index, *end_index){
            None => panic!("No path found"),
            Some(distance) => assert_eq!(distance, 5)
        }

    }

    #[test]
    fn test_sum_shortest_distances(){
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

        let galaxy = match GalaxyMap::from_file((&content).to_string(), 2){
            Ok(g) => g,
            Err(_) => panic!("Error during creating the galaxy map"),
        };

        let sum = match galaxy.find_sum_of_shortest_distances()  {
            Ok(val) => val,
            Err(_) => panic!("Was not able to calculate the sum"),
        };

        
        assert_eq!(sum, 374)

        
    

    }

}
