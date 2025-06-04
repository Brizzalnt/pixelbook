use std::fs;
use std::io::{self, Write}; 

#[derive(Debug)]
struct PixelData{
    alpha : u8,
    red : u8,
    green : u8,
    blue : u8,
    emit : u8,
    shine : u8,
    x: f64,
    y: f64,
    z: f64,
    
}

fn main() {

    print!("Please enter the file path: ");
    io::stdout().flush().expect("Failed to flush stdout"); 

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)
        .expect("Failed to read line");

    let file_path = file_path.trim(); 


    match fs::read_to_string(file_path) {
        Ok(content) => {

            println!("{}", content);

        }
        Err(err) => {
            eprintln!("Error reading file '{}': {}", file_path, err);
        }
    }

    let mut _pixel = PixelData{ alpha : 255, red: 0, green: 0, blue:0 , emit:0 , shine: 0,  x:0.0 , y:0.0 , z:0.0};
    println!("{:?}", _pixel);
}
