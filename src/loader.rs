use std::error::Error;

use crate::search::Record;

pub fn load_records_from_csv(path: &str)->Result<Vec<Record>, Box<dyn Error>>{
    let file = File::open(path)?;
    todo!()

}