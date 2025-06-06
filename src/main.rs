
use std::fs::File;
use std::io::{self, BufReader, Read};

mod nibber;

use std::io::BufWriter;
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



pub fn draw_pixel_cmyk(
    ax: u32,        // X-coordinate
    ay: u32,        // Y-coordinate
    acyan: f32,     // Cyan component (0.0 to 1.0)
    amagenta: f32,  // Magenta component (0.0 to 1.0)
    ayellow: f32,   // Yellow component (0.0 to 1.0)
    ak: f32,        // Black (Key) component (0.0 to 1.0)
    aalpha: f32,
) {


}


fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "my_file.dott";
    let separator = ','; 
    let mut arraystore: [f32;8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];


    let mut indexer:usize = 0;
    
    match read_file_into_vec_by_separator(filepath, separator) {
        Ok(data) => {
            println!("openfile...");
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
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }

    }
    
    let mut cy : u32;
    let mut cx : u32;
    let mut ccyan : f32;
    let mut cmagenta :f32;
    let mut cyellow : f32;
    let mut ck : f32;
    let mut calpha: f32;
    let mut pixelNumber : u128;
    let mut maxDotts : u128 ;
    let mut Datastring : String;
    maxDotts = 1;
    
    let cy : u32;
    let cx : u32;
    let cz : u32;
    cx = 1;
    cy = 1;
    cz = 1;
    let ccyan = 0.0;
    let cmagenta = 0.13;
    let cyellow = 0.87;
    let ck = 0.0;
    let calpha = 1.0;
    let mut pixelNumber =1;

          
    pixelNumber = 0;
    while pixelNumber <maxDotts {
         pixelNumber+=1;
              
        println!("done.");
        nibber::creator(cx,cy,cz,ccyan,cmagenta,cyellow,ck,calpha);

    }
    
   
    Ok(())
}


    