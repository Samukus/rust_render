use std::boxed::Box;
use std::ops::Mul;
use std::mem;
use crate::geometry::*;
use crate::model_trait::Model;

extern crate rand;
use rand::Rng;

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
            self.triangle_wire(triangle, color).unwrap();
        }
    }
    fn render_poly_lightning(&mut self, model: Box<Model>, multiplier: f64, offset: Vector3D, color: RgbColor) {
        let light_vec = Vector3D {x: 0.0, y: 0.0, z: -1.0};
        for elem in model.triangle_iter() {
            let triangle = (elem.clone() * multiplier) + offset.clone();
            let n = triangle.normal();
            let intensity = light_vec.scalar(n);
            if intensity < 0.0 {
                continue;
            }
            let trg_color = color * intensity;
            self.triangle_colored(triangle, trg_color.value()).unwrap();
        }
    }
    fn render_poly_rnd_colored(&mut self, model: Box<Model>, multiplier: f64, offset: Vector3D) {
        let mut rng = rand::thread_rng();
        for elem in model.triangle_iter() {
            let triangle = (elem.clone() * multiplier) + offset.clone();
            let color = rng.gen_range(0x000000, 0xFFFFFF);
            self.triangle_colored(triangle, color).unwrap();
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
    fn triangle_wire(&mut self, trgl: Triangle, color: u32) -> Result<(), String> {
        self.line(trgl.p0.x as i32, trgl.p0.y as i32, trgl.p1.x as i32, trgl.p1.y as i32, color)?;
        self.line(trgl.p1.x as i32, trgl.p1.y as i32, trgl.p2.x as i32, trgl.p2.y as i32, color)?;
        self.line(trgl.p2.x as i32, trgl.p2.y as i32, trgl.p0.x as i32, trgl.p0.y as i32, color)?;
        Ok(())
    }
    fn triangle_colored(&mut self, trgl: Triangle, color: u32) -> Result<(), String> {
        let mut t0 = trgl.p0;
        let mut t1 = trgl.p1;
        let mut t2 = trgl.p2;

        if t0.y == t1.y && t0.y == t2.y {
            return Ok(()); // i dont care about degenerate triangles
        }
        // sort the vertices, t0, t1, t2 lower-to-upper (bubblesort yay!)
        if t0.y > t1.y {
            mem::swap(&mut t0, &mut t1);
        }
        if t0.y > t2.y {
            mem::swap(&mut t0, &mut t2);
        }
        if t1.y > t2.y {
            mem::swap(&mut t1, &mut t2);
        }
        let total_height: usize = (t2.y - t0.y) as usize;
        self.line(t0.x as i32, t0.y as i32, t1.x as i32, t1.y as i32, color)?;
        self.line(t1.x as i32, t1.y as i32, t2.x as i32, t2.y as i32, color)?;
        self.line(t2.x as i32, t2.y as i32, t0.x as i32, t0.y as i32, color)?;
        for i in 0..total_height as usize {
            let second_half = i as f64 > t1.y - t0.y || t1.y==t0.y;
            let segment_height: usize = if second_half  { (t2.y - t1.y) as usize } else { (t1.y - t0.y) as usize };
            let alpha = i as f64 / total_height as f64;
            let beta  = (i as f64 - if second_half { t1.y - t0.y } else { 0.0 }) / segment_height as f64; // be careful: with above conditions no division by zero here
            let mut a = t0 + (t2 - t0) * alpha;
            let mut b = if second_half { t1 + (t2 - t1) * beta } else { t0 + (t1 - t0) * beta };
            if a.x > b.x {
                mem::swap(&mut a, &mut b);
            }
            for j in a.x as usize .. (b.x as usize + 1) {
                self.set(j as i32, t0.y as i32 + i as i32, color).unwrap();
            }
        }
        Ok(())
    }
    fn triangle_z_buffered(&mut self, trgl: Triangle) -> Result<(), String> {
        let mut p0 = trgl.p0;
        let mut p1 = trgl.p1;
        let mut p2 = trgl.p2;

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
        if (p0.x > p1.x) {
            mem::swap(&mut p0, &mut p1);
        }
        if p1.y != p0.y {
            println!("Wrong points");
            return Err(("Wrong points".to_string())); // i dont care about degenerate triangles
        }
        self.line(p0.x as i32, p0.y as i32, p1.x as i32, p1.y as i32, 0xFFFFFF)?;
        self.line(p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32, 0xFFFFFF)?;
        self.line(p2.x as i32, p2.y as i32, p0.x as i32, p0.y as i32, 0xFFFFFF)?;
        let N: usize = (p2.y - p0.y) as usize;

        let dxLeft  = (p0.x - p2.x) / (N as f64);
        let dxRight = (p1.x - p2.x) / (N as f64);

        let dzLeft  = (p0.z - p2.z) / (N as f64);
        let dzRight = (p1.z - p2.z) / (N as f64);

        println!("p0 {},{},{} p1 {},{},{}", p0.x, p0.y, p0.z, p1.x, p1.y, p1.z);
        println!("dxLeft {}, dxRight {}", dxLeft, dxRight);
        println!("dzLeft {}, dzRight {}", dzLeft, dzRight);

        for i in 0..(N+1) {
            let xLeft = (p2.x + dxLeft * (i as f64));
            let xRight = (p2.x + dxRight * (i as f64));

            let zLeft = (p2.z + dzLeft * (i as f64));
            let zRight = (p2.z + dzRight * (i as f64));
            let dZinternal = (zRight - zLeft) / (xRight - xLeft);

            let y = p2.y - i as f64;
            println!("line: x {}..{}; z {}..{}", xLeft, xRight, zLeft, zRight);
            for x in xLeft as usize .. xRight as usize + 1 {
                let z = zLeft + dZinternal * x as f64;
                let color_value: u8 = (255.0 * (z/1000.0)) as u8;
                let color = RgbColor{red: color_value, green: color_value, blue: color_value};
                self.set(x as i32, y as i32, color.value()).unwrap();
                println!("set: [{},{},{} color value {}]", x,y,z, color_value);
            }
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