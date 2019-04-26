extern crate image;

use image::{ImageBuffer, Rgb};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

#[derive(Clone)]
struct Pixel {
    x: i32,
    y: i32,
}

fn mirror_horizontal(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let height = img.dimensions().1;
    let width = img.dimensions().0;
    for y in (0..(height-1)/2).rev() {
        for x in (0..width-1).rev() {
            let tmp = img.get_pixel_mut(x, y).data;
            img.get_pixel_mut(x, y).data = img.get_pixel_mut(x, height-1 - y).data;
            img.get_pixel_mut(x, height-1 - y).data = tmp;
        }
    }
}



fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start: Pixel, end: Pixel) {
    //img.get_pixel_mut(start.x, start.y).data = [255, 255, 255];
    //img.get_pixel_mut(end.x, end.y).data = [255, 255, 255];

    let count = if (end.y - start.y).abs() > (end.x - start.x).abs() {
                    (end.y - start.y).abs() as f64
                }
                else {
                    (end.x - start.x).abs() as f64
                };
    println!("[{},{}..{},{}] - {} iterations", start.x, start.y, end.x, end.y, count);
    let d_x = (end.x - start.x) as f64 / count;
    let d_y = (end.y - start.y) as f64 / count;

    for i in (0..count as u32).rev() {
        let x = (start.x as f64 + i as f64 * d_x) as u32;
        let y = (start.y as f64 + i as f64 * d_y) as u32;
        img.get_pixel_mut(x, y).data = [255, 255, 255];
    }
}

fn main() {
    // a default (black) image containing Rgb values
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);

    let k = 3;
    let c_x = 200;
    let c_y = 200;

    let p0 = Pixel{x: 0*k + c_x, y: 2*k + c_y};
    let p1 = Pixel{x: 1*k + c_x, y: 4*k + c_y};
    let p5 = Pixel{x: 1*k + c_x, y: 0*k + c_y};
    let p6 = Pixel{x: 2*k + c_x, y: 2*k + c_y};
    let p2 = Pixel{x: 3*k + c_x, y: 4*k + c_y};
    let p4 = Pixel{x: 3*k + c_x, y: 0*k + c_y};
    let p3 = Pixel{x: 4*k + c_x, y: 2*k + c_y};

    draw_line(&mut img, p0.clone(), p1.clone());
    draw_line(&mut img, p1.clone(), p2.clone());
    draw_line(&mut img, p2.clone(), p3.clone());
    draw_line(&mut img, p3.clone(), p4.clone());
    draw_line(&mut img, p4.clone(), p5.clone());
    draw_line(&mut img, p5.clone(), p0.clone());

    draw_line(&mut img, p0.clone(), p6.clone());
    draw_line(&mut img, p1.clone(), p6.clone());
    draw_line(&mut img, p2.clone(), p6.clone());
    draw_line(&mut img, p3.clone(), p6.clone());
    draw_line(&mut img, p4.clone(), p6.clone());
    draw_line(&mut img, p5.clone(), p6.clone());

    // write it out to a file
    mirror_horizontal(&mut img);
    img.save("output.png").unwrap();
}
