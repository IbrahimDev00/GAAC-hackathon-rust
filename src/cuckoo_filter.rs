use crate::count_min::CountMinSketch;

pub fn cuckoo_filter(hashed_string: u64) {
    let mut cms = CountMinSketch::new(10000, 5);
    let mut cf = cuckoofilter::CuckooFilter::new();
    if cf.contains(&hashed_string) {
        println!("element is present in cuckoofilter");
        cms.add(&hashed_string.to_le_bytes());
    } else {
        println!("element is not present in cuckoofilter");
        cf.add(&hashed_string).unwrap();
        cms.add(&hashed_string.to_le_bytes());
    }
}