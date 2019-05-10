extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::canvas_trait::Canvas;

pub struct SdlCanvas {
    renderer: WindowCanvas,
    sdl_context: Sdl,
    width: u32,
    height: u32,
}

impl Canvas for SdlCanvas {
    fn set(&mut self, x: i32, y: i32, color: u32) -> Result<(), String> {
        self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
        self.renderer.draw_point(Point::new(x, (self.height-1) as i32 - y))?;
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
        SdlCanvas {
            renderer: renderer,
            sdl_context: sdl_context,
            width: width,
            height: height,
        }
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
    use crate::canvas_trait::RgbColor;
    let mut canvas: SdlCanvas = Canvas::new(100, 100);
    match canvas.line( -100, 0, 50, 50, RgbColor::WHITE.value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", -100, 0, 50, 50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", -100, 0, 50, 50, err),
    }
    match canvas.line(0, -100, 50, 50, RgbColor::WHITE.value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, -100, 50, 50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, -100, 50, 50, err),
    }
    match canvas.line(0, 0, 500,50, RgbColor::WHITE.value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, 0, 500,50),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, 0, 500, 50, err),
    }
    match canvas.line(0, 0, 50, 500, RgbColor::WHITE.value()) {
        Result::Ok(_val) => println!("Line {},{}..{},{} is OK", 0, 0, 50, 500),
        Result::Err(err) => println!("Line {},{}..{},{} is failed: {}", 0, 0, 50, 500, err),
    }
}

/* Pixel structure  definition */
// #[derive(Clone)]
// struct PixelT {
//     x: i32,
//     y: i32,
// }
// #[test]
// fn symmetry() {
//     let mut canvas: SdlCanvas = Canvas::new(100, 100);
//     let start: PixelT = PixelT{x: 0, y: 0};
//     let ends = [
//         PixelT{x: 10, y: 90},
//         PixelT{x: 30, y: 70},
//         PixelT{x: 50, y: 50},
//         PixelT{x: 70, y: 30},
//         PixelT{x: 90, y: 10},
//     ];

//     for i in 0..ends.len()-1 {
//         draw_line(&mut img, start.clone(), ends[i].clone(), _RED);
//         draw_line(&mut img, ends[i].clone(), start.clone(), _GREEN);    
//     }

//     for x in (0..img.dimensions().0 as u32).rev() {
//         for y in (0..img.dimensions().1 as u32).rev() {
//             println!("{},{} - {}, {}, {}", x, y, img.get_pixel(x, y).data[0],
//                                                 img.get_pixel(x, y).data[1],
//                                                 img.get_pixel(x, y).data[2]);
//             assert!(img.get_pixel_mut(x, y).data[0] != _RED[0]);
//         }
//     }
// }