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
    let mut _pixel = PixelData{ alpha : 255, red: 0, green: 0, blue:0 , emit:0 , shine: 0,  x:0.0 , y:0.0 , z:0.0};
    println!("{:?}", _pixel);
}
