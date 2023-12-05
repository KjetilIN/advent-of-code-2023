# Day 3 - Description

Given a engine schema:
```txt
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```
Any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Link to complete description: https://adventofcode.com/2023/day/3

## Input Description

1. Find the sum of all part numbers for a given engine schema
    - A part is a char that is not '.' or a number
    - The sum of a part is the sum of all numbers around the given part
2. Find the sum of all gear ratio
    - A gear is a part with '*' char 
    - A valid gear has to have two numbers around it
    - Gear Ratio is the two numbers around a gear multiplied

## Approach/Algorithm

This was a really cool task! Here is my approach:

Created a `EngineSchema` struct that has a vector of each line of the map, and length of a row and col:
```rust
/**
    The struct that represent an engine schema 
    - Has the vectors of all the rows
    - Has the length of columns and rows
 */
pub struct EngineSchema<'a>{
    pub rows: &'a Vec<String>,
    pub col_length: u32,
    pub row_length: u32
}
```

Then I implemented the methods for finding the different parts of the challenge. Here is the algorithm:

1. Iterate over each char for each row in the map
2. If the char is a part we get the part sum for the individual part by:
    a. Check in each direction for a numeric char
    b. Get the number whole numeric number 
    c. Sum up each of the numbers to get the part sum for the individual part
3. Add the part sum to the total part sum and return it!


For part 2, we do some slight changes in the code: 

1. Iterate over each char for each row in the map
2. If the char is a **gear** we get the gear ratio for the individual gear by:
    a. Check in each direction for a numeric char
    b. Get the number whole numeric number 
    c. **Check if we have found two numbers:**
        i. If we did, multiply them and return the result
        ii. Else we know that it is **not a valid gear** and we return 0
3. Add the gear ratio to the total gear ratio and return it!


## Learnings

- Again, TDD lead me to the answer fast, and found out small issues and edge-cases fast
- Did draw a sketch on a piece of paper, which was very helpful for visualizing 
- Used `Option` which was very useful for some functions

## Code Snippets

There is a lot of code, but I think it is very readable (which is my goal).

However, there are one cool function:


### Retrieving a number

This function is for getting a number given an index and a row.
The index is a number where a digit was located, and then the function finds the whole number.

This is by using a while loop like this:
```rust
//We add each numeric char that is to the left
let mut right_index = number_index + 1;
let mut right_number = Vec::new();

while char_vector[right_index].is_numeric() && char_vector.len() > right_index  {
    right_number.push(char_vector[right_index]);
    right_index += 1;
    if right_index == char_vector.len(){
        break;
    }
    
};
```

Then after we have checked both sides, we format a final string before we parse it:

```rust 
let result_string = format!(
        "{}{}{}",
        left_number.iter().rev().collect::<String>(),
        central_char,
        right_number.iter().collect::<String>()
    );
```

If you want to see the whole function, look for the following function signature:
```rust
pub fn get_number(row: &String, number_index:usize) -> u32
```