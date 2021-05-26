/// Definition of subfunction interface
pub mod interface;

/// Definition of polynomial
pub mod polynomial;

/// Definition of bump
pub mod bump;

use interface::FunctionOutput;

/// Factory to generate all subfunctions
pub struct Factory;

impl Factory {
  pub fn polynomial(
    &self,
    dur: f64,
    interval: (f64, f64),
    coeff: Vec<f64>,
    reverse: bool,
  ) -> Box<dyn FunctionOutput> {
    let new = Box::new(polynomial::Polynomial::new(dur, interval, coeff, reverse));
    new
  }
  pub fn bump(
    &self,
    dur: f64,
    interval: (f64, f64),
    scale: f64,
    offset: f64,
  ) -> Box<dyn FunctionOutput> {
    let new = Box::new(bump::Bump::new(dur, interval, scale, offset));
    new
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn polynomial() {
    let factory = Factory;
    let a = factory.polynomial(1.0, (0.0, 1.0), vec![4.0, 5.0], false);
    assert_eq!(Some(6.5), a.generate(0.5));
  }
  #[test]
  fn bump() {
    let factory = Factory;
    let a = factory.bump(1.0, (-1.0, 1.0), 1.0, 0.0);
    assert_eq!(Some(1.0), a.generate(0.5));
  }
}
