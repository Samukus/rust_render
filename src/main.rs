#[macro_use] extern crate scan_fmt;
#[macro_use] extern crate log;

extern crate image;

mod canvas_sdl;
mod canvas_tga;
mod canvas_trait;

use canvas_trait::{Canvas, RgbColor};
use canvas_sdl::SdlCanvas;
#[allow(unused_imports)]
use canvas_tga::TgaCanvas;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

/** Image Size */
const WIDTH: u32 = 1600;
const HEIGHT: u32 = 800;

/** Vertex coordinates definitions */
const _VERTEX_X: usize = 0;
const _VERTEX_Y: usize = 1;
const _VERTEX_Z: usize = 2;

/** Face point defenitions */
const _FACE_POINT_A: usize = 0;
const _FACE_POINT_B: usize = 1;

/** Project test images */
#[allow(dead_code)]
const OBJ_AFRO_HEAD: &str = "obj/african_head.obj";
#[allow(dead_code)]
const OBJ_ARTORIAS_SWORD: &str = "obj/artorias_sword.obj";
#[allow(dead_code)]
const OBJ_FROSTMOURNE: &str = "obj/frostmourne.obj";
#[allow(dead_code)]
const OBJ_VANGUARD: &str = "obj/vanguard.obj";
/** Current model: */
const MODEL_PATH: &str = OBJ_AFRO_HEAD;
const PERCENTAGE_MODEL_SCALER: u32 = 90;

static mut MAX_VERTEX_ABS: f64 = 0.0;

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
    let mut progress = 0;
    for line in file.lines() {
        let l = line.unwrap();
        scan_vertex(&l.clone(), &mut vertex);
        scan_face(&l.clone(), &mut faces);
        if progress % (10*line_count/100) == 0 {
            println!("Load progress: {}%", progress / (line_count/100));
        }
        progress += 1;
    }
    unsafe {
        for element in vertex.iter_mut() {
            element[_VERTEX_X] /= MAX_VERTEX_ABS;
            element[_VERTEX_Y] /= MAX_VERTEX_ABS;
            element[_VERTEX_Z] /= MAX_VERTEX_ABS;
        }
    }
    println!("Vertex: {}\nFaces: {}", vertex.len(), faces.len());
    return (vertex, faces);
}

fn main() {
    let mut canvas: SdlCanvas = canvas_trait::Canvas::new(WIDTH, HEIGHT);

    let (vertex, faces) = parse_model_obj(MODEL_PATH);
    for element in faces.iter() {
        let scaler: u32 = if WIDTH < HEIGHT { WIDTH } else { HEIGHT } * PERCENTAGE_MODEL_SCALER / 100;

        let vertex_a: usize = element[_FACE_POINT_A] - 1;
        let vertex_b: usize = element[_FACE_POINT_B] - 1;

        let vertex_a_x = vertex[vertex_a][_VERTEX_X] * scaler as f64/2.0 + WIDTH as f64/2.0;
        let vertex_a_y = vertex[vertex_a][_VERTEX_Y] * scaler as f64/2.0 + HEIGHT as f64/2.0;
        let vertex_b_x = vertex[vertex_b][_VERTEX_X] * scaler as f64/2.0 + WIDTH as f64/2.0;
        let vertex_b_y = vertex[vertex_b][_VERTEX_Y] * scaler as f64/2.0 + HEIGHT as f64/2.0;
        canvas.line(vertex_a_x as i32,
                    vertex_a_y as i32,
                    vertex_b_x as i32,
                    vertex_b_y as i32,
                    RgbColor::ICEBLUE.value()).unwrap();
    }
    canvas.out().unwrap();
}
