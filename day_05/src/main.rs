use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path, ops::Range, process::exit,
};
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_to_soil: Vec<AlmanacRange>,
    pub soil_to_fertilizer: Vec<AlmanacRange>,
    pub fertilizer_to_water: Vec<AlmanacRange>,
    pub water_to_light: Vec<AlmanacRange>,
    pub light_to_temperature: Vec<AlmanacRange>,
    pub temperature_to_humidity: Vec<AlmanacRange>,
    pub humidity_to_location: Vec<AlmanacRange>,
}

pub struct AlmanacRange{
    dest_range: Range<u64>,
    source_range: Range<u64>,
}

impl AlmanacRange {
    pub fn new(dest:u64, source:u64, range:u64) -> Self{
        let dest_end = dest.checked_add(range).expect("Overflow in dest + range");
        let source_end = source.checked_add(range).expect("Overflow in source + range");

        Self {
            dest_range: dest..dest_end,
            source_range: source..source_end,
        }
    }

    /// Gets the destination given a source 
    pub fn get_dest_from_source(&self, source:u64) -> u64{
        // Check if the given source is in the source length
        if self.source_range.contains(&source) {
            if let Some(append_length) = source.checked_sub(self.source_range.start) {
                return self.dest_range.start + append_length;
            }
        }

        source
    }

    pub fn get_dest_range_from_source_range(&self, source_range: Range<u64>) -> Range<u64> {
        if self.source_range.end <= source_range.start || self.source_range.start >= source_range.end {
            return source_range; // No overlap, return the original source range
        }

        // Calculate the intersection between the source range and the almanac range
        let intersection = source_range.start.max(self.source_range.start)..source_range.end.min(self.source_range.end);
        let intersection_length = source_range.end - source_range.start;

        // Calculate the destination for the intersected range
        let new_start = self.dest_range.start + (intersection.start - self.source_range.start);
        let new_end = new_start + intersection_length as u64;

        new_start..new_end
    }

}






