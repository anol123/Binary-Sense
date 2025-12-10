struct Record {
    id: u32,
    name: String,
}

fn binary_serach(records: &[Record], target_id: u32)-> Option<&Record> {
    let mut low = 0;
    let mut high = records.len();

    while low < high {
        let mid = (low + high) / 2;
        let mid_id = records[mid].id;

        if mid_id ==target_id{
            return Some(&records[mid]);
        } else if mid_id< target_id{
            low=mid+1;
        } else{
            high= mid-1;
        }
    }
    None
}
