use crate::factory::interface::FunctionOutput;

/// Function joins subfunction subdomains together to form a more
/// complex function. User can set delay to offset function in
/// time. Other functions can also be added to synthesize a more
/// complex response.
pub struct Function {
  piecewise: Vec<Box<dyn FunctionOutput>>,
  delay: f64,
  fcn: Vec<Function>,
  limits: (f64, f64),
}

impl Function {
  /// Creates a new function with default values
  pub fn new() -> Function {
    let out = Function {
      piecewise: Vec::new(),
      delay: 0.0,
      fcn: Vec::new(),
      limits: (0.0,0.0),
    };
    out
  }
  /// Creates a new function with a delay offset
  ///
  /// A positive delay pushes this function back relative to other functions
  /// and a negative value makes this function lead others.
  pub fn new_delay(delay: f64) -> Function {
    let out = Function {
      piecewise: Vec::new(),
      delay: delay,
      fcn: Vec::new(),
      limits: (0.0,0.0),
    };
    out
  }
  /// Supports addition of any output from the Factory
  pub fn add_subfunction(
    &mut self,
    sub: Box<dyn FunctionOutput>,
  ) {
    self.piecewise.push(sub);
    self.set_limits();
  }
  /// Supports addition of any other Function
  pub fn add_function(
    &mut self,
    fcn: Function,
  ) {
    self.fcn.push(fcn);
    self.set_limits();
  }
  /// Set the duration limits
  ///
  /// The parent function needs to know what the overall start/stop limits
  /// of all the functions are. This will change each time a sub-function
  /// is added to this function and when another function is added.
  ///
  /// It makes more sense to have multiple calls to this function during setup
  /// than while the user is requesting the function values.
  ///
  fn set_limits(&mut self) {
    // Find duration of the sub-functions first
    let mut start: f64 = self.delay;
    let mut stop: f64 = self.delay;
    for domain in self.piecewise.iter() {
      stop += domain.get_limits().1;
    }

    // Find durations of other functions in this tree
    for fcn in self.fcn.iter() {
      let fcn_lim = fcn.get_limits();
      if fcn_lim.0 < start {
        start = fcn_lim.0;
      }
      if fcn_lim.1 > stop {
        stop = fcn_lim.1;
      }
    }
    self.limits = (start, stop);
  }
}

impl FunctionOutput for Function {
  fn get_limits(&self) -> (f64,f64) {
    self.limits
  }
  /// Generate for functions
  ///
  /// The value that gets passed into this function uses zero as the starting
  /// reference point and progresses positively. Function trees that have delays
  /// that move their absolute starting point away from zero will be shifted so
  /// the overall function starts at zero.
  fn generate(
    &self,
    x: f64,
  ) -> Option<f64> {
    if (x < self.limits.0) || (x >= self.limits.1) {
      return None;
    }
    let mut result: f64 = 0.0;

    // function time is the value between the limits that the input time (referenced to zero)
    // corresponds too.
    let mut fcn_time = x - self.delay;

    // loop through this function first
    for domain in self.piecewise.iter() {
        result += match domain.generate(fcn_time) {
          None => 0.0,
          Some(y) => y,
        };
        fcn_time -= domain.get_limits().1;
    }

    // Now loop through other functions using the original input
    for fcn in self.fcn.iter() {
      result += match fcn.generate(x) {
        None => 0.0,
        Some(y) => y,
      };
    }
    Some(result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::factory;

  /// This is just a basic test of a single function with no delay and two sub-functions
  /// chained together. This also confirms that functions are valid over the same domain
  /// as sub-functions [0,duration)
  #[test]
  fn basic() {
    let mut a = Function::new();
    let factory = factory::Factory;

    a.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![7.0, 4.0, 8.0], false));
    a.add_subfunction(factory.polynomial(1.0, (-1.0, 1.0), vec![2.0, 0.0, -2.0], false));

    let x: Vec<f64> = vec![
      0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1., 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9 ];
    let y: Vec<f64> = vec![
      7., 7.48, 8.12, 8.92, 9.88, 11., 12.28, 13.72, 15.32, 17.08, 0., 0.72, 1.28, 1.68, 1.92, 2.,
      1.92, 1.68, 1.28, 0.72, 19.,
    ];

    for i in x.into_iter().enumerate() {
      // round output to two decimal places for accurate comparison
      assert_eq!(y[i.0], (a.generate(i.1).unwrap() * 100.0).round() / 100.0);
    }
    assert_eq!( None, a.generate( a.get_limits().1 ) );
  }

