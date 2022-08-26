use super::{utils, Color};
use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
    },
};

pub type Point3 = Vec3;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn random() -> Self {
        Self(
            utils::random_double(),
            utils::random_double(),
            utils::random_double(),
        )
    }

    pub fn random_range(range: Range<f64>) -> Self {
        Self(
            utils::random_double_range(range.clone()),
            utils::random_double_range(range.clone()),
            utils::random_double_range(range),
        )
    }

    /// Returns a random Point within a unit sphere using a rejection method. Pick a point in a
    /// cube and return if that point is in the sphere, otherwise retry.
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0..1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    // Returns a random point on a unit sphere using the `random_in_unit_sphere` method and turning
    // it into a unit vector.
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Returns `true` if all axes are almost zero (smaller than some eps).
    pub fn near_zero(&self) -> bool {
        let eps = 1.0e-8;
        self.0.abs() < eps && self.1.abs() < eps && self.2.abs() < eps
    }

    /// Returns the vector reflected using the normal. The size of the vector is kept.
    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - 2.0 * self.dot(*normal) * *normal
    }

    pub fn refract(&self, normal: &Vec3, eta_over_etap: f64) -> Self {
        let cos_theta = (-*self).dot(*normal);

        // perpendicular part of the refracted ray
        let r_perp = eta_over_etap * (*self + cos_theta * *normal);
        // parallel part of the refracted ray
        let r_parallel = -(1.0 - r_perp.length_squared()).sqrt() * *normal;

        r_perp + r_parallel
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

// Operator overloads

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("cannot use index larger than 2"),
        }
    }
}

impl IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("cannot use index larger than 2"),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.mul(self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl From<Color> for Vec3 {
    fn from(color: Color) -> Self {
        Self(color.red(), color.green(), color.blue())
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}
