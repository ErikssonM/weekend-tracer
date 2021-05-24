use std::iter::Sum;
use std::ops::{Add, Mul};

use rand::random;

use crate::geometry::{rand_in, v3, V3};

#[derive(Clone, Copy, Debug)]
pub struct Color(pub V3);

impl Color {
    pub fn black() -> Self {
        Color(v3(0.0, 0.0, 0.0))
    }

    pub fn to_ppm_row(&self, samples: u32) -> String {
        let ratio = 1.0 / (samples as f64);
        let r = self.0.x * ratio;
        let g = self.0.y * ratio;
        let b = self.0.z * ratio;

        format!(
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as u32,
            (256.0 * g.clamp(0.0, 0.999)) as u32,
            (256.0 * b.clamp(0.0, 0.999)) as u32,
        )
    }

    pub fn ppm(&self) -> String {
        // sqrt for gamma correction
        let r = self.0.x.sqrt();
        let g = self.0.y.sqrt();
        let b = self.0.z.sqrt();

        format!(
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as u32,
            (256.0 * g.clamp(0.0, 0.999)) as u32,
            (256.0 * b.clamp(0.0, 0.999)) as u32,
        )
    }

    pub fn mut_const_mul(&mut self, c: f64) {
        self.0 *= c;
    }

    pub fn random() -> Color {
        Color(v3(random(), random(), random()))
    }

    pub fn random_in(min: f64, max: f64) -> Color {
        Color(v3(rand_in(min, max), rand_in(min, max), rand_in(min, max)))
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(v3(
            self.0.x * rhs.0.x,
            self.0.y * rhs.0.y,
            self.0.z * rhs.0.z,
        ))
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::black(), |a, b| a + b)
    }
}