  /// Demonstrates that two functions stacked together with zero delay are handled properly
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
  /// Confirm that two functions add correctly when given delays
  ///        0.0      1.0         2.0         3.0
  /// fcn1             [-----------)
  /// fcn2    [--------------------------------)
  #[test]
  fn stack_delay1() {
    let factory = factory::Factory;
    let mut a = Function::new_delay(1.0);
    let mut b = Function::new_delay(0.0);
    a.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0], false));
    b.add_subfunction(factory.polynomial(3.0, (0.0, 1.0), vec![1.0], false));
    a.add_function( b );
    assert_eq!( Some(1.0), a.generate(0.50) );
    assert_eq!( Some(2.0), a.generate(1.00) );
    assert_eq!( Some(2.0), a.generate(1.50) );
    assert_eq!( Some(1.0), a.generate(2.00) );
    assert_eq!( Some(1.0), a.generate(2.50) );
  }
  /// Confirm that two functions add correctly when given delays
  ///        0.0      1.0         2.0         3.0
  /// fcn1    [--------------------------------)
  /// fcn2             [-----------)
  #[test]
  fn stack_delay2() {
    let factory = factory::Factory;
    let mut a = Function::new_delay(0.0);
    let mut b = Function::new_delay(1.0);
    a.add_subfunction(factory.polynomial(3.0, (0.0, 1.0), vec![1.0], false));
    b.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0], false));
    a.add_function( b );
    assert_eq!( Some(1.0), a.generate(0.50) );
    assert_eq!( Some(2.0), a.generate(1.00) );
    assert_eq!( Some(2.0), a.generate(1.50) );
    assert_eq!( Some(1.0), a.generate(2.00) );
    assert_eq!( Some(1.0), a.generate(2.50) );
  }
  /// Confirm that two functions add correctly when given delays
  ///       -1.0         0.0      1.0         2.0         3.0
  /// fcn1    [-----------)
  /// fcn2                         [-----------)
  #[test]
  fn stack_delay3() {
    let factory = factory::Factory;
    let mut a = Function::new_delay(-1.0);
    let mut b = Function::new_delay(1.0);
    a.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0], false));
    b.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0], false));
    a.add_function( b );
    assert_eq!( Some(1.0), a.generate(-1.00) );
    assert_eq!( Some(0.0), a.generate( 0.00) );
    assert_eq!( Some(1.0), a.generate( 1.00) );
    assert_eq!( None     , a.generate( 2.00) );
  }
  /// Confirm that two functions add correctly when given delays
  ///       -1.0         0.0      1.0         2.0         3.0
  /// fcn1                         [-----------)
  /// fcn2    [-----------)
  #[test]
  fn stack_delay4() {
    let factory = factory::Factory;
    let mut a = Function::new_delay(1.0);
    let mut b = Function::new_delay(-1.0);
    a.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0], false));
    b.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0], false));
    a.add_function( b );
    assert_eq!( Some(1.0), a.generate(-1.00) );
    assert_eq!( Some(0.0), a.generate( 0.00) );
    assert_eq!( Some(1.0), a.generate( 1.00) );
    assert_eq!( None     , a.generate( 2.00) );
  }
  /// Confirm that two functions add correctly when given delays
  ///       -1.0         0.0      1.0         2.0         3.0
  /// fcn1                [--------------------)
  /// fcn2    [--------------------)
  #[test]
  fn stack_delay5() {
    let factory = factory::Factory;
    let mut a = Function::new_delay(0.0);
    let mut b = Function::new_delay(-1.0);
    a.add_subfunction(factory.polynomial(2.0, (0.0, 1.0), vec![1.0], false));
    b.add_subfunction(factory.polynomial(2.0, (0.0, 1.0), vec![1.0], false));
    a.add_function( b );
    assert_eq!( Some(1.0), a.generate(-1.00) );
    assert_eq!( Some(2.0), a.generate( 0.00) );
    assert_eq!( Some(1.0), a.generate( 1.00) );
    assert_eq!( None     , a.generate( 2.00) );
  }
  /// Confirm that two functions add correctly when given delays
  ///       -1.0         0.0      1.0         2.0         3.0
  /// fcn1    [--------------------)
  /// fcn2                [--------------------)
  #[test]
  fn stack_delay6() {
    let factory = factory::Factory;
    let mut a = Function::new_delay(-1.0);
    let mut b = Function::new_delay(0.0);
    a.add_subfunction(factory.polynomial(2.0, (0.0, 1.0), vec![1.0], false));
    b.add_subfunction(factory.polynomial(2.0, (0.0, 1.0), vec![1.0], false));
    a.add_function( b );
    assert_eq!( Some(1.0), a.generate(-1.00) );
    assert_eq!( Some(2.0), a.generate( 0.00) );
    assert_eq!( Some(1.0), a.generate( 1.00) );
    assert_eq!( None     , a.generate( 2.00) );
  }
}
