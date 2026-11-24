use crate::complex::Complex;

/// Iterates z = z² + c, starting from z = 0.
///
/// Returns `Some(n)` if |z| exceeded 2 after `n` iterations, or `None`
/// if it stayed bounded for all `max_iter` iterations (so `c` is
/// presumed to be in the Mandelbrot set).
pub fn escape_time(c: Complex, max_iter: u32) -> Option<u32> {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX: u32 = 1000;

    #[test]
    fn origin_is_in_the_set() {
        assert_eq!(escape_time(Complex::new(0.0, 0.0), MAX), None);
    }

    #[test]
    fn minus_one_is_in_the_set() {
        // z cycles 0, -1, 0, -1, ... forever
        assert_eq!(escape_time(Complex::new(-1.0, 0.0), MAX), None);
    }

    #[test]
    fn minus_two_is_on_the_edge() {
        // z goes 0, -2, 2, 2, 2, ... |z| equals 2 but never exceeds it
        assert_eq!(escape_time(Complex::new(-2.0, 0.0), MAX), None);
    }

    #[test]
    fn one_escapes_after_three() {
        // z goes 0, 1, 2, 5 → |5|² = 25 > 4 on the check at i = 3
        assert_eq!(escape_time(Complex::new(1.0, 0.0), MAX), Some(3));
    }

    #[test]
    fn far_point_escapes_immediately() {
        // z goes 0, (2+2i)  |z|² = 8 > 4 on the check at i = 1
        assert_eq!(escape_time(Complex::new(2.0, 2.0), MAX), Some(1));
    }
}
