
// Make the standard subfunction interface available
pub use crate::subfunctions::interface::*;

pub struct Bump { // Duration insinuates time but it really represents the amount of
    // space/time/etc. that this Polynomial is valid over.
    duration: f64,
    // The interval for which to define the bump (largest interval is -1 to 1)
    interval: (f64,f64),
    // Scales amplitude of bump (default: 1.0)
    scale: f64,
    // Offsets bump in y-axis (default: 0.0)
    offset: f64,
}

/// Constructor for the Bump
impl Bump {
    fn new(dur: f64, interval: (f64,f64), scale:f64, offset:f64) -> Bump {
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
