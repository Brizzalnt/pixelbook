use std::fs;
use std::io::{self, Write}; 


#[derive(Debug)]
struct PixelData{
    alpha : u8,
    cyan : u8,
    magenta : u8,
    blue : u8,
    black : u8,
    shine : u8,
    x: f64,
    y: f64,
    z: f64,
    shape : String,
}

fn main() {
    let mut pixelNumber : u128;
    let mut maxDotts : u128 ;
    let mut colorRGBfromCMYK : String;
    maxDotts = 1000;
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
    pixelNumber = 0;
    while pixelNumber <maxDotts
    {
        pixelNumber +=1;
    let mut _pixel = PixelData{ alpha : 255, cyan: 0, magenta: 0, blue:0 , black :0 , shine: 0,  x:0.0 , y:0.0 , z:0.0, shape:"pixel".to_string()};
    println!("{:?}", _pixel);
    }
}
  