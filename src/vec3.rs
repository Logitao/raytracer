use crate::util::nearly_equal;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn add(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub fn sub(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }

    pub fn unit(&self) -> Vec3 {
        let len_sqrd = self.len_squared();

        Vec3 {
            x: self.x / len_sqrd.sqrt(),
            y: self.y / len_sqrd.sqrt(),
            z: self.z / len_sqrd.sqrt(),
        }
    }

    pub fn normalize(v: Vec3) -> Vec3 {
        let len_sqrd = v.len_squared();

        Vec3 {
            x: v.x / len_sqrd.sqrt(),
            y: v.y / len_sqrd.sqrt(),
            z: v.z / len_sqrd.sqrt(),
        }
    }

    pub fn len(&self) -> f32 {
        let sqrt = self.x * self.x + self.y * self.y + self.z * self.z;
        sqrt.sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Self {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn negate(v: Vec3) -> Self {
        Vec3 {
            x: -v.x,
            y: -v.y,
            z: -v.z,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::add(self, v)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::sub(self, v)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f32) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[test]
fn it_subs() {
    let sub_1 = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(1.0, 2.0, 3.0);

    let result_1 = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(result_1.x, sub_1.x);
    assert_eq!(result_1.y, sub_1.y);
    assert_eq!(result_1.z, sub_1.z);

    let sub_2 = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, 5.5, 6.0);
    let result_2 = Vec3::new(-3.0, -3.5, -3.0);
    assert_eq!(result_2.x, sub_2.x);
    assert_eq!(result_2.y, sub_2.y);
    assert_eq!(result_2.z, sub_2.z);
}

#[test]
fn it_mults() {
    let a = Vec3::new(1.0, 2.0, 3.0);

    let c = 4.32;
    let mult_1 = Vec3::new(1.0, 2.0, 3.0) * c;
    let mult_2 = c * Vec3::new(1.0, 2.0, 3.0);

    let result_1 = Vec3::new(c * a.x, c * a.y, c * a.z);
    let result_2 = Vec3::new(4.32, 8.64, 12.96);

    assert!(nearly_equal(result_1.x, mult_1.x));
    assert!(nearly_equal(result_1.y, mult_1.y));
    assert!(nearly_equal(result_1.z, mult_1.z));

    assert!(nearly_equal(result_1.x, mult_2.x));
    assert!(nearly_equal(result_1.y, mult_2.y));
    assert!(nearly_equal(result_1.z, mult_2.z));

    assert!(nearly_equal(result_2.x, mult_1.x));
    assert!(nearly_equal(result_2.y, mult_1.y));
    assert!(nearly_equal(result_2.z, mult_1.z));

    assert!(nearly_equal(result_2.x, mult_2.x));
    assert!(nearly_equal(result_2.y, mult_2.y));
    assert!(nearly_equal(result_2.z, mult_2.z));
}

#[test]
fn it_cross() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 2.0, 3.0);

    let c = Vec3::cross(a, b);

    let result_1 = Vec3::zero();

    assert_eq!(result_1.x, c.x);
    assert_eq!(result_1.y, c.y);
    assert_eq!(result_1.z, c.z);

    let d = Vec3::new(1.0, 2.0, 3.0);
    let e = Vec3::new(3.0, 2.0, 1.0);

    let f = Vec3::cross(d, e);
    let result_2 = Vec3::new(-4.0, 8.0, -4.0);

    assert_eq!(result_2.x, f.x);
    assert_eq!(result_2.y, f.y);
    assert_eq!(result_2.z, f.z);
}

#[test]
fn it_dot() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 2.0, 3.0);

    let c = Vec3::new(3.0, 2.0, 1.0);
    let d = Vec3::new(5.0, 8.0, 0.0);

    let result_1 = 14.0;
    let result_2 = a.len_squared();
    let result_3 = a.x * b.x + a.y * b.y + a.z * b.z;

    let result_4 = 31.0;
    let result_5 = c.x * d.x + c.y * d.y + c.z * d.z;

    let dot_1 = Vec3::dot(a, b);
    let dot_2 = Vec3::dot(c, d);

    assert!(nearly_equal(dot_1, result_1));
    assert!(nearly_equal(dot_1, result_2));
    assert!(nearly_equal(dot_1, result_3));

    assert!(nearly_equal(dot_2, result_4));
    assert!(nearly_equal(dot_2, result_5));
}

#[test]
fn it_units() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let unit = a.unit();

    assert!(nearly_equal(unit.len(), 1.0));
}
