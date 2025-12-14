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
    ];
}
