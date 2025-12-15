use std::io::{self, Write};

use crate::search::Record;

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
            name: "Charlie".to_string(),
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
    }
}
