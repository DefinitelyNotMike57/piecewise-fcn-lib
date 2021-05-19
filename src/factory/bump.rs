use std::cmp;

// Make the standard subfunction interface available
use crate::factory::interface::SubfunctionOutput;

pub struct Bump {
    // Duration insinuates time but it really represents the amount of
    // space/time/etc. that this Polynomial is valid over.
    duration: f64,
    // The interval for which to define the bump (function values outside -1.0->1.0 will be zero)
    interval: (f64, f64),
    // Scales amplitude of bump (default: 1.0)
    scale: f64,
    // Offsets bump in y-axis (default: 0.0)
    offset: f64,
}

/// Constructor for the Bump
impl Bump {
    fn new(dur: f64, interval: (f64, f64), scale: f64, offset: f64) -> Bump {
        let out = Bump {
            duration: dur,
            interval: interval,
            scale: scale,
            offset: offset,
        };
        out
    }
}

impl SubfunctionOutput for Bump {
    fn get_duration(&self) -> f64 {
        self.duration
    }
    fn generate(&self, x: f64) -> Option<f64> {
        let mut result = None;
        if (x >= 0.0) && (x < self.duration) {
            let new_x = x / self.duration * (self.interval.1 - self.interval.0) + self.interval.0;
            // The bump function only works between -1.0 and 1.0
            if (new_x > -1.0) && (new_x < 1.0) {
                result = Some((-1.0 / (1.0 - new_x.exp2())).exp());
            } else {
                result = Some(0.0);
            }
        }
        result
    }
}

#[cfg(test)]
mod bump {
    use super::*;

    #[test]
    fn normal() {
        let a = Bump::new(1.0, (-1.0, 1.0), 5.0, 1.0);
        assert_eq!(Some(6.0), a.generate(0.5));
    }
    #[test]
    fn out_of_bounds() {
        let a = Bump::new(1.0, (-1.0, 1.0), 5.0, 1.0);
        assert_eq!(None, a.generate(-0.5));
        assert_eq!(None, a.generate(1.5));
    }
}