impl Almanac {
    pub fn with_list(content: &String) -> Result<Self, String> {

        // Declare all variables that we need to create
        let mut seeds: Vec<u64> = Vec::new();

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

            // Jump over empty lines
            if line.is_empty(){
                continue;
            }
            
            // If we found the seeds 
            if line.starts_with("seeds:"){
                let seeds_parsed: Result<Vec<u64>, _> = line[6..].split_whitespace()
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


    // Find all the smallest destination numbers 
    pub fn find_smallest_destination_number(&self ) -> u64{
        let mut smallest_seed: u64 = u64::MAX;

        // For each 
        for seed in &self.seeds{
            let soil = get_dest_from_source_vectors(&self.seed_to_soil, seed.clone());
            let fertilizer = get_dest_from_source_vectors(&self.soil_to_fertilizer, soil);
            let water = get_dest_from_source_vectors(&self.fertilizer_to_water, fertilizer);
            let light = get_dest_from_source_vectors(&self.water_to_light, water);
            let temperature = get_dest_from_source_vectors(&self.light_to_temperature, light);
            let humidity = get_dest_from_source_vectors(&self.temperature_to_humidity, temperature);
            let location = get_dest_from_source_vectors(&self.humidity_to_location, humidity);
            
            // Change the location
            if location < smallest_seed{
                smallest_seed = location;
            }
        }


        return smallest_seed
        
    }

    // TODO: FIX THIS FOR PART 2
    pub fn find_min_location_from_range(&self)->u64{
        // Check if we have seeds in pairs
        if self.seeds.len() % 2 != 0 {
            eprintln!("Error: not an even number of seeds given");
            std::process::exit(1);
        }

        // Create the vector of ranges that needs to be checked
        let mut ranges: Vec<Range<u64>> = Vec::new();

        let mut counter = 0;
        while counter < self.seeds.len() {
            let start: u64 = self.seeds[counter];
            let length: u64 = self.seeds[counter + 1];

            ranges.push(Range {
                start,
                end: start + length,
            });

            counter += 2;
        }

        
        let mut smallest_location: u64 = u64::MAX;

        // For each 
        for seed in ranges{
            let mut seed_vec: Vec<Range<u64>> = Vec::new();
            seed_vec.push(seed);

            let soil = get_dest_range_from_range_vectors(&self.seed_to_soil, seed_vec);
            let fertilizer = get_dest_range_from_range_vectors(&self.soil_to_fertilizer, soil);
            let water = get_dest_range_from_range_vectors(&self.fertilizer_to_water, fertilizer);
            let light = get_dest_range_from_range_vectors(&self.water_to_light, water);
            let temperature = get_dest_range_from_range_vectors(&self.light_to_temperature, light);
            let humidity = get_dest_range_from_range_vectors(&self.temperature_to_humidity, temperature);
            let location = get_dest_range_from_range_vectors(&self.humidity_to_location, humidity);
            
            let mut temp_smallest_range = location.first().unwrap();

            for range in &location{
                let new_small = temp_smallest_range.start.min(range.start);
                if new_small < smallest_location{
                    smallest_location = new_small;
                    temp_smallest_range = &range;
                }

            }
        }

        smallest_location
        

    }



}

fn get_dest_from_source_vectors(source_vector: &Vec<AlmanacRange>, current_source: u64) -> u64{
    for almanac_item in source_vector{
        let new_potential_source = almanac_item.get_dest_from_source(current_source);
        if new_potential_source != current_source{
            return new_potential_source;
        }
    }  
    return current_source;
}

fn get_dest_range_from_range_vectors(source_vector: &Vec<AlmanacRange>, current_source: Vec<Range<u64>>) -> Vec<Range<u64>>{
    let mut result_ranges: Vec<Range<u64>> = Vec::new();

    for source_range in current_source.into_iter() {
        let mut remaining_source_range = Some(source_range.clone());

        let mut has_overlap = false;

        for almanac_item in source_vector {
            if almanac_item.source_range.end <= source_range.start || almanac_item.source_range.start >= source_range.end {
                continue; // No intersection between the source range and the almanac range
            }

            has_overlap = true;

            // Calculate the intersection between the source range and the almanac range
            let intersection = source_range.start.max(almanac_item.source_range.start)..source_range.end.min(almanac_item.source_range.end);

            // If there are non-overlapping parts before the intersection, push them to the result vector
            if intersection.start > remaining_source_range.as_ref().unwrap().start {
                let non_overlap_start = remaining_source_range.as_ref().unwrap().start;
                let non_overlap_end = intersection.start;
                result_ranges.push(non_overlap_start..non_overlap_end);
            }

            // Calculate the destination for the intersected range
            let new_start = almanac_item.dest_range.start + (intersection.start - almanac_item.source_range.start);
            let length = intersection.end - new_start;
            let new_end = new_start + length as u64;
            result_ranges.push(new_start..new_end);

            // Update remaining source range after the intersection
            remaining_source_range = Some(intersection.end..source_range.end);
        }

        // If there is no overlap, push the original source range
        if !has_overlap {
            result_ranges.push(source_range);
        }

        // If there are non-overlapping parts after the last intersection, push them to the result vector
        if let Some(remaining) = remaining_source_range {
            result_ranges.push(remaining);
        }
    }

    result_ranges
    
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


    let almanac = match Almanac::with_list(&content){
        Ok(instance) => instance,
        Err(_) => {
            eprintln!("ERROR");
            exit(1);
        }
    };

    let smallest_location = almanac.find_smallest_destination_number();
    // TODO: Do part 2
    //let smallest_location_v2 = almanac.find_min_location_from_range();
    println!("Smallest location: {}", smallest_location);
    // println!("Smallest location (range): {}", smallest_location_v2);

    Ok(())
}
