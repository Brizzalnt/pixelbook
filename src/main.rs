use std::fs;
use std::io::{self, Write}; 
use plotters::prelude::*;

use gif::{Encoder, Frame, Repeat};
use std::fs::File;
use std::io::BufWriter;


#[derive(Debug)]
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
struct CmykColor {
    cyan_c: f32,    
    magenta_c: f32,
    yellow_c: f32,
    key_c: f32,    
}
fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let mut pixelNumber : u128;
    let mut maxDotts : u128 ;
    let mut colorRGBfromCMYK : String;
    maxDotts = 1000;


          
    pixelNumber = 0;
    while pixelNumber <maxDotts {
        pixelNumber +=1;
        let mut _pixel = PixelData{ alpha : 255, cyan: 1.0, magenta: 1.0, yellow:1.0 , key :1.0 , shine: 0,  x:0.0 , y:0.0 , z:0.0, shape:"pixel".to_string()};
        println!("{:?}", _pixel);
    }
    Ok(())
}


