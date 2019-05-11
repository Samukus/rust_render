use std::slice::Iter;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use crate::model_trait::Model;
use crate::geometry::{
    Vector3D,
    Triangle,
};

#[derive(Clone)]
pub struct ObjModel {
    triangles: Vec<Triangle>,
}

impl ObjModel {
    fn scan_vertex(&mut self, line: &str) -> Result<Vector3D, String> {
        let (x, y, z) = scan_fmt!(line, // input string
                        "v {} {} {}",  // format
                        f64, f64, f64);   // type of a-c Options
        if x.is_some() && y.is_some() && z.is_some() {
            debug!("Vertex: {} {} {}", x.unwrap(), y.unwrap(), z.unwrap() as f64);
            Ok(Vector3D {
                x: x.unwrap(),
                y: y.unwrap(),
                z: z.unwrap(),
            })
        }
        else {
            Err("Not a vertex line format".to_string())
        }
    }

    fn scan_face(&mut self, line: &str) -> Result<(usize, usize, usize), String> {
        let (a, b, c) = scan_fmt!(line, // input string
                        "f {}/{*}/{*} {}/{*}/{*} {}/{*}/{*}",  // format
                        usize, usize, usize);   // type of a-c Options
        if a.is_some() && b.is_some() && c.is_some() {
            debug!("Triangle: {}-{}-{}", a.unwrap(), b.unwrap(), c.unwrap());
            return Ok((a.unwrap() as usize, b.unwrap() as usize, c.unwrap() as usize));
        }
        let (a, b, c) = scan_fmt!(line, // input string
                        "f {} {} {}",  // format
                        usize, usize, usize);   // type of a-c Options
        if a.is_some() && b.is_some() && c.is_some() {
            debug!("Triangle: {}-{}-{}", a.unwrap(), b.unwrap(), c.unwrap());
            return Ok((a.unwrap() as usize, b.unwrap() as usize, c.unwrap() as usize));
        }
        Err("Not a face line format".to_string())
    }

    fn read_model(&mut self, file_path: &Path) -> Result<(), String> {
        // TODO: catch error
        let str_path = file_path.to_str().unwrap();
        println!("Loading model {}", str_path);
        let mut vertex: Vec<Vector3D> = Vec::new();

        let line_count = BufReader::new(&File::open(str_path).unwrap()).lines().count();
        let f = File::open(str_path).unwrap();
        let file = BufReader::new(&f);
        let mut progress = 0;
        let mut max_abs = 0.0;
        for line in file.lines() {
            let l = line.unwrap();

            if progress % (5*line_count/100) == 0 {
                println!("Load progress: {}%", progress / (line_count/100));
            }
            progress += 1;

            match self.scan_vertex(&l.clone()) {
                Result::Ok(val) => {
                    if val.x.abs() > max_abs {
                        max_abs = val.x.abs();
                    }
                    if val.y.abs() > max_abs {
                        max_abs = val.y.abs();
                    }
                    if val.z.abs() > max_abs {
                        max_abs = val.z.abs();
                    }
                    vertex.push(val);
                    continue;
                }
                // Skip error because of wrong format
                Result::Err(err) => debug!("Scan vertex error: {}", err),
            }

            match self.scan_face(&l.clone()) {
                Result::Ok(val) => {
                    self.triangles.push(
                        Triangle {
                            p1: vertex[val.0 - 1].clone() / max_abs,
                            p2: vertex[val.1 - 1].clone() / max_abs,
                            p3: vertex[val.2 - 1].clone() / max_abs,
                        }
                    );
                    continue;
                }
                // Skip error because of wrong format
                Result::Err(err) => debug!("Scan face error: {}", err),
            }
        }
        println!("Triangles: {}", self.triangles.len());
        Ok(())
    }
}

impl Model for ObjModel {
    fn new(file_path: &Path) -> Self {
        let mut model = ObjModel { 
            triangles: Vec::new(),
        };
        match model.read_model(file_path) {
            Result::Ok(_val) => println!("Model {} load successfully", file_path.to_str().unwrap()),
            Result::Err(err) => println!("Model loading error: {}", err),
        }
        model
    }
    fn triangle_iter(&self) -> Iter<Triangle> {
        self.triangles.iter()
    }
}