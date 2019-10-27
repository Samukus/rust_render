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
    let light_vec = Vector3D {x: 0.0, y: 0.0, z: -1.0}.normalize();
    let mut canvas: SdlCanvas = canvas_trait::Canvas::new(WIDTH, HEIGHT);

    println!("Render afro");
    canvas.render_poly(Box::new(afro_head.clone()), SIZE,
                       Vector3D {
                           x: WIDTH as f64 * 0.33,
                           y: HEIGHT as f64 * 0.50,
                           z: 300.0,
                       },
                       RgbColor::new(Ergbcolor::GRAY), light_vec);
    // Workaround to reduce floating point render errors
    canvas.render_wire(Box::new(afro_head.clone()), SIZE,
                       Vector3D {
                           x: WIDTH as f64 * 0.33,
                           y: HEIGHT as f64 * 0.50,
                           z: 300.0,
                       },
                       RgbColor::new(Ergbcolor::GRAY));
    println!("Render frostmourne");
    canvas.render_poly(Box::new(frostmourne), SIZE,
                       Vector3D {
                           x: WIDTH as f64 * 0.66,
                           y: HEIGHT as f64 * 0.50,
                           z: 50.0
                       },
                       RgbColor::new(Ergbcolor::ICEBLUE), light_vec);
    println!("Output");
    canvas.out().unwrap();
}
