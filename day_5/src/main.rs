use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path, ops::Range, process::exit,
};

struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: HashMap<u32, u32>,
    soil_to_fertilizer: HashMap<u32, u32>,
    fertilizer_to_water: HashMap<u32, u32>,
    water_to_light: HashMap<u32, u32>,
    light_to_temperature: HashMap<u32, u32>,
    temperature_to_humidity: HashMap<u32, u32>,
    humidity_to_location: HashMap<u32, u32>,
}

impl Almanac {
    fn with_list(content: &String) -> Result<Self, String> {

        // Declare all variables that we need to create
        let mut seeds: Vec<u32> = Vec::new();
        let mut seed_to_soil: HashMap<u32, u32> = HashMap::new();
        let mut soil_to_fertilizer: HashMap<u32, u32> = HashMap::new();
        let mut fertilizer_to_water: HashMap<u32, u32> = HashMap::new();
        let mut water_to_light: HashMap<u32, u32> = HashMap::new();
        let mut light_to_temperature: HashMap<u32, u32> = HashMap::new();
        let mut temperature_to_humidity: HashMap<u32, u32> = HashMap::new();
        let mut humidity_to_location: HashMap<u32, u32> = HashMap::new();


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
            let numbers: Vec<u32> = line
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
                "seed-to-soil map" => add_range_to_map(&mut seed_to_soil, dest, source, range),
                "soil-to-fertilizer map" => add_range_to_map(&mut soil_to_fertilizer, dest, source, range),
                "fertilizer-to-water map" => add_range_to_map(&mut fertilizer_to_water, dest, source, range),
                "water-to-light map" => add_range_to_map(&mut water_to_light, dest, source, range),
                "light-to-temperature map" => add_range_to_map(&mut water_to_light, dest, source, range),
                "temperature-to-humidity map" => add_range_to_map(&mut water_to_light, dest, source, range),
                "humidity-to-location map" => add_range_to_map(&mut water_to_light, dest, source, range),
                _ =>{}
            }
        }
        


        // Validate that the constructor is correct 
        if seeds.len() == 0 {
            eprintln!("ERROR: no seeds given, length is 0");
            return Err("Illegal file".to_string());
        }

        if seed_to_soil.len() == 0 {
            eprintln!("ERROR: no seeds-to-soil given, length is 0");
            return Err("Illegal file".to_string());
        }

        if soil_to_fertilizer.len() == 0 {
            eprintln!("ERROR: no soil_to_fertilizer given, length is 0");
            return Err("Illegal file".to_string());
        }

        if fertilizer_to_water.len() == 0 {
            eprintln!("ERROR: no fertilizer_to_water given, length is 0");
            return Err("Illegal file".to_string());
        }

        if water_to_light.len() == 0 {
            eprintln!("ERROR: no water_to_light given, length is 0");
            return Err("Illegal file".to_string());
        }

        if light_to_temperature.len() == 0 {
            eprintln!("ERROR: no light_to_temperature given, length is 0");
            return Err("Illegal file".to_string());
        }

        if temperature_to_humidity.len() == 0 {
            eprintln!("ERROR: no temperature_to_humidity, length is 0");
            return Err("Illegal file".to_string());
        }

        if humidity_to_location.len() == 0 {
            eprintln!("ERROR: no humidity_to_location given, length is 0");
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

// Function that adds the range of numbers to the correct Hashmap
fn add_range_to_map(map: &mut HashMap<u32,u32>, dest:u32, source:u32, range:u32 ){
    let dest_range: Range<u32> = dest..dest+range;
    let source_range: Range<u32> = source..source+range;

    for (dest_key, source_key) in dest_range.zip(source_range) {
        map.insert(dest_key, source_key);
    }

    println!("ADDED");
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
