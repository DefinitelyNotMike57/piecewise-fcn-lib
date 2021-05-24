mod bump;
mod interface;
mod polynomial;

use interface::SubfunctionOutput;

/// This is the factory interface for making subfunctions
pub trait SubfunctionFactory {
  fn polynomial(
    &self,
    dur: f64,
    interval: (f64, f64),
    coeff: Vec<f64>,
    reverse: bool,
  ) -> Box<dyn SubfunctionOutput>;
  fn bump(
    &self,
    dur: f64,
    interval: (f64, f64),
    scale: f64,
    offset: f64,
  ) -> Box<dyn SubfunctionOutput>;
}

struct Factory;
/*
impl SubfunctionFactory for Factory {
    fn polynomial(&self,dur: f64, interval: (f64,f64), coeff: Vec<f64>, reverse: bool) -> Box<dyn SubfunctionOutput> {
        let new = Polynomial::new( dur, interval, coeff, reverse );
        new
    }
    fn bump(&self, dur:f64, interval: (f64,f64), scale:f64, offset:f64 ) -> Box<dyn SubfunctionOutput> {
        let new = Bump::new( dur, interval, scale, offset );
        new
    }
}
*/

#[cfg(test)]
mod factory {
  use super::*;
}
