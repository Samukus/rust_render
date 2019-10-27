use std::boxed::Box;
use std::ops::Mul;
use std::mem;
use crate::geometry::*;
use crate::model_trait::Model;

extern crate rand;
use rand::Rng;

pub trait Canvas {
    fn set(&mut self, point: Vector3D, color: u32) -> Result<(), String>;
    fn get(&self, x: i32, y: i32) -> Result<u32, String>;
    fn new(x: u32, y: u32) -> Self;
    fn out(&mut self) -> Result<(), String>;
    fn get_height(&self) -> u32;
    fn get_width(&self) -> u32;
    fn render_wire(&mut self, model: Box<Model>, multiplier: f64, offset: Vector3D, color: RgbColor) {
        for elem in model.triangle_iter() {
            let triangle = (elem.clone() * multiplier) + offset.clone();
            self.triangle_wire(triangle, color.value()).unwrap();
        }
    }
    fn render_poly(&mut self, model: Box<Model>, multiplier: f64, offset: Vector3D, color: RgbColor, light_vec: Vector3D) {
        for elem in model.triangle_iter() {
            let triangle = (elem.clone() * multiplier) + offset.clone();
            let n = triangle.normal();
            let intensity = light_vec.scalar(n);
            let trg_color = color * intensity;
            self.triangle(triangle.clone(), trg_color.value()).unwrap();
        }
    }
    fn render_poly_rnd_colored(&mut self, model: Box<Model>, multiplier: f64, offset: Vector3D) {
        let mut rng = rand::thread_rng();
        for elem in model.triangle_iter() {
            let triangle = (elem.clone() * multiplier) + offset.clone();
            let color = rng.gen_range(0x000000, 0xFFFFFF);
            self.triangle(triangle, color).unwrap();
        }
    }
    fn line(&mut self, p0: Vector3D, p1: Vector3D, color: u32) -> Result<(), String> {
        let a = p0;
        let b = p1;
        let count = if (b.y - a.y).abs() > (b.x - a.x).abs() {
                        (b.y - a.y)
                    } else {
                        (b.x - a.x)
                    }.abs();
        // println!("[{},{}..{},{}] - {} iterations", a.x, a.y, b.x, b.y, count);
        let delta = (b - a) / count;

        for i in 0..count as u32 {
            let point = a + delta * i as f64;
            self.set(point.round(), color)?;
        }
        Ok(())
    }
    fn triangle_wire(&mut self, trgl: Triangle, color: u32) -> Result<(), String> {
        self.line(trgl.p0, trgl.p1, color)?;
        self.line(trgl.p1, trgl.p2, color)?;
        self.line(trgl.p2, trgl.p0, color)?;
        Ok(())
    }
    fn triangle(&mut self, trgl: Triangle, color: u32) -> Result<(), String> {
        let mut p0 = trgl.p0.round();
        let mut p1 = trgl.p1.round();
        let mut p2 = trgl.p2.round();

        // println!("Trgl: {}, {}, {}", p0, p1, p2);

        if p0.y == p1.y && p0.y == p2.y {
            return Ok(()); // i dont care about degenerate triangles
        }
        // sort the vertices, p0, p1, p2 lower-to-upper (bubblesort yay!)
        if p0.y > p1.y {
            mem::swap(&mut p0, &mut p1);
        }
        if p0.y > p2.y {
            mem::swap(&mut p0, &mut p2);
        }
        if p1.y > p2.y {
            mem::swap(&mut p1, &mut p2);
        }

        let n_full = (p2.y - p0.y).round();
        let n_from_2_to_1 = (p2.y - p1.y).round();
        let n_from_1_to_0 = (p1.y - p0.y).round();
        let alpha: Vector3D = (p0 - p2) / n_full;
        let p4: Vector3D = p2 + alpha * n_from_2_to_1;

        for i in 0..n_from_2_to_1 as usize {
            // Top half
            let beta = (p1 - p2) / (n_from_2_to_1 as f64);
            let p0_internal: Vector3D = p2 + (beta * (i as f64));;
            let p1_internal: Vector3D = p2 + (alpha * (i as f64));
            self.line(p0_internal.round(), p1_internal.round(), color)?;
        }

        for i in 0..n_from_1_to_0 as usize {
            // Bottom half
            let beta = (p0 - p1) / (n_from_1_to_0 as f64);
            let p0_internal: Vector3D = p1 + (beta * (i as f64));;
            let p1_internal: Vector3D = p4 + (alpha * (i as f64));
            self.line(p0_internal.round(), p1_internal.round(), color)?;
        }
        Ok(())
    }
}

/** Image Colors */
#[allow(dead_code)]
pub enum Ergbcolor {
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

#[derive(Clone, Copy)]
pub struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RgbColor {
    #[allow(dead_code)]
    pub fn new(color: Ergbcolor) -> Self {
        match color {
            Ergbcolor::RED =>       RgbColor{red: 0xFF, green: 0x00, blue: 0x00}, // 0xFF0000
            Ergbcolor::GREEN =>     RgbColor{red: 0x00, green: 0xFF, blue: 0x00}, // 0x00FF00
            Ergbcolor::BLUE =>      RgbColor{red: 0xFF, green: 0x00, blue: 0xFF}, // 0x0000FF
            Ergbcolor::ICE =>       RgbColor{red: 0xC3, green: 0xCB, blue: 0xD9}, // 0xC3CBD9
            Ergbcolor::ICEBLUE =>   RgbColor{red: 0x07, green: 0xF3, blue: 0xE5}, // 0x07F3E5
            Ergbcolor::WHITE =>     RgbColor{red: 0xFF, green: 0xFF, blue: 0xFF}, // 0xFFFFFF
            Ergbcolor::GRAY =>      RgbColor{red: 0x80, green: 0x80, blue: 0x80}, // 0x808080
            Ergbcolor::DARKGRAY =>  RgbColor{red: 0x40, green: 0x40, blue: 0x40}, // 0x404040
            Ergbcolor::BLACK =>     RgbColor{red: 0x00, green: 0x00, blue: 0x00}, // 0x000000
        }
    }
    pub fn value(&self) -> u32 {
        let mut result: u32 = self.red as u32 * 256;
        result = (result + self.green as u32) * 256;
        (result + self.blue as u32) 
    }
}

impl Mul<f64> for RgbColor {
    type Output = RgbColor;
    fn mul(self, k: f64) -> Self {
        RgbColor {
            red: (self.red as f64 * k) as u8,
            green: (self.green as f64 * k) as u8,
            blue: (self.blue as f64 * k) as u8,
        }
    }
}

#[test]
fn test_rgb() {
    let rgb: RgbColor = RgbColor {red: 255, green: 255, blue: 255};
    println!("Rgb: {:06X}", rgb.value());
    let rgb: RgbColor = RgbColor {red: 255, green: 0, blue: 0};
    println!("Rgb: {:06X}", rgb.value());
    let rgb: RgbColor = RgbColor {red: 0, green: 255, blue: 0};
    println!("Rgb: {:06X}", rgb.value());
    let rgb: RgbColor = RgbColor {red: 0, green: 0, blue: 255};
    println!("Rgb: {:06X}", rgb.value());
    let rgb: RgbColor = RgbColor {red: 16, green: 16, blue: 16};
    println!("Rgb: {:06X}", rgb.value());
}