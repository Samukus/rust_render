#[macro_use] extern crate scan_fmt;
#[macro_use] extern crate log;
extern crate image;

mod canvas_sdl;
#[allow(unused_imports)]
use canvas_sdl::SdlCanvas;

mod canvas_tga;
#[allow(unused_imports)]
use canvas_tga::TgaCanvas;

mod canvas_trait;
use canvas_trait::{Canvas, RgbColor};

mod model_trait;

mod model_obj;
use model_obj::ObjModel;

mod geometry;
use geometry::*;

/** Image Size */
const WIDTH: u32 = 1600;
const HEIGHT: u32 = 800;
const SIZE: f64 = HEIGHT as f64 * 0.45;

/** Project test images */
#[allow(dead_code)]
const OBJ_AFRO_HEAD: &str = "obj/african_head.obj";
#[allow(dead_code)]
const OBJ_ARTORIAS_SWORD: &str = "obj/artorias_sword.obj";
#[allow(dead_code)]
const OBJ_FROSTMOURNE: &str = "obj/frostmourne.obj";
#[allow(dead_code)]
const OBJ_VANGUARD: &str = "obj/vanguard.obj";

fn main() {
    let afro_head: ObjModel = model_trait::Model::new(std::path::Path::new(OBJ_AFRO_HEAD));
    // let frostmourne: ObjModel = model_trait::Model::new(std::path::Path::new(OBJ_FROSTMOURNE));
    // let artorias_sword: ObjModel = model_trait::Model::new(std::path::Path::new(OBJ_ARTORIAS_SWORD));
    let mut canvas: SdlCanvas = canvas_trait::Canvas::new(WIDTH, HEIGHT);

    canvas.render_poly(Box::new(afro_head), SIZE,
                       Vector3D {
                           x: WIDTH as f64 * 0.25,
                           y: HEIGHT as f64 * 0.50,
                           z: 0.0
                       },
                       RgbColor::WHITE.value());
    // canvas.render_wire(Box::new(artorias_sword), SIZE * 2.0,
    //                    Vector3D {
    //                        x: WIDTH as f64 * 0.50,
    //                        y: HEIGHT as f64 * 0.05,
    //                        z: 0.0
    //                    },
    //                    RgbColor::RED.value()); 
    // canvas.render_wire(Box::new(frostmourne), SIZE,
    //                    Vector3D {
    //                        x: WIDTH as f64 * 0.75,
    //                        y: HEIGHT as f64 * 0.50,
    //                        z: 0.0
    //                    },
    //                    RgbColor::ICEBLUE.value()); 
    canvas.out().unwrap();
}
