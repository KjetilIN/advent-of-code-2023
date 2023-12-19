# Day 9 - Description

Given a range of numbers 
1. Find the next number 
2. Find the first number 

Each line is a sequence of these numbers and both tasks asks to return a sequence of these numbers

Link to complete description: https://adventofcode.com/2023/day/9

## Approach/Algorithm

To solve this we need to know the following:

If the sequence is ``, all we need to find is how much to increment that number with.
We could do this different ways, but the task asks us to check how much the incremented number is incremented with. And then the next. We can do this and we will find that there is a constant number that all numbers are incremented with:

```
1   3   6  10  15  21
  2   3   4   5   6
    1   1   1   1
      0   0   0
```

With this information we can both find the first and the last number. We can image that we are trying to predict the first number and the last number. In this case numbers A, B, C on both sides:

```
C   3   6  10  15  C
  B   3   4   5   B
    A   1   1   A
      0   0   0
```

To find the first and the last number
1. Loop over each sequence of numbers and calculate the difference
    1. Break out of the loop if all numbers are 0
    2. For the first number => we add the first difference number to a vector
    3. For the next number => we add the last difference number to a vector
2. Order the sequence of difference numbers from each level. The order should be from the lowest level to the highest level.
3. Calculate the result by iterating over each number. We create a variable storing the result
    1. For the first number => Set result to the current difference number minus the current result
    2. For the last number => Add the incremented number to the result


## Code Snippets

Prediction data structure was very simple and the traits implemented was not too hard as well:
```rust
pub struct Prediction {
    numbers: Vec<i64>,
}

pub trait PredictionMethods {
    fn with_numbers(numbers: &str) -> Result<Self, String> where Self: Sized;
    fn predict_next_number(&self) -> i64;
    fn predict_first_number(&self) -> i64;
}
```

