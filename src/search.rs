#[derive(Debug)]
pub struct Record {
    pub id: u32,
    pub name: String,
}

pub fn binary_serach(records: &[Record], target_id: u32)-> Option<&Record> {
    let mut low = 0;
    let mut high = records.len()-1;

    while low <= high {
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
pub fn binary_search_by_id_std(records: &[Record], target_id: u32) -> Option<&Record> {
    match records.binary_search_by_key(&target_id, |r| r.id) {
        Ok(index) => Some(&records[index]),
        Err(_) => None,
    }
}
