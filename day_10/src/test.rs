#[cfg(test)]
mod test{
    use crate::{mazemap::{MazeMap, MazeMapMethods}, direction::Direction, shoe_lace::find_area_from_points};


    #[test]
    fn test_constructor_mazemap(){
        let content = String::from(".....
.S-7.
.|.|.
.L-J.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        assert_eq!(map.map_width, 5);
        assert_eq!(map.map_height, 5);

        assert_eq!(map.start_index, 6);
        assert_eq!(map.map[6], 'S');
    }

    #[test]
    fn test_get_north(){
        let content = String::from(".....
.S-7.
.|.|.
.L-J.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()

            }
        };

        match map.get_north(6){
            None => assert!(true),
            Some(_) => panic!()
        }

        match map.get_north(11) {
            None => panic!(),
            Some((new, b)) =>{
                assert!(new == 6 );
                assert!(b == Direction::None)
            }
        }

        // North of J => | 
        match map.get_north(18) {
            None => panic!(),
            Some((new, b)) =>{
                assert!(new == 13 );
                assert!(map.map[13] == '|');
                assert!(b == Direction::North)
            }
        }

        // North of | => 7
        match map.get_north(13) {
            None => panic!(),
            Some((new, b)) =>{
                assert!(new == 8 );
                assert!(map.map[8] == '7');
                assert!(b == Direction::West)
            }
        }
        match map.get_north(map.start_index) {
            None => {
                assert!(true);
            },
            Some((_, _)) =>{
                panic!()
            }
        }
    }

    #[test]
    fn test_get_east(){
        let content = String::from(".....
.S-7.
.|.|.
.L-J.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()

            }
        };

        // East of S
        match map.get_east(6){
            None => panic!(),
            Some((index, direction)) => {
                assert!(index == 7);
                assert!(direction == Direction::East);
                assert!(map.map[7] == '-');
            }
        }

        // East of 7 => none
        match map.get_east(8){
            None => {
                assert!(true);
            },
            Some((_, _)) => {
                panic!()
            }
        }

        // East of - => 7
        match map.get_east(7){
            None => {
                panic!()
            },
            Some((index, dir)) => {
                assert!(index == 8);
                assert!(dir == Direction::South);
                assert!(map.map[8] == '7');
            }
        }


        // East of - => J
        match map.get_east(17){
            None => {
                panic!()
            },
            Some((index, dir)) => {
                assert!(index == 18);
                assert!(dir == Direction::North);
                assert!(map.map[18] == 'J');
            }
        }
    }


    #[test]
    fn test_get_west(){
        let content = String::from(".....
.S-7.
.|.|.
.L-J.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()

            }
        };

        // West of 7 => - 
        match map.get_west(8){
            None => panic!(),
            Some((index, direction)) => {
                assert!(index == 7);
                assert!(direction == Direction::West);
                assert!(map.map[7] == '-');
            }
        }

        // West of - => L
        match map.get_west(17){
            None => panic!(),
            Some((index, direction)) => {
                assert!(index == 16);
                assert!(direction == Direction::North);
                assert!(map.map[16] == 'L');
            }
        }

        // Check left boundary 
        match map.get_west(5){
            None => assert!(true),
            Some((_, _)) => {
                panic!()
            }
        }
    }


    #[test]
    fn test_get_south(){
        let content = String::from(".....
.S-7.
.|.|.
.L-J.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()

            }
        };

        // South of S => |
        match map.get_south(6){
            None => panic!(),
            Some((index, direction)) => {
                assert!(index == 11);
                assert!(direction == Direction::South);
                assert!(map.map[11] == '|');
            }
        }

        // South of | => L
        match map.get_south(11){
            None => panic!(),
            Some((index, direction)) => {
                assert!(index == 16);
                assert!(direction == Direction::East);
                assert!(map.map[16] == 'L');
            }
        }

        // South of | => J
        match map.get_south(13){
            None => panic!(),
            Some((index, direction)) => {
                assert!(index == 18);
                assert!(direction == Direction::West);
                assert!(map.map[18] == 'J');
            }
        }
    }


    #[test]
    fn test_find_starting_pipe_east(){
        let content = String::from(".....
.S-7.
.|.|.
.L-J.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        let _ =  match map.find_starting_pipe(){
            None => panic!(),
            Some((index, dir)) =>{
                assert!(index == 7);
                assert!(map.map[7] == '-');
                assert!(dir == Direction::East);
            }
        };
    
    }

    #[test]
    fn test_find_starting_pipe_none(){
        let content = String::from(".....
.S.7.
...|.
.L-J.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        let _ =  match map.find_starting_pipe(){
            None => assert!(true),
            Some((_, _)) =>{
                panic!()
            }
        };
    
    }


    #[test]
    fn test_find_starting_pipe_north(){
        let content = String::from(".....
.J-7.
.|.|.
.L-S.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        let _ =  match map.find_starting_pipe(){
            None => panic!(),
            Some((index, dir)) =>{
                assert!(index == 13);
                assert!(map.map[13] == '|');
                assert!(dir == Direction::North);
            }
        };
    
    }

    #[test]
    fn test_find_starting_pipe_west(){
        let content = String::from(".....
.J-7.
.|...
.L-S.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        let _ =  match map.find_starting_pipe(){
            None => panic!(),
            Some((index, dir)) =>{
                assert!(index == 17);
                assert!(map.map[17] == '-');
                assert!(dir == Direction::West);
            }
        };
    
    }


    #[test]
    fn test_counter(){
        let content = String::from(".....
.S-7.
.|.|.
.L-J.
.....");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        let count = match map.count_half_circle(){
            Ok(val) => val,
            Err(_) => panic!(),
        };

        assert!(count == 4);
    }

    #[test]
    fn test_find_area(){
        let content = String::from("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

       let points = match map.collect_points(){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        let area = find_area_from_points(&points);
        println!("Area: {area}");

        assert!(area == 26);
    }

    #[test]
    fn test_count_pipes(){
        let content = String::from("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

       let pipes = match map.collect_points(){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        assert!(pipes.len() == 46);
    }

    #[test]
    fn test_count_enclosed_tiles(){
        let content = String::from("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");

        let map = match MazeMap::from_file(&content){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

       let tiles = match map.count_enclosed_tiles(){
            Ok(val) => val,
            Err(_) =>{
                panic!()
            }
        };

        assert!(tiles == 4);
    }




}