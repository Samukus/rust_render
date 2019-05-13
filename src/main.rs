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
use canvas_trait::Canvas;
#[allow(unused)]
use canvas_trait::RgbColor;
use canvas_trait::Ergbcolor;

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
const OBJ_FROSTMOURNE: &str = "obj/frostmourne.obj";
#[allow(dead_code)]
const OBJ_VANGUARD: &str = "obj/vanguard.obj";

fn main() {
    let afro_head: ObjModel = model_trait::Model::new(std::path::Path::new(OBJ_AFRO_HEAD));
    let frostmourne: ObjModel = model_trait::Model::new(std::path::Path::new(OBJ_FROSTMOURNE));
    let mut canvas: SdlCanvas = canvas_trait::Canvas::new(WIDTH, HEIGHT);
    canvas.render_poly_lightning(Box::new(afro_head), SIZE,
                       Vector3D {
                           x: WIDTH as f64 * 0.33,
                           y: HEIGHT as f64 * 0.50,
                           z: 0.0
                       },
                       RgbColor::new(Ergbcolor::GRAY));
    canvas.render_poly_lightning(Box::new(frostmourne), SIZE,
                       Vector3D {
                           x: WIDTH as f64 * 0.66,
                           y: HEIGHT as f64 * 0.50,
                           z: 0.0
                       },
                       RgbColor::new(Ergbcolor::ICEBLUE));
    canvas.out().unwrap();
}
