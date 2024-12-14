mod hash;
mod cuckoo_filter;
mod count_min;
use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter text:");

    // Read the input from the standard input
    io::stdin()
        .read_line(&mut input) // Append input into the mutable String
        .expect("Failed to read line");

    // Remove the newline character from the input
    let input = input.trim();
    let hashed_value = hash::hash(input.as_bytes());
    println!("Hashed value: {}", hashed_value);
    cuckoo_filter::cuckoo_filter(hashed_value);
}
