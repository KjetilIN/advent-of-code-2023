pub mod main;

#[cfg(test)]
mod tests {
    use crate::main::{check_game_possible, minimum_cubes_powered};

    // TEST FOR PART 1 
    #[test]
    fn test_game_10_non_valid() {
        // Test 
        let test_string = "Game 10: 6 red, 11 blue, 12 green; 1 blue, 2 red, 3 green; 14 green, 6 red, 7 blue; 6 red, 10 blue, 9 green; 6 blue, 2 red";

        let result: u32 = check_game_possible(test_string);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_game_1_non_valid() {
        // Test 
        let test_string = "Game 1: 1 green, 2 blue; 13 red, 2 blue, 3 green; 4 green, 14 red";

        let result: u32 = check_game_possible(test_string);
        assert_eq!(result, 0);
    }


    #[test]
    fn test_game_3_non_valid() {
        // Test 
        let test_string = "Game 3: 1 blue, 12 green, 2 red; 9 red, 16 green; 1 red, 10 green, 1 blue; 1 red, 14 green";

        let result: u32 = check_game_possible(test_string);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_game_example_valid() {
        // Test 
        let test_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let result: u32 = check_game_possible(test_string);
        assert_eq!(result, 1);
    }

    // TEST FOR PART 2

    #[test]
    fn test_cube_example_1() {
        // Test 
        let test_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let result: u32 = minimum_cubes_powered(test_string);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_cube_example_2() {
        // Test 
        let test_string = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

        let result: u32 = minimum_cubes_powered(test_string);
        assert_eq!(result, 12);
    }


    #[test]
    fn test_cube_example_3() {
        // Test 
        let test_string = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        let result: u32 = minimum_cubes_powered(test_string);
        assert_eq!(result, 1560);
    }

    #[test]
    fn test_cube_example_4() {
        // Test 
        let test_string = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";

        let result: u32 = minimum_cubes_powered(test_string);
        assert_eq!(result, 630);
    }
}