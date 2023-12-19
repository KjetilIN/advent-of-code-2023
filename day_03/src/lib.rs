pub mod main;

#[cfg(test)]
mod tests {
    use crate::main::get_number;
    use crate::main::EngineSchema;


    // Tests for getting the a string for a given number 
    #[test]
    fn test_get_number_valid_left() {
        let test_string = String::from("..3325665.......88765");
        let test_index = 8;

        let number = get_number(&test_string, test_index);

        assert_eq!(number, 3325665);
    }

    #[test]
    fn test_get_number_valid_right() {
        let test_string = String::from("...345678.....665544");
        let test_index = 3;

        let number = get_number(&test_string, test_index);

        assert_eq!(number, 345678);
    }

    #[test]
    fn test_get_number_valid_both() {
        let test_string = String::from("345...345678346....335..");
        let test_index = 10;

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

    // Tests with the demo map
    // The demo map:
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..
    #[test]
    fn test_constructor() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        assert_eq!(engine.col_length, 10);
        assert_eq!(engine.row_length, 10);
    }


    #[test]
    fn test_get_char_from_vector_valid() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        match engine.get_char_from_vector(0, 2){
            Some(char) => assert_eq!(char, '7'),
            None => assert!(false, "ERROR: got none for '7'")
        };

        
        match engine.get_char_from_vector(3, 6){
            Some(char) => assert_eq!(char, '#'),
            None => assert!(false, "ERROR: got none for '7'")
        };

        // Test where we expect NONE
        match engine.get_char_from_vector(10, 2){
            Some(_) => assert!(false, "ERROR: got none for '7'"),
            None => assert!(true)
        };

    }


    #[test]
    fn test_is_part_number_valid() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        assert_eq!(engine.is_part_number(0, 0), true);
        assert_eq!(engine.is_part_number(2, 2), true);
    }

    #[test]
    fn test_is_part_number_invalid() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // '.' should be false
        assert_eq!(engine.is_part_number(1, 5), false); 

        // '#' should be false
        assert_eq!(engine.is_part_number(1, 4), false);

        // Should return false because out of bounce 
        assert_eq!(engine.is_part_number(10, 2), false);
    }


    #[test]
    fn test_get_part_sum_1() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the part sum for the '*' on the second row
        // It has one number top left and one below
        // Sum should be 502
        assert_eq!(engine.get_part_sum(1, 3), 502);
        
    }

    #[test]
    fn test_get_part_sum_2() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the part sum for the '*' on the fifth row
        // It has one number to the left 
        // Sum should be 617

        assert_eq!(engine.get_part_sum(4, 3), 617);
    }

    #[test]
    fn test_get_part_sum_3() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the part sum for the '*' on the fourth row
        // It has one number top 
        // Sum should be 633
        
        assert_eq!(engine.get_part_sum(3, 6), 633);
    }

    #[test]
    fn test_get_part_sum_4() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the part sum for the '*' on the ninth row
        // It has one number top right and one below
        // Sum should be 1352
        
        assert_eq!(engine.get_part_sum(8, 5), 1353);
    }


    #[test]
    fn test_get_part_sum_5() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the part sum for the '+' on the sixth row
        // It has one number bottom left
        // Sum should be 592
        
        assert_eq!(engine.get_part_sum(5, 5), 592);
    }

    #[test]
    fn test_get_total_part_sum() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the part total part sum of the map, which should be 4361   
        assert_eq!(engine.get_total_part_sum(), 4361);
    }


    #[test]
    fn test_get_gear_ration_invalid() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the gear for the '*' on the fourth row
        // It has one number top meaning it should be invalid
        assert_eq!(engine.get_gear_ratio(3, 6), 0);
    }

    #[test]
    fn test_get_gear_ration_valid() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the gear ratio for the '*' on the ninth row
        // It has one number top right and one below 
        // Gear ratio should be (755 * 598 = 451490)
        
        assert_eq!(engine.get_gear_ratio(8, 5), 451490);
    }


    #[test]
    fn test_get_total_gear_ratio() {
        let schema = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let mut parts_vector = Vec::<String>::new();
        for line in schema.lines(){
            parts_vector.push(line.to_string().clone());
        }


        let engine = EngineSchema::new(&parts_vector);

        // We are testing the part total part sum of the map, which should be 467835   
        assert_eq!(engine.get_total_gear_ratio(), 467835);
    }


}