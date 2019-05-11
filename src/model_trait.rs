use std::slice::Iter;
use std::path::Path;

use crate::geometry::Triangle;

pub trait Model {
    fn new(file_path: &Path) -> Self where Self: Sized;
    fn triangle_iter(&self) -> Iter<Triangle>;
}