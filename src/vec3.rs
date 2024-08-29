use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

pub type Scalar = f64;

use crate::common;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [Scalar; 3],
}

impl Vec3 {
    pub const fn new(x: Scalar, y: Scalar, z:Scalar) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub const fn uni(x: Scalar) -> Vec3 {
        Vec3::new(x, x, x)
    }

    pub fn random() -> Vec3 {
        Vec3::new(
            common::random_double(),
            common::random_double(),
            common::random_double(),
        )
    }

    pub fn random_range(min: Scalar, max: Scalar) -> Vec3 {
        Vec3::new(
            common::random_double_range(min, max),
            common::random_double_range(min, max),
            common::random_double_range(min, max),
        )
    }

    pub fn x(&self) -> Scalar {
        self.e[0]
    }
    pub fn y(&self) -> Scalar {
        self.e[1]
    }
    pub fn z(&self) -> Scalar {
        self.e[2]
    }
    
    pub fn length(&self) -> Scalar {
        Scalar::sqrt(self.length_squared())
    }
    
    pub fn length_squared(&self) -> Scalar {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

// Vec3 *= scalar
impl MulAssign<Scalar> for Vec3 {
    fn mul_assign(&mut self, t: Scalar) {
        *self = *self * t;
    }
}

// Vec3 /= Scalar
impl DivAssign<Scalar> for Vec3 {
    fn div_assign(&mut self, t: Scalar) {
        *self = *self / t;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        self.add(-v)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

// f64 * Vec3
impl Mul<Vec3> for Scalar {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Mul<Scalar> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Scalar) -> Self::Output {
        rhs.mul(self)
    }
}

impl Div<Scalar> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Scalar) -> Self::Output {
        self.mul(1.0 / rhs)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> Scalar {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new( 
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x())
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn fit01(v: Vec3) -> Vec3 {
    0.5 * (v + Vec3::uni(1.0))
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}