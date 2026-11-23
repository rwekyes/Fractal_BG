use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Squared magnitude, |z|². Skips the sqrt: it's cheaper, and
    /// comparing against 4.0 works the same as comparing |z| against 2.0.
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let sum = Complex::new(1.0, 2.0) + Complex::new(3.0, -1.0);
        assert_eq!(sum, Complex::new(4.0, 1.0));
    }

    #[test]
    fn i_squared_is_minus_one() {
        let i = Complex::new(0.0, 1.0);
        assert_eq!(i * i, Complex::new(-1.0, 0.0));
    }

    #[test]
    fn multiplication() {
        // (1 + 2i)(3 + 4i) = 3 + 4i + 6i + 8i² = -5 + 10i
        let product = Complex::new(1.0, 2.0) * Complex::new(3.0, 4.0);
        assert_eq!(product, Complex::new(-5.0, 10.0));
    }

    #[test]
    fn norm_sqr_of_3_4_is_25() {
        assert_eq!(Complex::new(3.0, 4.0).norm_sqr(), 25.0);
    }
}
