pub mod main;

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::main::{self, number_from_string_with_number_strings};

    #[test]
    fn find_number_92() {
        let test_string = "nineasfdsgfd3dfsseven2"; // => 92

        let mut value_map: HashMap<&str, i32> = HashMap::new();
        value_map.insert("one", 1);
        value_map.insert("two", 2);
        value_map.insert("three", 3);
        value_map.insert("four", 4);
        value_map.insert("five", 5);
        value_map.insert("six", 6);
        value_map.insert("seven", 7);
        value_map.insert("eight", 8);
        value_map.insert("nine", 9);

        // 92
        let numb = number_from_string_with_number_strings(test_string, &value_map);

        assert_eq!(numb, 92);


    }

    #[test]
    fn find_number_98() {

        let mut value_map: HashMap<&str, i32> = HashMap::new();
        value_map.insert("one", 1);
        value_map.insert("two", 2);
        value_map.insert("three", 3);
        value_map.insert("four", 4);
        value_map.insert("five", 5);
        value_map.insert("six", 6);
        value_map.insert("seven", 7);
        value_map.insert("eight", 8);
        value_map.insert("nine", 9);

        // 98
        let test_string = "9seven8";

        let numb = number_from_string_with_number_strings(test_string, &value_map);

        assert_eq!(numb, 98);


    }


    #[test]
    fn find_number_88() {

        let mut value_map: HashMap<&str, i32> = HashMap::new();
        value_map.insert("one", 1);
        value_map.insert("two", 2);
        value_map.insert("three", 3);
        value_map.insert("four", 4);
        value_map.insert("five", 5);
        value_map.insert("six", 6);
        value_map.insert("seven", 7);
        value_map.insert("eight", 8);
        value_map.insert("nine", 9);

        // 88
        let test_string = "eighteight9dnvcqznjvfpreight";

        let numb = number_from_string_with_number_strings(test_string, &value_map);

        assert_eq!(numb, 88);
    }

}