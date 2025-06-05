use std::{char, fs};
use std::io::{self, Write}; 
use plotters::chart::{self, ChartState};
use plotters::coord::cartesian;
use plotters::prelude::*;


use std::fs::File;
use std::io::BufWriter;



pub fn draw_pixel_cmyk(
    ozx: f64,        // X-coordinate
    ozy: f64,        // Y-coordinate
    ozcyan: f32,     // Cyan component (0.0 to 1.0)
    ozmagenta: f32,  // Magenta component (0.0 to 1.0)
    ozyellow: f32,   // Yellow component (0.0 to 1.0)
    ozk: f32,        // Black (Key) component (0.0 to 1.0)
    ozalpha: f32,
) {


}
struct PixelData{
    alpha : u8,
    cyan : f32,
    magenta : f32,
    yellow : f32,
    key: f32,
    shine : u8,
    x: f64,
    y: f64,
    z: f64,
    shape : String,
}

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let mut ocy :f64;
    let mut ocx : f64;
    let mut occyan : f32;
    let mut ocmagenta :f32;
    let mut ocyellow : f32;
    let mut ock : f32;
    let mut ocalpha: f32;
    let mut pixelNumber : u128;
    let mut maxDotts : u128 ;
    let mut colorRGBfromCMYK : String;
    maxDotts = 1000;


          
    pixelNumber = 0;
    while pixelNumber <maxDotts {
 
        ocy = 0.0;
        ocx = 0.0;
        occyan = 1.0;
        ocmagenta = 1.0;
        ocyellow = 1.0;
        ock = 1.0;
        ocalpha = 1.0;
        pixelNumber +=1;
        
        println!("pixel.{pixelNumber}.to_string()");
        draw_pixel_cmyk(ocy,ocx,occyan,ocmagenta,ocyellow,ock,ocalpha);
    }
    Ok(())
}


