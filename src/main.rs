
use std::fs::File;
use std::io::{self, BufReader, Read};
use image::{RgbaImage, Rgba};

use std::io::BufWriter;
use std::sync::{LazyLock, Mutex}; // Import Mutex

// pub static mut imgbuf: RgbaImage = RgbaImage::new(32, 32); // OLD line
pub static imgbuf: LazyLock<Mutex<RgbaImage>> = LazyLock::new(|| {
    Mutex::new(RgbaImage::new(128, 128)) // Wrap the RgbaImage in a Mutex
});
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut filepath = "";
    filepath = "loc_glow.locf";
    let separator = ','; 
    let mut indexer:usize = 0;
    let mut arraystore: [u32;3] = [0,0,0];
    //location files
    match read_file_into_vec_by_separator(filepath, separator) {
        Ok(data) => {
            println!("glow_locations ...");
            for (i, item) in data.iter().enumerate() {
                let mut item2 = item.parse::<u32>().unwrap();
                indexer +=1;
                
                
                match indexer{
                    1 => arraystore[0] = item2,
                    2 => arraystore[1] = item2,
                    3 => arraystore[2] = item2,

                    _ => print!("unexpected")
                }
                
                if indexer == 3{
                    
                    indexer = 0;
                    
                    drawNibbGlow(arraystore[0],arraystore[1],arraystore[2]);
               
                    
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }

    }
    
    imgbuf.lock().unwrap().save("output.png").unwrap();
    Ok(())



}






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


fn drawNibbGlow(ux:u32,uy:u32,uz:u32)  -> Result<(), Box<dyn std::error::Error>> {
    let mut filepath = "";
    filepath = "glow.nibb";
    let separator = ','; 
    let mut arraystore: [f32;8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut offset_x = ux;
    let mut offset_y = uy;
    let mut array_index:usize = 0;
    let mut indexer:usize = 0;
    let mut count:usize =0;
    match read_file_into_vec_by_separator(filepath, separator) {
        Ok(data) => {
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
    let mut new_color :Rgba<u8> = Rgba([r, g, b,a]);
    let mut pixel_color = get_pixel_color(&imgbuf.lock().unwrap(), ax, ay);
    let mut mix_pixel_color = mix_rgba_pixels(new_color,pixel_color);
    imgbuf.lock().unwrap().put_pixel(ax, ay, mix_pixel_color);
        

    
}
fn get_pixel_color(image: &RgbaImage, x: u32, y: u32) -> Rgba<u8> {
    let pixel = image.get_pixel(x, y);
    pixel.clone()
}

pub fn mix_rgba_pixels(foreground: Rgba<u8>, background: Rgba<u8>) -> Rgba<u8> {
    // Convert components to f32 for accurate floating-point calculations
    let fg_r = foreground[0] as f32;
    let fg_g = foreground[1] as f32;
    let fg_b = foreground[2] as f32;
    let fg_a = foreground[3] as f32 / 255.0; // Normalize alpha to 0.0-1.0

    let bg_r = background[0] as f32;
    let bg_g = background[1] as f32;
    let bg_b = background[2] as f32;
    let bg_a = background[3] as f32 / 255.0; // Normalize alpha to 0.0-1.0

    // Calculate the resulting alpha (A_out = A_fg + A_bg * (1 - A_fg))
    let out_a = fg_a + bg_a * (1.0 - fg_a);

    // Handle cases where the resulting alpha is zero or near-zero to prevent division by zero.
    // If the final pixel is fully transparent, its RGB components don't matter,
    // so we return transparent black.
    if out_a < f32::EPSILON { // Use a small epsilon for float comparisons
        return Rgba([0, 0, 0, 0]);
    }

    // Calculate the resulting RGB components (C_out = (C_fg * A_fg + C_bg * A_bg * (1 - A_fg)) / A_out)
    let out_r = (fg_r * fg_a + bg_r * bg_a * (1.0 - fg_a)) / out_a;
    let out_g = (fg_g * fg_a + bg_g * bg_a * (1.0 - fg_a)) / out_a;
    let out_b = (fg_b * fg_a + bg_b * bg_a * (1.0 - fg_a)) / out_a;

    // Convert back to u8, clamping values to 0-255 and rounding
    Rgba([
        out_r.round() as u8,
        out_g.round() as u8,
        out_b.round() as u8,
        (out_a * 255.0).round() as u8, // Convert normalized alpha back to 0-255
    ])
}
