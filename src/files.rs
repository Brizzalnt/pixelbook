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


    let mut indexer:usize = 0;
    
    match read_file_into_vec_by_separator(filepath, separator) {
        Ok(data) => {
            println!("Contents of the file, split by '{}':", separator);
            for (i, item) in data.iter().enumerate() {
                let mut labeler: String;
                let x2 = String::from("x");
                let y2: String = String::from("y");
                let z2 = String::from("z");
                let c2 = String::from("cyan");
                let m2: String = String::from("magenta");
                let y3 = String::from("yellow");
                let k2 = String::from("black");
                let a2: String = String::from("alpha");
                let end: String = String::from("!");
                
                

            
                indexer +=1;
                match indexer{
                    1 => labeler = x2.to_string(),
                    2 => labeler = y2.to_string(),
                    3 => labeler = z2.to_string(),
                    4 => labeler = c2.to_string(),
                    5 => labeler = m2.to_string(),
                    6 => labeler = y3.to_string(),
                    7 => labeler = k2.to_string(),
                    8 => labeler = a2.to_string(),
                    _ => labeler = end.to_string()
                }
                print!("{}: \"{}\"", labeler, item);
                if indexer == 8{
                    indexer = 0;
                    println!("")
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }


}
