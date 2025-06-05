use std::{char, fs};
use std::io::{self, Write}; 
use plotters::chart::{self, ChartState};
use plotters::coord::cartesian;
use plotters::prelude::*;


use std::fs::File;
use std::io::BufWriter;



pub fn draw_pixel_cmyk(
    ax: f64,        // X-coordinate
    ay: f64,        // Y-coordinate
    acyan: f32,     // Cyan component (0.0 to 1.0)
    amagenta: f32,  // Magenta component (0.0 to 1.0)
    ayellow: f32,   // Yellow component (0.0 to 1.0)
    ak: f32,        // Black (Key) component (0.0 to 1.0)
    aalpha: f32,
) {


}


fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let mut by :f64;
    let mut bx : f64;
    let mut bcyan : f32;
    let mut bmagenta :f32;
    let mut byellow : f32;
    let mut bk : f32;
    let mut balpha: f32;
    let mut pixelNumber : u128;
    let mut maxDotts : u128 ;
    let mut colorRGBfromCMYK : String;
    maxDotts = 1000;


          
    pixelNumber = 0;
    while pixelNumber <maxDotts {
 
        by = 0.0;
        bx = 0.0;
        bcyan = 1.0;
        bmagenta = 1.0;
        byellow = 1.0;
        bck = 1.0;
        bcalpha = 1.0;
        pixelNumber +=1;
        
        println!("pixel.{pixelNumber}.x{by}.y{bx}c.{bcyan}m.{bmagenta}y.{byellow}k.{bck}a.{bcalpha}.end");
        draw_pixel_cmyk(by,bx,bcyan,bmagenta,byellow,bck,bcalpha);
    }
    Ok(())
}
fn creator(){
    
    let cy = 0.0;
    let cx = 0.0;
    let ccyan = 1.0;
    let cmagenta = 1.0;
    let cyellow = 1.0;
    let ck = 1.0;
    let ccalpha = 0.0;
    let pixelNumber =1;
    let mut datastring = String::new();
    let maxDotts: u128=1000;
    while pixelNumber<maxDotts {
        
    
    datastring += &pixelNumber.to_string();
    datastring += "p";
    datastring += &cy.to_string();
    datastring += "y";
    datastring += &cx.to_string();
    datastring += "x";
    datastring += &ccyan.to_string();
    datastring += "c";
    datastring += &cmagenta.to_string();
    datastring += "m";
    datastring += &cyellow.to_string();
    datastring += "y";
    datastring += &ck.to_string();
    datastring += "k";
    datastring += &calpha.to_string();
    datastring += "a";  
    
    }
    println!("{datastring}");
}

