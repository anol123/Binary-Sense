use std::{error::Error, fs::File};

use csv::ReaderBuilder;

use crate::search::Record;

pub fn load_records_from_csv(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().trim(csv::Trim::All).from_reader(file);
    let mut records = Vec::new();

    for result in reader.deserialize() {
        let record: Record = result?;
        records.push(record);
    }

    Ok(records)
}
