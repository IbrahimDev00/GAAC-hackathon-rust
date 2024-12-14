use xxhash_rust::xxh3::xxh3_64;

pub fn hash(input_string: &[u8]) -> u64 {
    let hashed_string = xxh3_64(input_string);
    //println!("Hashed string: {}", hashed_string);
    return hashed_string;
}