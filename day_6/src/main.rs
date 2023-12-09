use std::{path::Path, fs::File, io::{BufReader, Read}, process::exit};

trait RaceMethods{
    fn new(distances: Vec<u64>, records: Vec<u64>) -> Result<Self, String> where Self: Sized;
    fn race(distance: u64, holding_time: u64) ->u64;
    fn find_ways_to_beat_record(distance: u64, record: u64) -> u64;
    fn can_beat(distance: u64, record: u64, holding_time: u64) -> bool;
    fn find_total_ways_to_beat_record(&self) ->u64;
    fn find_min_time (distance: u64, record: u64) ->u64;
    fn find_max_time(distance: u64, record: u64, low_init: u64) -> u64;
    fn find_ways_to_beat_single(distance: u64, record: u64) -> u64;
}

struct Race{
    time_vec: Vec<u64>,
    record_vec: Vec<u64>
}

impl RaceMethods for Race {
    fn new(distances: Vec<u64>, records: Vec<u64>) -> Result<Self, String> where Self: Sized {
        if distances.len() != records.len(){
            eprintln!("ERROR: distance and record vectors must be the same length");
            return Err("Constructor error".to_string());
        }

        Ok(Self { time_vec: distances, record_vec: records })
    }

    fn race(distance: u64, holding_time: u64) ->u64 {
        let mut result_time: u64 = 0;
        let mut speed: u64 = 0; 
        let mut hold_time = holding_time.clone();
        let mut time = distance.clone();

        while time > 0{
            if hold_time > 0 {
                speed += 1;
                hold_time -= 1;
            }else{
                result_time += speed;
            }
            time -= 1;
        } 

        result_time

    }

    fn find_ways_to_beat_record(distance: u64, record: u64) -> u64 {
        let mut ways_to_beat_record: u64 = 0;
        for i in 0..distance{

            let time = Self::race(distance, i);
            if time > record{
                ways_to_beat_record +=1;
            }
        }

        ways_to_beat_record
    }

    fn find_total_ways_to_beat_record(&self) ->u64 {
        let mut counter = 0; 
        let mut total_ways:u64 = 1;

        while  counter  <  self.time_vec.len() {
            let current_record = self.record_vec.get(counter).expect("Error getting record");
            let current_distance = self.time_vec.get(counter).expect("Error getting the distance");

            let ways_to_beat_current_record = Self::find_ways_to_beat_record(*current_distance, *current_record);
            
            if ways_to_beat_current_record != 0{
                total_ways *= ways_to_beat_current_record;
            }

            counter+=1 
        }

        total_ways
        
    }

    fn can_beat(distance: u64, record: u64, holding_time: u64) -> bool {
        Self::race(distance, holding_time) > record
    }

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

    fn find_max_time(distance: u64, record: u64, low_init: u64) -> u64 {
    
        // Binary search for refinement
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

    fn find_ways_to_beat_single(distance: u64, record: u64) -> u64{
        let min = Race::find_min_time(distance, record);
        return Race::find_max_time(distance, record, min) - min;
    }
}


fn create_number_vector_from_line(index: usize ,line: &str) -> Vec<u64>{
    let line_vec: Vec<_> = line[index..].split_whitespace().collect();

    let numbers: Vec<u64> = line_vec.iter()
    .filter_map(|&s| s.parse().ok())
    .collect();

    numbers
}

fn concatenate_numbers(numbers: Vec<u64>) -> Option<u64> {
    // Convert each number to a string and concatenate them
    let concatenated_str: String = numbers.iter().map(|&num| num.to_string()).collect();

    // Parse the concatenated string into a single u64
    match concatenated_str.parse() {
        Ok(result) => Some(result),
        Err(_) => None, // Parsing failed, return None or handle the error accordingly
    }
}


fn main() -> std::io::Result<()>{
    println!("--- Day 6: Wait For It ---");

    // Save the content of a file to a string; 
    let mut content = String::new();
    
    // Open the file of input relative to the folder
    let path = Path::new("times.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file. 
    // A buffer reader is a way to optimize reads, by making a single sys call 
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;

    // Create variables 
    let mut time_vec: Vec<u64> = Vec::new();
    let mut record_vec: Vec<u64> = Vec::new();

    // For each line we need to fine the number
    for line in content.lines(){
        if line.starts_with("Time:"){
            time_vec = create_number_vector_from_line(5, line);
        }else if line.starts_with("Distance:"){
            record_vec = create_number_vector_from_line(9, line)
        }else{
            eprintln!("ERROR: line was neither a time or record: {line}");
            exit(1);
        }

    }

    // Part 1  => Answer: 4403592
    let race = Race::new(time_vec.clone(), record_vec.clone()).unwrap_or_else(|err|{
        eprintln!("ERROR: was not able to create race struct: {err}");
        exit(1);
    });

    let ways_to_win_total = race.find_total_ways_to_beat_record();


    println!("Ways to win total = {ways_to_win_total}");

    // Part 2

    let high_time: u64 =  match concatenate_numbers(time_vec.clone()){
        Some(numb) => numb,
        None => {
            eprintln!("ERROR: was not join time_vec: {:?}", time_vec);
            exit(1);
        }
    };

    let high_record: u64 = match concatenate_numbers(record_vec.clone()){
        Some(numb) => numb,
        None => {
            eprintln!("ERROR: was not join record_vec: {:?}", record_vec);
            exit(1);
        }
    };
     
    let ways_to_win_total = Race::find_ways_to_beat_single(high_time, high_record);

    println!("Ways to win (part 2) = {ways_to_win_total}");

    Ok(())
}
