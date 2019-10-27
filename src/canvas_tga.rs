use image::{ImageBuffer, Rgb};
use crate::canvas_trait::Canvas;
use crate::geometry::*;

pub struct TgaCanvas {
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
}

impl Canvas for TgaCanvas {
    fn set(&mut self, point: Vector3D, color: u32) -> Result<(), String> {
        let x = point.x.round() as i32;
        let y = point.y.round() as i32;
        let _z = point.z.round() as i32;
        if x as u32 >= self.width || y as u32 >= self.height || x < 0 || y < 0 {
            return Err("Out of bounds coordinates".to_string());
        }
        let rgb_color: Rgb<u8> = Rgb { data: [(color >> (8*2)) as u8,
                                              (color >> (8*1)) as u8,
                                              (color as u8)] };
        self.image.get_pixel_mut(x as u32, ((self.height-1) as i32 - y) as u32).data = rgb_color.data;
        Ok(())
    }
    fn get(&self, _x: i32, _y: i32) -> Result<u32, String> {
        Ok(0)
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn new(width: u32, height: u32) -> Self {
        let img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
        TgaCanvas {
            image: img,
            width: width,
            height: height,
        }
    }
    fn out(&mut self) -> Result<(), String> {
        self.image.save("output.png").unwrap();
        Ok(())
    }
}

#[test]
fn out_of_image_bounds() {
    use crate::canvas_trait::{RgbColor, Ergbcolor};
    let mut canvas: TgaCanvas = Canvas::new(100, 100);
    match canvas.line( -100, 0, 50, 50, RgbColor::new(Ergbcolor::WHITE).value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", -100, 0, 50, 50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", -100, 0, 50, 50, err),
    }
    match canvas.line(0, -100, 50, 50, RgbColor::new(Ergbcolor::WHITE).value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, -100, 50, 50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, -100, 50, 50, err),
    }
    match canvas.line(0, 0, 500,50, RgbColor::new(Ergbcolor::WHITE).value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, 0, 500,50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, 0, 500, 50, err),
    }
    match canvas.line(0, 0, 50, 500, RgbColor::new(Ergbcolor::WHITE).value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, 0, 50, 500),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, 0, 50, 500, err),
    }
}