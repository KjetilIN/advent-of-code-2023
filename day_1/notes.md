# Day 1 - Description

Day one was the goal to try to get a two digit number for each line, where each line is fuzzed 

## Input Description

A list of fuzzed input in `input.txt`


## Output Description

The sum of all the two digit numbers in the file. 

## Approach/Algorithm


### Solution part 1 
The first part was very simple:
    1. Iterate through each char
    2. Check if the char is numeric, else continue
    3. If it is numeric and the first number is not been set 
        - Set the first number
        - Always set the last number
    4. Use this formula for calculating the number:
        - `first_digit*10 + last_digit`

### Solution part 2 (bad)
The second part had included numbers as plain text. I decided to use a hashmap to turn "one" into 1, etc.
Then after we have gone through the first algorithm we check for plain text numbers:

1. For each key value pair in the hashmap. 
2. Find all instances of the key in the string, and for each of them
    - Here I used `match_indices` to get a list of tuples of indexes and values
    - Then I used `map` to only get the indexes
3. Check if the set index of first and last number is lower or higher
    - If true, set new high or new number to the value of the map
4. In the end the two numbers should have been replaced


This solution is bad, because it iterates over the string twice and has a nested for loop. 
Will find a better solution for this, later. (When I do not have exams hehe )

## Learnings

- Used testing to find out issues with my code => Should do that in the future as well
- `match_indices` was handy
- Read the description multiple times, because you might miss small details (As I did the first time)

## Code Snippets


Here is the solution of Part 1:

```rust 
pub fn number_from_string(input: &str) -> u32{
    // Save the result in a variable 
    let mut first_digit:u32 = 0;
    let mut last_digit:u32 = 0;

    // Iterate over the chars 
    for ch in input.chars(){
        // Check if the char is numeric
        if ch.is_numeric(){
            // Set the first digit if it has not been set
            if first_digit == 0{
                first_digit = ch.to_digit(10).unwrap()
            }
            // Reset the last number 
            last_digit = ch.to_digit(10).unwrap();
        }
    }
    // Return the result as a two digit number
    first_digit*10 + last_digit
}
```


This is the part where I iterate over the `HashMap<&str,i32>` for the conversion of string numbers:

```rust
// Iterate over all possible numbers as strings in the hashmap
for (number_string, number) in value_map.into_iter(){
    // Get all the indexes for all matches 
    for new_index in input.match_indices(number_string).map(|(new_index,_)| new_index){

        // If the index is lower, we found a new "left most" number
        // If the index is higher, we found a new "right most" number
        if new_index <= index_first {
            index_first = new_index; 
            first_digit = (*number as u32).clone();
        }else if new_index >= index_last {
            index_last = new_index;
            last_digit = (*number as u32).clone();
        };
    };
}
```

