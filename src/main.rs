#[macro_use] extern crate scan_fmt;
extern crate image;
extern crate termion;

use termion::{color};
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use image::{ImageBuffer, Rgb};

/** Image Size */
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

/** Image Colors */
const _RED:         Rgb<u8> = Rgb { data: [255, 0,   0] };
const _GREEN:       Rgb<u8> = Rgb { data: [0,   255, 0] };
const _BLUE:        Rgb<u8> = Rgb { data: [0,   0,   255] };
const _ICE:         Rgb<u8> = Rgb { data: [195, 203, 217] };
const _ICE_BLUE:    Rgb<u8> = Rgb { data: [7,   243, 229] };
const _WHITE:       Rgb<u8> = Rgb { data: [255, 255, 255] };
const _GRAY:        Rgb<u8> = Rgb { data: [128, 128, 128] };
const _DARK_GRAY:   Rgb<u8> = Rgb { data: [64,  64,  64] };
const _BLACK:       Rgb<u8> = Rgb { data: [0,   0,   0] };

/** Vertex coordinates definitions */
const _VERTEX_X: usize = 0;
const _VERTEX_Y: usize = 1;
const _VERTEX_Z: usize = 2;

/** Face point defenitions */
const _FACE_POINT_A: usize = 0;
const _FACE_POINT_B: usize = 1;

/** Pixel structure  definition */
#[derive(Clone)]
struct PixelT {
    x: i32,
    y: i32,
}

/** Project test images */
const _OBJ_AFRO_HEAD: &str = "obj/african_head.obj";
const _OBJ_ARTORIAS_SWORD: &str = "obj/artorias_sword.obj";
const _OBJ_FROSTMOURNE: &str = "obj/frostmourne.obj";
const _OBJ_VANGUARD: &str = "obj/vanguard.obj";
/** Current model: */
const MODEL_PATH: &str = _OBJ_AFRO_HEAD;
const PERCENTAGE_MODEL_SCALER: u32 = 90;

static mut MAX_VERTEX_ABS: f64 = 0.0;

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
fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start: PixelT, end: PixelT, color: Rgb<u8>) {
    let height = img.dimensions().1;
    let width = img.dimensions().0;
    let count = if (end.y - start.y).abs() > (end.x - start.x).abs() {
                    (end.y - start.y)
                } else {
                    (end.x - start.x)
                }.abs() as f64 + 1.0;
    //println!("[{},{}..{},{}] - {} iterations", start.x, start.y, end.x, end.y, count);
    let d_x = (end.x - start.x) as f64 / count;
    let d_y = (end.y - start.y) as f64 / count;

    for i in (0..(count) as u32).rev() {
        let x = start.x as f64 + i as f64 * d_x;
        let y = start.y as f64 + i as f64 * d_y;
        if x as u32 >= width || y as u32 >= height || x < 0.0 || y < 0.0 {
            continue;
        }
        img.get_pixel_mut(x as u32, y as u32).data = color.data;
    }
}

#[test]
fn out_of_image_bounds() {
    // a default (black) image containing Rgb values
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(100, 100);
    draw_line(&mut img, PixelT{x: -100, y: 0}, PixelT{x: 50, y: 50}, _WHITE);
    draw_line(&mut img, PixelT{x: 0, y: -100}, PixelT{x: 50, y: 50}, _WHITE);
    draw_line(&mut img, PixelT{x: 0, y: 0}, PixelT{x: 500, y: 50}, _WHITE);
    draw_line(&mut img, PixelT{x: 0, y: 0}, PixelT{x: 50, y: 500}, _WHITE);
}

#[test]
fn symmetry() {
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(100, 100);
    let start: PixelT = PixelT{x: 0, y: 0};
    let ends = [
        PixelT{x: 10, y: 90},
        PixelT{x: 30, y: 70},
        PixelT{x: 50, y: 50},
        PixelT{x: 70, y: 30},
        PixelT{x: 90, y: 10},
    ];

    for i in 0..ends.len()-1 {
        draw_line(&mut img, start.clone(), ends[i].clone(), _RED);
        draw_line(&mut img, ends[i].clone(), start.clone(), _GREEN);    
    }

    for x in (0..img.dimensions().0 as u32).rev() {
        for y in (0..img.dimensions().1 as u32).rev() {
            println!("{},{} - {}, {}, {}", x, y, img.get_pixel(x, y).data[0],
                                                img.get_pixel(x, y).data[1],
                                                img.get_pixel(x, y).data[2]);
            assert!(img.get_pixel_mut(x, y).data[0] != _RED[0]);
        }
    }
}

