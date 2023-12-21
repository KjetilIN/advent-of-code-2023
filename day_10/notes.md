# Day 10 - Description

This task gives you a schema of pipes. For example:

```text
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
```

`S` represents a starting pipe,
`.` represents ground,
`-` represents a horizontal pipe,
`|` represents a vertical pipe, and `7, F, J, L` represent 90 degree corner pipes. 

There will always be a single loop of pipes. There could also be more pipes in the schema, then the pipes that are needed for the loop.

The task are:
1. Count how many steps the pipe that is the furthest away is. 
    1. For example the pipe `J` is 4 steps away
    ```text
    .....
    .S-7.
    .|.|.
    .L-J.
    .....
    ```
2. Count how many pipes/tiles are enclosed in the loop



Link to complete description: https://adventofcode.com/2023/day/10

## Approach/Algorithm

I created a `MazeMap` struct and implemented the following trait:
```rust
#[derive(Debug)]
pub struct MazeMap {
    pub map: Vec<char>,
    pub start_index: u32,
    pub map_width: u32,
    pub map_height: u32,
}

pub trait MazeMapMethods {
    fn from_file(content: &String) -> Result<Self, String>
    where
        Self: Sized;
    fn count_half_circle(&self) -> Result<u32, String> ;
    fn find_starting_pipe(&self) -> Option<(u32, Direction)>;
    fn get_north(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_east(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_west(&self, index:u32) -> Option<(u32, Direction)>;
    fn get_south(&self, index:u32) -> Option<(u32, Direction)>;
}
```

To follow a pipe we need to know what direction the flow is going in. This is the `Direction` of the pipe:

```rust
pub enum Direction{
    North,
    South,
    West,
    East,
    None,
}
```

Each pipe is also an enum:

```rust
#[derive(Debug)]
pub enum Pipe{
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
    Undefined
}


pub fn get_pipe(ch: char)-> Pipe{
    return match &ch {
        '-' => Pipe::Horizontal,
        '|' => Pipe::Vertical,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        'S' => Pipe::Start,
        _ => Pipe::Undefined,
    }
}
```

For part 1.
1. Find the index of the start pipe (constructor does this)
2. Find the next pipe from the starting pipe and set the direction of the flow based on the type of pipe
3. Loop until we are at the start pipe. We increment the step for each pipe we "move through":
    1. Each pipe we check the current direction. Based on the direction we get the north, south, east or west pipe. From the method we get the new index and new direction of the flow. We update the current index and direction
4. Return count of steps divided by 2. This is because count of steps will be how many steps the entire loop is. The farthest away pipe is half the amount of the steps. 


Part 2:

> Unsolved

