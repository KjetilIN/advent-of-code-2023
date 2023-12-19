# Day 2 - Description

Given a list of games: 
1. Find the game ids of the games that are possible given that there are x amount of cubes for each color
2. Find the fewest numbers of cubes for each color and multiply them

Link to complete description: https://adventofcode.com/2023/day/2 

## Input Description

Given a list of games. A game looks like this;

`Game 7: 4 red, 6 green, 1 blue; 3 blue, 9 green, 5 red; 5 blue, 5 green, 4 red; 6 green, 5 blue, 5 red; 13 red; 4 red, 2 blue, 9 green`

Each game has an ID. Then a set of cubes is shown. Each time we show a set of cubes we use semicolon to tell the new set of cubes is being shown.  

## Approach/Algorithm

I decided to turn each string into a Vector. This was a smart choice because I could then iterate over each item in the char. I made the Vector by slitting a formatted string on spaces. (I first had to format to remove colons and make semicolons have space on the left side. Used replace for that).

For both parts I made a vector and then iterated through each item. 

### Part 1 

1. If we find `Game` we know the next element is the game ID, so we store that
2. If we find one of the colors - `red` `green` `blue` - we know the element before is the amount so we safe that
    - Then check if the number of cubes for that given color goes over the maximum, then we return `0` as game ID (The game was not possible)
3. If we find `;` we reset the values for each color - we have not gone over the max and it is a knew "show" of cubes coming up
4. If the whole vector has been iterated through, we know that the game is valid and we return the game ID

### Part 2 

We the almost the same as part 1, but this time we need to find the minimum. We store the minimum amount of cubes for each color in a variable. Then: 

If we find a color for a show, check if the amount is bigger. If it is, set it as the new minimum of the given color. 

At the end we will have tree variables, where they are the minimum amount of cubes for each color, so we multiply and return them. 

## Learnings

- Thinking and try to go for the least amount of iterations helps for design. I found a much better solution with the goal of iterating over a game once. 


## Code Snippets

For creating the vector:

```rust
let formatted_game = input.trim()
                          .replace(",", "")
                          .replace(";", " ;");

// Turn the line into a vector 
let word_list: Vec<_> = formatted_game.split(" ").collect();
```

When we find a color part 1:

```rust
&"blue" => {
    // Get the amount of the color blue
    let amount: u32 =  get_amount(&word_list, index);

    // Add it to the total
    blue += amount;

    // Check if game is possible 
    if blue > MAX_POSSIBLE_BLUES{
        return 0 
    }

},
```

When we find a color in part 2:

```rust
&"green" => {
    // Get the amount for the color
    let amount: u32 =  get_amount(&word_list, index);

    // If the amount is more
    if amount > green{
        green = amount;
    }

},
```

