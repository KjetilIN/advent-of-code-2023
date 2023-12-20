#[cfg(test)]
mod test{
    use crate::{mazemap::{MazeMap, MazeMapMethods}, direction::Direction};


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
                assert!(b == Direction::North)
            }
        }
    }



}