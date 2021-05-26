use piecewise_fcn_lib as pw;
use pw::factory::interface::FunctionOutput;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// A simple piecewise function
///
/// Create a function with different sub-function types that are not consistent across their
/// domain interfaces and export the result to CSV so the user can visualize it.
///
#[test]
fn example1() {
  let factory = pw::factory::Factory;
  let mut fcn1 = pw::function::Function::new();
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.bump(1.0, (0.0, 1.0), 2.0, 0.0));

  let sample_rate_hz: f64 = 1000.0;
  pw::write_to_file( Box::new(fcn1), 1000.0, "tests/output/example1.csv".to_string() );

}
