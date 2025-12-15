use std::io::{self, Write};

use crate::search::{Record, binary_search_by_id_std, binary_serach};

mod search;

fn main() {
    let mut records = vec![
        Record {
            id: 1,
            name: "Anol".to_string(),
        },
        Record {
            id: 2,
            name: "Alice".to_string(),
        },
        Record {
            id: 3,
            name: "Bob".to_string(),
        },
        Record {
            id: 5,
            name: "Amruta".to_string(),
        },
        Record {
            id: 7,
            name: "David".to_string(),
        },
        Record {
            id: 10,
            name: "Eve".to_string(),
        },
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
        // match result{
        //     Some(record) => println!("Record with id {} found {:?}",target_id,record),
        //     None => println!("No record found!"),

        // }

        let result = binary_search_by_id_std(&records, target_id);
        match result {
            Some(record)=>println!("Record with id {} found {:?}",target_id,record),
             None => println!("No record found!"),
        }
    }
}
