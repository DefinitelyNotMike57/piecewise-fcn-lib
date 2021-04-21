/// Polynomial defines a continuous sub-domain
///
/// This object allows the user to map any interval of a polynomial into a
/// sub-space for use in a function.
///
pub struct Polynomial {
    // Duration insinuates time but it really represents the amount of
    // space/time/etc. that this Polynomial is valid over.
    duration: f64,
    // The interval for which to define the polynomial.
    interval: Vec<f64>,
    // Coefficients of the polynomial. The 0 index is the coefficient of x^0,
    // the 1 index is the coefficient of the x^1 and so on.
    coefficients: Vec<f64>,
    // This flag allows the user to reverse the output of the polynomial. It
    // might not be intuitive but defining a polynomial with the exact reverse
    // structure is not always straight-forward and this helps mitigate that
    // heart-ache.
    reverse: bool,
}

/// This defines the common interface for all piecewise types
///
///
pub trait Domain {

    fn new(dur: f64, interval: Vec<f64>, coeff: Vec<f64>, reverse: bool) -> Polynomial;
    fn get_duration(&self) -> f64;
    ///
    fn generate(&self, x: f64) -> Option<f64>;
}

impl Domain for Polynomial {
    fn new(dur: f64, interval: Vec<f64>, coeff: Vec<f64>, reverse: bool) -> Polynomial {
        let out = Polynomial {
            duration: dur,
            interval: interval,
            coefficients: coeff,
            reverse: reverse,
        };
        out
    }
    fn get_duration(&self) -> f64 {
        self.duration
    }
    fn generate(&self, x: f64) -> Option<f64> {
        let mut result = None;
        if (x >= 0.0) && (x < self.duration) {
            let new_x: f64;
            if self.reverse {
                new_x =
                    self.interval[1] - x / self.duration * (self.interval[1] - self.interval[0]);
            } else {
                new_x =
                    x / self.duration * (self.interval[1] - self.interval[0]) + self.interval[0];
            }
            let mut out: f64 = 0.0;
            for idx in self.coefficients.iter().enumerate() {
                out += idx.1 * new_x.powf(idx.0 as f64);
            }
            result = Some(out);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polynomial() {
        let a = Polynomial::new(1.0, vec![0.0, 1.0], vec![4.0, 5.0], false);
        assert_eq!(Some(4.0), a.generate(0.0));
        assert_eq!(Some(6.5), a.generate(0.5));
    }
    #[test]
    fn out_of_bounds() {
        let a = Polynomial::new(1.0, vec![0.0, 1.0], vec![4.0, 5.0], false);
        assert_eq!(None, a.generate(-0.5));
        assert_eq!(None, a.generate(1.5));
    }
    #[test]
    fn reverse() {
        let a = Polynomial::new(1.0, vec![0.0, 1.0], vec![4.0, 5.0], true);
        assert_eq!(Some(9.0), a.generate(0.0));
    }
}
