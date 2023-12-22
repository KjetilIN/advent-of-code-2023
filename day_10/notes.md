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
    1. For example in this case there are 4 tiles inside the loop. (Represented with `I`)
    ```text
    ..........
    .S------7.
    .|F----7|.
    .||OOOO||.
    .||OOOO||.
    .|L-7F-J|.
    .|II||II|.
    .L--JL--J.
    ..........
    ```



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

## For part 1.

1. Find the index of the start pipe (constructor does this)
2. Find the next pipe from the starting pipe and set the direction of the flow based on the type of pipe
3. Loop until we are at the start pipe. We increment the step for each pipe we "move through":
    1. Each pipe we check the current direction. Based on the direction we get the north, south, east or west pipe. From the method we get the new index and new direction of the flow. We update the current index and direction
4. Return count of steps divided by 2. This is because count of steps will be how many steps the entire loop is. The farthest away pipe is half the amount of the steps. 


## For part 2:

Part 2 was very mathematical. I was very stuck until I got tips to look into **Pick's theorem** and **Shoelace formula**. They are to mathematic formulas that forces us to think about the loop, as a polygon. First, I created a function that gets all the points for the loop. It uses the `Point` struct:

```rust
pub struct Point{
    pub x: i32,
    pub y: i32
}
```

A given polygon has both interior points (points that are inside the polygon) and boundary points, we can calculate the area of the polygon. It also has to be a simple lattice polygon. 
1. Simple => meaning no overlap or holes in the polygon.
2. Lattice => meaning that all points are one a whole number. For instance (2,4) is lattice point but (1.2 , 1.0) is not. 

Pick's theorem gives the following formula for the area:
```
A = i + b/2 - 1 
```

We want to know how many inner tiles the loop has - also known as interior points in the polygon `i`. We can use Pick's theorem to express the number of inner tiles. 

We can find `b` - the number of boundary points, by just counting how many pipes we have. We only need to find the area. 

We can calculate it with the **Shoelace formula**. It offers a set of formulas for calculating the area of a polygon by cross multiplication of all the points. I used the formula for the Trapezoid 


The two theorems was then put into a single function:
```rust
fn count_enclosed_tiles(&self) -> Result<u32, String> {
    let points = match self.collect_points(){
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    let area = find_area_from_points(&points);
    let pipes = points.len().clone() as u32;

    // Formula i = A - b/2 + 1
    let tiles:u32 = area - pipes/2 + 1;

    Ok(tiles)

}
```


## Sources:
- Pick's theorem => https://en.wikipedia.org/wiki/Pick%27s_theorem 
- Shoelace formula => https://en.wikipedia.org/wiki/Shoelace_formula