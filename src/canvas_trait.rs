use std::boxed::Box;
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
    fn render_poly(&mut self, model: Box<Model>, multiplier: f64, offset: Vector3D, color: u32) {
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