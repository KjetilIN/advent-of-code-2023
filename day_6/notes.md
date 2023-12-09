# Day 6 - Description

A race is defined by a total amount of distance.
The starting speed is always zero. 
You can either: 
1. Use a single a distance unit for incrementing the speed by one
2. Move with your speed

Given a set of distances and corresponding records:
Find the amount of ways you could beat a record with:

1. Two vectors of distances and records. 
2. Two concatenated numbers (distance and record) from the two vectors

Link to complete description: https://adventofcode.com/2023/day/6

## Approach/Algorithm

I first created the struct for representing a race: 

```rust
struct Race{
    distance_vec: Vec<u64>,
    record_vec: Vec<u64>
}
```

The methods I implemented is listed in the following trait: 
```rust
trait RaceMethods{
    fn new(distances: Vec<u64>, records: Vec<u64>) -> Result<Self, String> where Self: Sized;
    fn race(distance: u64, holding_time: u64) ->u64;
    fn find_ways_to_beat_record(distance: u64, record: u64) -> u64;
    fn can_beat(distance: u64, record: u64, holding_time: u64) -> bool;
    fn find_total_ways_to_beat_record(&self) ->u64;
    fn find_min_time(distance: u64, record: u64) ->u64;
    fn find_max_time(distance: u64, record: u64, low_init: u64) -> u64;
    fn find_ways_to_beat_single(distance: u64, record: u64) -> u64;
}
```

For part 1, I could just brute force. 
1. Tried each holding time possible and found the race distance.
2. If the race distance was able to beat the record with the given holding time, we increment the amount 

For past 2, my brute force method was too slow.
With the following formula I could find the number of times we could beat the record:
1. Find the max holding time that is able to beat the record 
2. Find the minimum holding time that is able to beat the record
3. Take the max and subtract the minimum

This works quite well. I then used binary search in combination with a exponential increment. 
For the min:
1.  First exponential increment the holding time until I can not beat the record 
2.  The refine with binary search 
(For maximum, I did the same but not with step 2)

## Learnings

- Binary Search was a good algorithm that made sense once I found the formula. I should try to implement other algorithms when I see that they are useful!
- Used `trait` for defining what functions my struct needs. Cleaner way to create functions

## Code Snippets

For finding the minimum hold time: 

```rust
fn find_min_time(distance: u64, record: u64) ->u64{
    let mut holding_time: u64 = 1;

    // Exponential increment
    while !Race::can_beat(distance, record, holding_time) {
        holding_time *= 2;
    }

    // Binary search for refinement
    let mut low = holding_time / 2;
    let mut high = holding_time;

    while low < high {
        let mid = low + (high - low) / 2;

        if Race::can_beat(distance, record, mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low
}
```

For maximum hold time:

```rust
fn find_max_time(distance: u64, record: u64, low_init: u64) -> u64 {
    let mut low = low_init;
    let mut high = distance;

    while low < high {
        let mid = low + (high - low) / 2;

        if Race::can_beat(distance, record, mid) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}
```


