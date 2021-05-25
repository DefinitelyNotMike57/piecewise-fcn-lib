// Make the standard subfunction interface available
use crate::factory::interface::SubfunctionOutput;

/// The bump function
///
/// The bump function requires four inputs upon creation:
/// * Duration: Length that the user wants to use this function for
/// * Interval: The bump function is defined over (-1,1) any interval outside of those bounds is set to the offset value
/// * Scale: Scale factor to control the amplitude of the bump
/// * Offset: Value by which to offset the bump
pub struct Bump {
  // Duration insinuates time but it really represents the amount of
  // space/time/etc. that this function is valid over.
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
  /// Creates a new bump function
  pub fn new(
    dur: f64,
    interval: (f64, f64),
    scale: f64,
    offset: f64,
  ) -> Bump {
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
  fn generate(
    &self,
    x: f64,
  ) -> Option<f64> {
    let mut result = None;
    if (x >= 0.0) && (x < self.duration) {
      let new_x = x / self.duration * (self.interval.1 - self.interval.0) + self.interval.0;
      // The bump function only works between -1.0 and 1.0
      if (new_x > -1.0) && (new_x < 1.0) {
        result = Some(self.scale * (new_x.powf(2.0) / (new_x.powf(2.0) - 1.0)).exp() + self.offset);
      } else {
        result = Some(self.offset);
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Verify the one-sided structure of the bump function
  #[test]
  fn normal() {
    let a = Bump::new(2.0, (-1.0, 1.0), 1.0, 0.0);
    assert_eq!(0.00, (a.generate(0.0).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.01, (a.generate(0.1).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.17, (a.generate(0.2).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.38, (a.generate(0.3).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.57, (a.generate(0.4).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.72, (a.generate(0.5).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.83, (a.generate(0.6).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.91, (a.generate(0.7).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.96, (a.generate(0.8).unwrap() * 100.0).round() / 100.0);
    assert_eq!(0.99, (a.generate(0.9).unwrap() * 100.0).round() / 100.0);
    assert_eq!(1.00, (a.generate(1.0).unwrap() * 100.0).round() / 100.0);
  }

  #[test]
  fn scale() {
    let a = Bump::new(1.0, (-1.0, 1.0), 5.0, 0.0);
    assert_eq!(Some(5.0), a.generate(0.5));
  }

  #[test]
  fn offset() {
    let a = Bump::new(1.0, (-2.0, 2.0), 1.0, 2.0);
    assert_eq!(Some(2.0), a.generate(0.0));
    assert_eq!(Some(3.0), a.generate(0.5));
    assert_eq!(Some(2.0), a.generate(0.9));
  }
  #[test]
  fn out_of_bounds() {
    let a = Bump::new(1.0, (-2.0, 2.0), 1.0, 2.0);
    assert_eq!(None, a.generate(-0.1));
    assert_eq!(None, a.generate( 1.1));
  }
}
