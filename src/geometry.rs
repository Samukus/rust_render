use std::ops::Div;
use std::ops::Mul;
use std::ops::Add;

#[derive(Clone)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f64) -> Self {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;
    fn mul(self, k: f64) -> Self {
        Vector3D {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn add(self, offset: Vector3D) -> Self {
        Vector3D {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub p1: Vector3D,
    pub p2: Vector3D,
    pub p3: Vector3D,
}

impl Div<f64> for Triangle {
    type Output = Triangle;
    fn div(self, rhs: f64) -> Self {
        Triangle {
            p1: self.p1 / rhs,
            p2: self.p2 / rhs,
            p3: self.p3 / rhs,
        }
    }
}

impl Mul<f64> for Triangle {
    type Output = Triangle;
    fn mul(self, k: f64) -> Self {
        Triangle {
            p1: self.p1 * k,
            p2: self.p2 * k,
            p3: self.p3 * k,
        }
    }
}

impl Add<Vector3D> for Triangle {
    type Output = Triangle;
    fn add(self, offset: Vector3D) -> Self {
        Triangle {
            p1: self.p1 + offset.clone(),
            p2: self.p2 + offset.clone(),
            p3: self.p3 + offset.clone(),
        }
    }
}