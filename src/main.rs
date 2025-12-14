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
}
