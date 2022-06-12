use std::fs;

pub fn read_jpeg(filename: String) {
    let contents = fs::read(filename).expect("Something went wrong reading the file");
    println!(
        "{}", contents.len()
    )
}