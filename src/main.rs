use std::{char, fs};
use std::io::{self, Write}; 

mod files;
mod nibber;
use std::fs::File;
use std::io::BufWriter;



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

    files::run();
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
    cx = 1;
    cy = 1;
    let ccyan = 1.0;
    let cmagenta = 1.0;
    let cyellow = 1.0;
    let ck = 1.0;
    let calpha = 0.0;
    let mut pixelNumber =1;

          
    pixelNumber = 0;
    while pixelNumber <maxDotts {
         pixelNumber+=1;
              
        println!("done.");
        draw_pixel_cmyk(cy,cx,ccyan,cmagenta,cyellow,ck,calpha);
    }
    nibber::draw_pixel(cx,cy);
    Ok(())
}


    