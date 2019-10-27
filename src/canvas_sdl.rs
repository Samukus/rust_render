extern crate sdl2;
extern crate ndarray;

use ndarray::Array;
use ndarray::Array2;
use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::canvas_trait::Canvas;
use crate::geometry::*;

pub struct SdlCanvas {
    renderer: WindowCanvas,
    sdl_context: Sdl,
    width: u32,
    height: u32,
    z_buffer: Array2<i32>,
}

impl Canvas for SdlCanvas {
    fn set(&mut self, point: Vector3D, color: u32) -> Result<(), String> {
        let x = point.x.round();
        let y = point.y.round();
        let z = point.z.round();
        if     x >= self.width  as f64 || x < 0.0
            || y >= self.height as f64 || y < 0.0
        {
            return Ok(());
        }
        // Check Z buffer
        if z as i32 >= self.z_buffer[[x as usize, y as usize]] {
            self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
            let gray_level: u8 = (255.0 * (z/1500.0)) as u8;
            self.renderer.set_draw_color(Color::RGB(gray_level, gray_level, gray_level));
            self.renderer.draw_point(Point::new(x as i32, (self.height-1) as i32 - y as i32))?;
            self.z_buffer[[x as usize, y as usize]] = z as i32;
            // println!("Set: {},{}", x, y);
        }
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
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-3d-renderer", width, height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string()).unwrap();

        let renderer = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();
        let mut canvas = SdlCanvas {
            renderer: renderer,
            sdl_context: sdl_context,
            width: width,
            height: height,
            z_buffer: Array::zeros((width as usize, height as usize)),
        };
        for mut row in canvas.z_buffer.genrows_mut() {
                row.fill(std::i32::MIN);
        }
        return canvas;
    }

    fn out(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        self.renderer.present();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

#[test]
fn out_of_image_bounds() {
    use crate::canvas_trait::{RgbColor, Ergbcolor};
    let mut canvas: SdlCanvas = Canvas::new(100, 100);
    match canvas.line( Vector3D {x: -100.0, y: 0.0, z: 0.0},
                       Vector3D {x: 50.0, y: 50.0, z: 0.0},
                       RgbColor::new(Ergbcolor::WHITE).value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", -100, 0, 50, 50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", -100, 0, 50, 50, err),
    }
    match canvas.line( Vector3D {x: 0.0, y: -100.0, z: 0.0},
                       Vector3D {x: 50.0, y: 50.0, z: 0.0},
                       RgbColor::new(Ergbcolor::WHITE).value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, -100, 50, 50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, -100, 50, 50, err),
    }
    match canvas.line( Vector3D {x: 0.0, y: 0.0, z: 0.0},
                       Vector3D {x: 500.0, y: 50.0, z: 0.0},
                       RgbColor::new(Ergbcolor::WHITE).value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, 0, 500,50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, 0, 500, 50, err),
    }
    match canvas.line( Vector3D {x: 0.0, y: 0.0, z: 0.0},
                       Vector3D {x: 50.0, y: 500.0, z: 0.0},
                       RgbColor::new(Ergbcolor::WHITE).value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, 0, 50, 500),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, 0, 50, 500, err),
    }
}