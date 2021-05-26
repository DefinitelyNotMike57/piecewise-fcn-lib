use crate::factory;
use crate::factory::interface;

/// Function joins subfunction subdomains together to form a more
/// complex function. User can set delay to offset function in
/// time. Other functions can also be added to synthesize a more
/// complex response.
pub struct Function {
  piecewise: Vec<Box<dyn interface::SubfunctionOutput>>,
  delay: f64,
  fcn: Vec<Function>,
}

impl Function {
  pub fn new() -> Function {
    let out = Function {
      piecewise: Vec::new(),
      delay: 0.0,
      fcn: Vec::new(),
    };
    out
  }
  pub fn new_delay(delay: f64) -> Function {
    let out = Function {
      piecewise: Vec::new(),
      delay: delay,
      fcn: Vec::new(),
    };
    out
  }
  pub fn add_subfunction(
    &mut self,
    sub: Box<dyn interface::SubfunctionOutput>,
  ) {
    self.piecewise.push(sub);
  }
  pub fn add_function(
    &mut self,
    fcn: Function,
  ) {
    self.fcn.push(fcn);
  }
  fn get_duration(&self) -> f64 {
    let mut duration: f64 = self.delay;
    for domain in self.piecewise.iter() {
      duration += domain.get_duration();
    }
    for fcn in self.fcn.iter() {
      let temp = fcn.get_duration();
      if duration < temp {
        duration = temp;
      }
    }
    duration
  }
  pub fn generate(
    &self,
    x: f64,
  ) -> Option<f64> {
    if x >= self.get_duration() {
      return None;
    }
    let mut result: f64 = 0.0;
    let mut rel_time = x - self.delay;
    for domain in self.piecewise.iter() {
      if (rel_time < domain.get_duration()) && (rel_time >= 0.0) {
        result = domain.generate(rel_time).unwrap();
      }
      rel_time -= domain.get_duration();
    }
    for fcn in self.fcn.iter() {
      result += fcn.generate(x).unwrap();
    }
    Some(result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let mut a = Function::new();
    let factory = factory::Factory;

    a.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![7.0, 4.0, 8.0], false));
    a.add_subfunction(factory.polynomial(1.0, (-1.0, 1.0), vec![2.0, 0.0, -2.0], false));
    a.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![7.0, 4.0, 8.0], true));

    let x: Vec<f64> = vec![
      0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1., 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8,
      1.9, 2., 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9,
    ];
    let y: Vec<f64> = vec![
      7., 7.48, 8.12, 8.92, 9.88, 11., 12.28, 13.72, 15.32, 17.08, 0., 0.72, 1.28, 1.68, 1.92, 2.,
      1.92, 1.68, 1.28, 0.72, 19., 17.08, 15.32, 13.72, 12.28, 11., 9.88, 8.92, 8.12, 7.48, 7.,
    ];

    for i in x.into_iter().enumerate() {
      // round output to two decimal places for accurate comparison
      assert_eq!(y[i.0], (a.generate(i.1).unwrap() * 100.0).round() / 100.0);
    }
  }
  #[test]
  fn stacked() {
    let factory = factory::Factory;
    let mut a = Function::new();
    a.add_subfunction(factory.polynomial(1.0, (-1.0, 1.0), vec![2.0, 0.0, -2.0], false));
    let mut b = Function::new();
    b.add_subfunction(factory.polynomial(1.0, (-1.0, 1.0), vec![-2.0, 0.0, 2.0], false));
    a.add_function(b);

    for x in 0..100 {
      assert_eq!(Some(0.0), a.generate((x as f64) / 100.0));
    }
  }
  #[test]
  fn delay() {
    let factory = factory::Factory;
    let mut a = Function::new();
    a.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![2.0], false));
    let mut b = Function::new_delay(0.25);
    b.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![2.0], false));
    let mut c = Function::new_delay(0.5);
    c.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![2.0], false));
    let mut d = Function::new_delay(0.75);
    d.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![2.0], false));
    c.add_function(d);
    b.add_function(c);
    a.add_function(b);

    assert_eq!(Some(2.0), a.generate(0.125));
    assert_eq!(Some(4.0), a.generate(0.375));
    assert_eq!(Some(6.0), a.generate(0.625));
    assert_eq!(Some(8.0), a.generate(0.875));
    assert_eq!(Some(6.0), a.generate(1.125));
    assert_eq!(Some(4.0), a.generate(1.375));
    assert_eq!(Some(2.0), a.generate(1.625));
  }
}
