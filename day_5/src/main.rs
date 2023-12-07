use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path, ops::Range, process::exit,
};
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: Vec<AlmanacRange>,
    soil_to_fertilizer: Vec<AlmanacRange>,
    fertilizer_to_water: Vec<AlmanacRange>,
    water_to_light: Vec<AlmanacRange>,
    light_to_temperature: Vec<AlmanacRange>,
    temperature_to_humidity: Vec<AlmanacRange>,
    humidity_to_location: Vec<AlmanacRange>,
}

struct AlmanacRange{
    dest_range: Range<u64>,
    source_range: Range<u64>,
    length: u64
}

impl AlmanacRange {
    fn new(dest:u64, source:u64, range:u64) -> Self{
        let dest_end = dest.checked_add(range).expect("Overflow in dest + range");
        let source_end = source.checked_add(range).expect("Overflow in source + range");

        Self {
            dest_range: dest..dest_end,
            source_range: source..source_end,
            length: range,
        }
    }

    fn is_initialized(&self) -> bool {
        // Check if the fields are in a valid state
        !self.dest_range.is_empty() && !self.source_range.is_empty() && self.length > 0
    }
}

impl Almanac {
    fn with_list(content: &String) -> Result<Self, String> {

        // Declare all variables that we need to create
        let mut seeds: Vec<u32> = Vec::new();

        // Initialize with default values
        let mut seed_to_soil: Vec<AlmanacRange> = Vec::new();
        let mut soil_to_fertilizer: Vec<AlmanacRange>= Vec::new();
        let mut fertilizer_to_water: Vec<AlmanacRange> = Vec::new();
        let mut water_to_light: Vec<AlmanacRange>= Vec::new();
        let mut light_to_temperature: Vec<AlmanacRange> = Vec::new();
        let mut temperature_to_humidity:Vec<AlmanacRange> = Vec::new();
        let mut humidity_to_location: Vec<AlmanacRange>= Vec::new();


        // Keep track of the current section
        let mut current_section = String::new();


        // For each line of the content 
        for line in content.lines(){

            println!("LINE: {}", line);

            // Jump over empty lines
            if line.is_empty(){
                continue;
            }
            
            // If we found the seeds 
            if line.starts_with("seeds:"){
                let seeds_parsed: Result<Vec<u32>, _> = line[6..].split_whitespace()
                                                                 .map(|x| x.parse())
                                                                 .collect();

                // Handle the result of parsing the seed list 
                match seeds_parsed {
                    Ok(list) => seeds = list,
                    Err(err) => {
                        eprintln!("ERROR: parsing seeds : {err}");
                        return Err(err.to_string());
                    },
                }
                continue;
            }

            // If it ends with a colon, we are at a new section
            if line.ends_with(":"){
                current_section = line.trim_end_matches(":").to_string();
                continue;
            }

            // We have a line of numbers that we need to parse 
            // There should be three numbers
            let numbers: Vec<u64> = line
                                    .split_whitespace()
                                    .map(|s| s.parse().unwrap())
                                    .collect();

            // Check if the length match 
            if numbers.len()!= 3{
                eprintln!("ERROR: line did not give the expected three numbers: {line}");
                return Err("Line length error".to_string());
            }

            // With the three given numbers we can handle each case of the given current section
            let (dest, source, range) = (numbers[0], numbers[1], numbers[2]);

            match current_section.as_str(){
                "seed-to-soil map" => seed_to_soil.push(AlmanacRange::new(dest, source, range)),
                "soil-to-fertilizer map" => soil_to_fertilizer.push(AlmanacRange::new(dest, source, range)),
                "fertilizer-to-water map" => fertilizer_to_water.push(AlmanacRange::new(dest, source, range)),
                "water-to-light map" => water_to_light.push(AlmanacRange::new(dest, source, range)),
                "light-to-temperature map" => light_to_temperature.push(AlmanacRange::new(dest, source, range)),
                "temperature-to-humidity map" => temperature_to_humidity.push(AlmanacRange::new(dest, source, range)),
                "humidity-to-location map" => humidity_to_location.push(AlmanacRange::new(dest, source, range)),
                _ =>{}
            }
        }
        


        // Validate that the constructor is correct 
        if seeds.len() == 0 {
            eprintln!("ERROR: no seeds given, length is 0");
            return Err("Illegal file".to_string());
        }


        // Return a correctly constructed self 
        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }



}


fn main() -> std::io::Result<()> {
    println!("--- Day 5: If You Give A Seed A Fertilizer ---");

    // Save the content of a file to a string;
    let mut content = String::new();


    // Open the file of input relative to the folder
    let path = Path::new("seed_input.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file.
    // A buffer reader is a way to optimize reads, by making a single sys call
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;


    let test = match Almanac::with_list(&content){
        Ok(instance) => instance,
        Err(_) => {
            eprintln!("ERROR");
            exit(1);
        }
    };

    Ok(())
}
