pub mod main;

#[cfg(test)]
mod tests {
    use crate::main::check_game_possible;

    #[test]
    fn test_game_10() {
        // Test 
        let test_string = "Game 10: 6 red, 11 blue, 12 green; 1 blue, 2 red, 3 green; 14 green, 6 red, 7 blue; 6 red, 10 blue, 9 green; 6 blue, 2 red";

        let result = check_game_possible(test_string);
        assert_eq!(result, 4);
    }
}