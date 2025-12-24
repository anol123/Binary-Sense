use std::io::{self, Write};

use crate::search::{Record, binary_search_by_id_std};

mod search;
mod loader;

fn main() {
    let mut records = vec![
        Record::new(1, "Anol"),
        Record::new(2, "Alice"),
        Record::new(3, "Bob"),
        Record::new(5, "Amruta"),
        Record::new(7, "David"),
        Record::new(10, "Eve"),
        Record::new(11, "Niraj"),
        Record::new(12, "Sayan"),
        Record::new(13, "Amruta Padhi"),
        Record::new(14, "Rohit"),
        Record::new(17, "PP"),
        Record::new(20, "Anol SDE"),
    ];

    //take user input of id they want to search
    //take user input of wheather search or quit

    records.sort_by_key(|record| record.id);

    loop {
        println!("Enter an id to search (or 'q' to quit):");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let mut trimmed = input.trim();

        //handle quit case
        if trimmed.eq_ignore_ascii_case("q") {
            println!("Goodbye!");
            break;
        }

        let mut target_id: u32 = match trimmed.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number or 'q' to quit.\n");
                continue;
            }
        };

        //use custom binary search
        // let result = binary_serach(&records, target_id);

        // //Checking the option for some and none
        // match result {
        //     Some(record) => println!("Record with id {} found {:?}", target_id, record),
        //     None => println!("No record found!"),
        // }

        let result = binary_search_by_id_std(&records, target_id);
        match result {
            Some(record)=>println!("Record with id {} found {:?}",target_id,record),
             None => println!("No record found!"),
        }
    }
}
