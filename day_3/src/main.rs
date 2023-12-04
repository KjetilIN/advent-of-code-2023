use std::{fs::File, io::BufReader};
use std::io::Read;

struct Map<'a>{
    rows: &'a Vec<String>,
    col_length: u32,
    row_length: u32
}

impl<'a> Map<'a>{

    fn new(rows: &'a Vec<String>) -> Self{
        let row_length = rows.len() as u32;
        let col_length = rows.get(0).unwrap().len().clone() as u32;

        Self { rows, col_length, row_length }
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
