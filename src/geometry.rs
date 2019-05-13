use std::ops::Div;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn abs(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normalize(&self) -> Vector3D {
        let result = Vector3D {x: self.x, y: self.y, z: self.z};
        result / result.abs()
    }
    pub fn scalar(&self, vec: Vector3D) -> f64 {
        self.x*vec.x + self.y*vec.y + self.z*vec.z
    }
}
#[test]
fn vector_abs() {
    let a = Vector3D {x: 10.0, y: 0.0, z: 0.0};
    println!("|{}| = {}", a, a.abs());
    let a = Vector3D {x: 10.0, y: 10.0, z: 10.0};
    println!("|{}| = {}", a, a.abs());
}
#[test]
fn vector_normalize() {
    let a = Vector3D {x: 10.0, y: 0.0, z: 0.0};
    println!("Normalized {}: {} (abs {})", a, a.normalize(), a.abs());
    let a = Vector3D {x: 10.0, y: 10.0, z: 10.0};
    println!("Normalized {}: {} (abs {})", a, a.normalize(), a.normalize().abs());
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
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

impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn sub(self, offset: Vector3D) -> Self {
        Vector3D {
            x: self.x - offset.x,
            y: self.y - offset.y,
            z: self.z - offset.z,
        }
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;
    fn neg(self) -> Self {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn mul(self, v: Vector3D) -> Self {
        Vector3D {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
}

#[test]
fn vector_commutative_multiple() {
    let a = Vector3D {x: 10.0, y: 0.0, z: 0.0};
    let b = Vector3D {x: 0.0, y: 10.0, z: 0.0};
    println!("Commutative vector multiply: {}x{}={}", a, b, a*b);
    println!("Commutative vector multiply: {}x{}={}", b, a, b*a);
}



#[derive(Clone)]
pub struct Triangle {
    pub p0: Vector3D,
    pub p1: Vector3D,
    pub p2: Vector3D,
}

impl Triangle {
    pub fn normal(&self) -> Vector3D {
        let v1 = self.p1 - self.p0;
        let v2 = self.p2 - self.p0;
        (v2 * v1).normalize()
    }
}

impl Div<f64> for Triangle {
    type Output = Triangle;
    fn div(self, rhs: f64) -> Self {
        Triangle {
            p0: self.p0 / rhs,
            p1: self.p1 / rhs,
            p2: self.p2 / rhs,
        }
    }
}

impl Mul<f64> for Triangle {
    type Output = Triangle;
    fn mul(self, k: f64) -> Self {
        Triangle {
            p0: self.p0 * k,
            p1: self.p1 * k,
            p2: self.p2 * k,
        }
    }
}

impl Add<Vector3D> for Triangle {
    type Output = Triangle;
    fn add(self, offset: Vector3D) -> Self {
        Triangle {
            p0: self.p0 + offset.clone(),
            p1: self.p1 + offset.clone(),
            p2: self.p2 + offset.clone(),
        }
    }
}