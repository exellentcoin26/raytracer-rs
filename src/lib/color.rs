use super::Vec3;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign},
};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        assert!(
            (0f64..=1f64).contains(&red)
                && (0f64..=1f64).contains(&green)
                && (0f64..=1f64).contains(&blue)
        );
        Self(red, green, blue)
    }

    pub fn red(&self) -> f64 {
        assert!((0f64..=1f64).contains(&self.0));
        self.0
    }

    pub fn green(&self) -> f64 {
        assert!((0f64..=1f64).contains(&self.1));
        self.1
    }

    pub fn blue(&self) -> f64 {
        assert!((0f64..=1f64).contains(&self.2));
        self.2
    }

    pub fn valid(&self) -> bool {
        (0f64..=1f64).contains(&self.0)
            && (0f64..=1f64).contains(&self.1)
            && (0f64..=1f64).contains(&self.2)
    }

    pub fn write_color(&self) -> String {
        assert!(self.valid());

        // gamma correct before output
        let gamma_inv = 1.0 / 2.0;
        let r = self.0.powf(gamma_inv);
        let g = self.1.powf(gamma_inv);
        let b = self.2.powf(gamma_inv);

        format!(
            "{} {} {}",
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        assert!(self.valid());
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs.mul(self)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
        assert!(self.valid());
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
        assert!(self.valid());
    }
}

impl TryFrom<Vec3> for Color {
    type Error = String;

    fn try_from(value: Vec3) -> Result<Self, Self::Error> {
        if !((0f64..=1f64).contains(&value.x())
            && (0f64..=1f64).contains(&value.y())
            && (0f64..=1f64).contains(&value.z()))
        {
            return Err("could not convert `Vec3` to `Color`: `x`, `y` or `z` values are not contained in the `(0..=1)` range".to_string());
        }

        Ok(Self(value.x(), value.y(), value.z()))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(self.valid());
        write!(f, "({} {} {})", self.0, self.1, self.2)
    }
}
