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
    let mut arraystore: [f32;8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];


    let mut indexer:usize = 0;
    
    match read_file_into_vec_by_separator(filepath, separator) {
        Ok(data) => {
            println!("Contents of the file, split by '{}':", separator);
            for (i, item) in data.iter().enumerate() {
                let mut item2 = item.parse::<f32>().unwrap();
                indexer +=1;
                match indexer{
                    1 => arraystore[0] = item2,
                    2 => arraystore[1] = item2,
                    3 => arraystore[2] = item2,
                    4 => arraystore[3] = item2,
                    5 => arraystore[4] = item2,
                    6 => arraystore[5] = item2,
                    7 => arraystore[6] = item2,
                    8 => arraystore[7] = item2,
                    _ => print!("unexpected")
                }
                
                if indexer == 8{
                    indexer = 0;
                    println!("finished array!")
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }


}
