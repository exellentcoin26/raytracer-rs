use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign},
};

pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        assert!(
            red >= 0f64
                && red <= 1f64
                && green >= 0f64
                && green <= 1f64
                && blue >= 0f64
                && blue <= 1f64
        );
        Self(red, green, blue)
    }

    pub fn red(&self) -> f64 {
        assert!(self.0 <= 1f64 && self.0 >= 0f64);
        self.0
    }

    pub fn green(&self) -> f64 {
        assert!(self.1 <= 1f64 && self.1 >= 0f64);
        self.1
    }

    pub fn blue(&self) -> f64 {
        assert!(self.2 <= 1f64 && self.2 >= 0f64);
        self.2
    }

    pub fn valid(&self) -> bool {
        self.0 >= 0f64
            && self.0 <= 1f64
            && self.1 >= 0f64
            && self.1 <= 1f64
            && self.2 >= 0f64
            && self.2 <= 1f64
    }

    pub fn write_color(&self) -> String {
        assert!(self.valid());
        format!(
            "{} {} {}",
            (self.0 * 255f64).round() as u8,
            (self.1 * 255f64).round() as u8,
            (self.2 * 255f64).round() as u8
        )
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        assert!(self.valid())
    }
}

impl Mul<f64> for Color {
    type Output = Color;

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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(self.valid());
        write!(f, "({} {} {})", self.0, self.1, self.2)
    }
}
