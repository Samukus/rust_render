use std::boxed::Box;
use crate::geometry::*;
use crate::model_trait::Model;

pub trait Canvas {
    fn set(&mut self, x: i32, y: i32, color: u32) -> Result<(), String>;
    fn get(&self, x: i32, y: i32) -> Result<u32, String>;
    fn new(x: u32, y: u32) -> Self;
    fn out(&mut self) -> Result<(), String>;
    fn get_height(&self) -> u32;
    fn get_width(&self) -> u32;
    fn render_wire(&mut self, model: Box<Model>, multiplier: f64, offset: Vector3D, color: u32) {
        for elem in model.triangle_iter() {
            let triangle = (elem.clone() * multiplier) + offset.clone();
            self.line(triangle.p1.x as i32, triangle.p1.y as i32, triangle.p2.x as i32, triangle.p2.y as i32, color).unwrap();
            self.line(triangle.p2.x as i32, triangle.p2.y as i32, triangle.p3.x as i32, triangle.p3.y as i32, color).unwrap();
            self.line(triangle.p3.x as i32, triangle.p3.y as i32, triangle.p1.x as i32, triangle.p1.y as i32, color).unwrap();
        }
    }
    fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) -> Result<(), String> {
        let count = if (y1 - y0).abs() > (x1 - x0).abs() {
                        (y1 - y0)
                    } else {
                        (x1 - x0)
                    }.abs() as f64 + 1.0;
        debug!("[{},{}..{},{}] - {} iterations", x0, y0, x1, y1, count);
        let d_x = (x1 - x0) as f64 / count;
        let d_y = (y1 - y0) as f64 / count;

        for i in 0..count as u32 {
            let x = x0 as f64 + i as f64 * d_x;
            let y = y0 as f64 + i as f64 * d_y;
            self.set(x as i32, y as i32, color)?;
        }
        Ok(())
    }
}

/** Image Colors */
#[allow(dead_code)]
pub enum RgbColor {
    RED,
    GREEN,
    BLUE,
    ICE,
    ICEBLUE,
    WHITE,
    GRAY,
    DARKGRAY,
    BLACK,
}

impl RgbColor {
    pub fn value(&self) -> u32 {
        match *self {
            RgbColor::RED => 0xFF0000,
            RgbColor::GREEN => 0x00FF00,
            RgbColor::BLUE => 0x0000FF,
            RgbColor::ICE => 0xC3CBD9,
            RgbColor::ICEBLUE => 0x07F3E5,
            RgbColor::WHITE => 0xFFFFFF,
            RgbColor::GRAY => 0x808080,
            RgbColor::DARKGRAY => 0x404040,
            RgbColor::BLACK => 0x000000,
        }
    }
}