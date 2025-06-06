use image::{RgbImage, Rgb};
pub fn creator(ax:u32,ay:u32,az:u32,c:f32,m:f32,y:f32,k:f32,a:f32){
    let r = (255.0 * (1.0 - c) * (1.0 - k)) as u8;
    let g = (255.0 * (1.0 - m) * (1.0 - k)) as u8;
    let b = (255.0 * (1.0 - y) * (1.0 - k)) as u8;


    let mut imgbuf = RgbImage::new(32, 32);
    imgbuf.put_pixel(ax, ay, Rgb([r, g, b]));
            

    imgbuf.save("output.png").unwrap();
}