use std::error::Error;
use std::fs::File; 
use csv::Reader; 

pub fn reader(file_path: &str) -> Result<csv::Reader<File>, Box<dyn Error>> {
    // Open the file for reading
    let read_file = File::open(file_path)?;

    // Create a CSV reader from the file
    let reader = csv::Reader::from_reader(read_file);

    Ok(reader)
}



fn main() -> Result<(), Box<dyn Error>> {
    // Example usage of the reader function
    let file_path = "data.csv";
    let mut reader = reader(file_path)?;

    // Iterate through the records and process them
    for result in reader.records() {
        match result {
            Ok(record) => {
                // Process or print each record
                println!("{:?}", record);
            }
            Err(err) => {
                eprintln!("Error reading a row: {}", err);
            }
        }
    }

    Ok(())
}






   


     
   