fn scan_vertex(line: &str, vertex: &mut Vec<[f64; 3]>) -> bool {
    let (x, y, z) = scan_fmt!(line, // input string
                    "v {} {} {}",  // format
                    f64, f64, f64);   // type of a-c Options
    if x.is_some() && y.is_some() && z.is_some() {
        // println!("Vertex: {} {} {}", x.unwrap(), y.unwrap(), z.unwrap() as f64);
        vertex.push([x.unwrap(), y.unwrap(), z.unwrap()]);
        unsafe {
            if x.unwrap().abs() > MAX_VERTEX_ABS {
                MAX_VERTEX_ABS = x.unwrap().abs();
            }
            if y.unwrap().abs() > MAX_VERTEX_ABS {
                MAX_VERTEX_ABS = y.unwrap().abs();
            }
            if z.unwrap().abs() > MAX_VERTEX_ABS {
                MAX_VERTEX_ABS = z.unwrap().abs();
            }
        }
        return true;
    }
    else {
        return false;
    }
}

fn scan_face(line: &str, faces: &mut Vec<[usize; 2]>) -> bool {
    let (a, b, c) = scan_fmt!(line, // input string
                    "f {}/{*}/{*} {}/{*}/{*} {}/{*}/{*}",  // format
                    usize, usize, usize);   // type of a-c Options
    if a.is_some() && b.is_some() && c.is_some() {
        //println!("Triangle: {}-{}-{}", a.unwrap(), b.unwrap(), c.unwrap());
        faces.push([a.unwrap(), b.unwrap()]);
        faces.push([b.unwrap(), c.unwrap()]);
        faces.push([c.unwrap(), a.unwrap()]);
        return true;
    }
    let (a, b, c) = scan_fmt!(line, // input string
                    "f {} {} {}",  // format
                    usize, usize, usize);   // type of a-c Options
    if a.is_some() && b.is_some() && c.is_some() {
        //println!("Triangle: {}-{}-{}", a.unwrap(), b.unwrap(), c.unwrap());
        faces.push([a.unwrap(), b.unwrap()]);
        faces.push([b.unwrap(), c.unwrap()]);
        faces.push([c.unwrap(), a.unwrap()]);
        return true;
    }
    return false;
}

fn parse_model_obj(file_path: &str) -> (Vec<[f64; 3]>, Vec<[usize; 2]>) {
    println!("Parse model {}", file_path);
    let mut vertex: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<[usize; 2]> = Vec::new();

    let line_count = BufReader::new(&File::open(file_path).unwrap()).lines().count();
    let f = File::open(file_path).unwrap();
    let file = BufReader::new(&f);
    println!("Lines: {}", line_count);
    let mut iter = 0;
    for line in file.lines() {
        let l = line.unwrap();
        scan_vertex(&l.clone(), &mut vertex);
        scan_face(&l.clone(), &mut faces);
        if iter % (10*line_count/100) == 0 {
            println!("{}Load progress: {}%", color::Fg(color::Yellow), iter / (line_count/100));
        }
        iter += 1;
    }
    print!("{}", color::Fg(color::White));
    unsafe {
        for i in 0..vertex.len() {
            vertex[i][_VERTEX_X] /= MAX_VERTEX_ABS;
            vertex[i][_VERTEX_Y] /= MAX_VERTEX_ABS;
            vertex[i][_VERTEX_Z] /= MAX_VERTEX_ABS;
        }
    }
    println!("Vertex: {}\nFaces: {}", vertex.len(), faces.len());
    return (vertex, faces);
}

fn main() {
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
    let (vertex, faces) = parse_model_obj(MODEL_PATH);

    for i in 0..faces.len() {
        let scaler: u32 = if WIDTH < HEIGHT { WIDTH } else { HEIGHT } * PERCENTAGE_MODEL_SCALER / 100;

        let vertex_a: usize = faces[i][_FACE_POINT_A] - 1;
        let vertex_b: usize = faces[i][_FACE_POINT_B] - 1;

        let vertex_a_x = vertex[vertex_a][_VERTEX_X] * scaler as f64/2.0 + WIDTH as f64/2.0;
        let vertex_a_y = vertex[vertex_a][_VERTEX_Y] * scaler as f64/2.0 + HEIGHT as f64/2.0;
        let vertex_b_x = vertex[vertex_b][_VERTEX_X] * scaler as f64/2.0 + WIDTH as f64/2.0;
        let vertex_b_y = vertex[vertex_b][_VERTEX_Y] * scaler as f64/2.0 + HEIGHT as f64/2.0;
        draw_line(&mut img, PixelT{x: vertex_a_x as i32, y: vertex_a_y as i32},
                            PixelT{x: vertex_b_x as i32, y: vertex_b_y as i32}, _WHITE);
        if i % (10*faces.len()/100) == 0 {
            println!("{}Process progress: {}%", color::Fg(color::Green), i / (faces.len()/100));
        }
    }
    print!("{}", color::Fg(color::White));

    // write it out to a file
    println!("Miror image");
    mirror_horizontal(&mut img);
    println!("Output image");
    img.save("output.png").unwrap();
}
