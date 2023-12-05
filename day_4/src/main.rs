use std::{path::Path, fs::File, io::{BufReader, Read}, error::Error};


struct Card {
    card_id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>
}


impl Card {
    
    fn new(card_string: &String) -> Result<Self, String>{

        let card_id:u32 = match &card_string[5..9].trim().parse(){
            Ok(id) => *id,
            Err(_) => {
                eprintln!("ERROR: creating card from id string : {:?}", &card_string[5..9].to_string());
                return Error("Could not create card");
            },
        };

        let winning_numbers: Vec<u32> = Vec::new();
        let card_numbers: Vec<u32> = Vec::new();


        Ok(Self{
            card_id,
            winning_numbers,
            card_numbers
        })


    }
    
}




fn main() ->std::io::Result<()> {
    println!("--- Day 4: Scratchcards ---");

    let mut content = String::new();
    
    // Open the file of input relative to the folder
    let path = Path::new("cardList.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file. 
    // A buffer reader is a way to optimize reads, by making a single sys call 
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;

    // For each line we need to fine the number
    for line in content.lines(){
        // Add the number from the file to the total sum
      

    }

    Ok(())
}
