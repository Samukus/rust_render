#[macro_use] extern crate scan_fmt;
extern crate image;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use image::{ImageBuffer, Rgb};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const _RED:      Rgb<u8> = Rgb { data: [255, 0,   0] };
const _GREEN:    Rgb<u8> = Rgb { data: [0,   255, 0] };
const _BLUE:     Rgb<u8> = Rgb { data: [0,   0,   255] };
const _WHITE:    Rgb<u8> = Rgb { data: [255, 255, 255] };
const _BLACK:    Rgb<u8> = Rgb { data: [0,   0,   0] };
const MODEL_PATH: &str = "obj/african_head.obj";

#[derive(Clone)]
struct PixelT {
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
                    "v {f} {f} {f}",  // format
                    f64, f64, f64);   // type of a-c Options
    if x != None && y != None && z != None {
        //println!("Vertex: {} {} {}", x.unwrap(), y.unwrap(), z.unwrap());
        vertex.push([x.unwrap(), y.unwrap(), z.unwrap()]);
        return true;
    }
    else {
        return false;
    }
}

fn scan_face(line: &str, faces: &mut Vec<[usize; 2]>) -> bool {
    let (a, b, c) = scan_fmt!(line, // input string
                    "f {d}/{*d}/{*d} {d}/{*d}/{*d} {d}/{*d}/{*d}",  // format
                    usize, usize, usize);   // type of a-c Options
    if a != None && b != None && c != None {
        //println!("Triangle: {}-{}-{}", a.unwrap(), b.unwrap(), c.unwrap());
        faces.push([a.unwrap(), b.unwrap()]);
        faces.push([b.unwrap(), c.unwrap()]);
        faces.push([c.unwrap(), a.unwrap()]);
        return true;
    }
    else {
        return false;
    }               
}

fn parse_model_obj(file_path: &str) -> (Vec<[f64; 3]>, Vec<[usize; 2]>) {
    let mut vertex: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<[usize; 2]> = Vec::new();

    let f = File::open(file_path).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        scan_vertex(&l.clone(), &mut vertex);
        scan_face(&l.clone(), &mut faces);
    }
    return (vertex, faces);
}

fn main() {
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
    let (vertex, faces) = parse_model_obj(MODEL_PATH);
    println!("Vertex: {}\nFaces: {}", vertex.len(), faces.len());

    for i in 0..faces.len()-1 {
        let vertex_a_x = vertex[faces[i][0] - 1][0] * WIDTH as f64/2.0 + WIDTH as f64/2.0;
        let vertex_a_y = vertex[faces[i][0] - 1][1] * HEIGHT as f64/2.0 + HEIGHT as f64/2.0;
        let vertex_b_x = vertex[faces[i][1] - 1][0] * WIDTH as f64/2.0 + WIDTH as f64/2.0;
        let vertex_b_y = vertex[faces[i][1] - 1][1] * HEIGHT as f64/2.0 + HEIGHT as f64/2.0;
        draw_line(&mut img, PixelT{x: vertex_a_x as i32, y: vertex_a_y as i32},
                            PixelT{x: vertex_b_x as i32, y: vertex_b_y as i32}, _WHITE);
    }

    // write it out to a file
    mirror_horizontal(&mut img);
    img.save("output.png").unwrap();
}
