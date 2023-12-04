use std::{fs::File, io::BufReader};
use std::io::Read;

struct Map<'a>{
    rows: &'a Vec<String>
}

impl Map<'_>{

    fn new(given_rows: &'a Vec<String>) -> Self{
        Self { rows: given_rows }

    }
    
}



fn main() -> std::io::Result<()> {
    println!("--- Day 3: Gear Ratios ---");

    let mut content = String::new();

    let file = File::open("map.txt")?;
    let mut buf_reader = BufReader::new(file);

    buf_reader.read_to_string(&mut content).unwrap_or_else(|err|{
        eprintln!("Error reading the map file : {err}");
        panic!();
    });


    Ok(())
}
