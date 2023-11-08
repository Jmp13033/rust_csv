use std::error::Error;
use std::fs::File;
use csv::Reader;

#[derive(Debug)]
pub struct Patient {
    PatientID: i32,
    Name: String,
    Age: i32,
    Gender: String,
}

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
    let mut rdr = reader(file_path)?;
    for result in rdr.records() {
        let record = result?;
        let patient = Patient {
            PatientID: record[0].parse()?,
            Name: record[1].to_string(),
            Age: record[2].parse()?,
            Gender: record[3].to_string(),
        };
        println!("{:?}", patient);
    }

    Ok(())
}







   


     
   

