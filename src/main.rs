
use std::fs::File;
use std::io::{self, BufReader, Read};
use image::{RgbaImage, Rgba};

use std::io::BufWriter;
use std::sync::{LazyLock, Mutex}; // Import Mutex

// pub static mut imgbuf: RgbaImage = RgbaImage::new(32, 32); // OLD line
pub static imgbuf: LazyLock<Mutex<RgbaImage>> = LazyLock::new(|| {
    Mutex::new(RgbaImage::new(32, 32)) // Wrap the RgbaImage in a Mutex
});
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


fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let mut filepath = "";
    filepath = "glow.nibb";
    let separator = ','; 
    let mut arraystore: [f32;8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut offset_x = 16;
    let mut offset_y = 16;
    let mut array_index:usize = 0;
    let mut indexer:usize = 0;
    let mut count:usize =0;
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
                    
                    creator(arraystore[0]as u32+offset_x,arraystore[1]as u32+offset_y,arraystore[2]as u32,arraystore[3],arraystore[4],arraystore[5],arraystore[6],arraystore[7]);
               
                    
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }

    }
    
   
    Ok(())
}
pub fn creator(ax:u32,ay:u32,az:u32,c:f32,m:f32,y:f32,k:f32,a:f32){
    let r = (255.0 * (1.0 - c) * (1.0 - k)) as u8;
    let g = (255.0 * (1.0 - m) * (1.0 - k)) as u8;
    let b = (255.0 * (1.0 - y) * (1.0 - k)) as u8;
    let a = (255.0 * (1.0 - a) *(1.0 - k )) as u8;

    
    imgbuf.lock().unwrap().put_pixel(ax, ay, Rgba([r, g, b,a]));
            

    imgbuf.lock().unwrap().save("output.png").unwrap();
}

    
