use std::{path::Path, fs::File, io::{BufReader, Read}, process::exit};

trait RaceMethods{
    fn new(distances: Vec<u32>, records: Vec<u32>) -> Result<Self, String> where Self: Sized;
    fn race(distance: u32, holding_time: u32) ->u32;
    fn find_ways_to_beat_record(distance: u32, record: u32) -> u32;
    fn find_total_ways_to_beat_record(&self) ->u32;
}

struct Race{
    time_vec: Vec<u32>,
    record_vec: Vec<u32>
}

impl RaceMethods for Race {
    fn new(distances: Vec<u32>, records: Vec<u32>) -> Result<Self, String> where Self: Sized {
        if distances.len() != records.len(){
            eprintln!("ERROR: distance and record vectors must be the same length");
            return Err("Constructor error".to_string());
        }

        Ok(Self { time_vec: distances, record_vec: records })
    }

    fn race(distance: u32, holding_time: u32) ->u32 {
        let mut result_time: u32 = 0;
        let mut speed: u32 = 0; 
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

    fn find_ways_to_beat_record(distance: u32, record: u32) -> u32 {
        let mut ways_to_beat_record: u32 = 0;
        for i in 0..distance{
            let time = Self::race(distance, i);
            if time > record{
                ways_to_beat_record +=1;
            }
        }

        ways_to_beat_record
    }

    fn find_total_ways_to_beat_record(&self) ->u32 {
        let mut counter = 0; 
        let mut total_ways:u32 = 1;

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
}


fn create_number_vector_from_line(index: usize ,line: &str) -> Vec<u32>{
    let line_vec: Vec<_> = line[index..].split_whitespace().collect();

    let numbers: Vec<u32> = line_vec.iter()
    .filter_map(|&s| s.parse().ok())
    .collect();

    numbers
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
    let mut time_vec: Vec<u32> = Vec::new();
    let mut record_vec: Vec<u32> = Vec::new();

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

    // Part 1 
    let race = Race::new(time_vec, record_vec).unwrap_or_else(|err|{
        eprintln!("ERROR: was not able to create race struct: {err}");
        exit(1);
    });

    let ways_to_win_total = race.find_total_ways_to_beat_record();


    println!("Ways to win total = {ways_to_win_total}");


    Ok(())
}
