use std::fs::File;
use std::io::{self, BufReader, Read};
pub fn read_file_into_vec_by_separator(
    filepath: &str,
    separator: char,
) -> io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    // Read the entire file content into a String
    reader.read_to_string(&mut contents)?;

    // Split the string by the separator and collect into a Vec<String>
    let result: Vec<String> = contents
        .split(separator)
        .map(|s| s.trim().to_string()) // Trim whitespace and convert to String
        .collect();

    Ok(result)
}

pub fn run() {
    let filepath = "my_file.dott";
    let separator = ','; 



    match read_file_into_vec_by_separator(filepath, separator) {
        Ok(data) => {
            println!("Contents of the file, split by '{}':", separator);
            for (i, item) in data.iter().enumerate() {
                println!("{}: \"{}\"", i, item);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }


}