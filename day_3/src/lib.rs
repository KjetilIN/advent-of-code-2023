pub mod main;

#[cfg(test)]
mod tests {
    use crate::main::get_number;


    // Tests for getting the a string for a given number 
    #[test]
    fn test_get_number_valid_left() {
        let test_string = String::from("..3325665.");
        let test_index = 8;

        let number = get_number(&test_string, test_index);

        assert_eq!(number, 3325665);
    }

    #[test]
    fn test_get_number_valid_right() {
        let test_string = String::from("...345678.");
        let test_index = 3;

        let number = get_number(&test_string, test_index);

        assert_eq!(number, 345678);
    }

    #[test]
    fn test_get_number_valid_both() {
        let test_string = String::from("...345678346....");
        let test_index = 7;

        let number = get_number(&test_string, test_index);

        assert_eq!(number, 345678346);
    }


    #[test]
    fn test_get_number_invalid_no_panic() {
        let test_string = String::from(".............");
        let test_index = 7;

        let number = get_number(&test_string, test_index);

        assert_eq!(number, 0);
    }
}