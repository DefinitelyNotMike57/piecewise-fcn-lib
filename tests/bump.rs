use piecewise_fcn_lib as pw;

/// Helping to understand duration
///
/// The duration value that is used to implement all of the factory outputs
/// is essentially a scaling function for the x-axis. This simple demonstration
/// of the same bump from -1 to 1 scaled for different durations
/// will hopefully make that clear.
///
#[test]
fn bump_duration() {
  let factory = pw::factory::Factory;
  let mut fcn1 = pw::function::Function::new();
  fcn1.add_subfunction(factory.bump(1.0, (-1.0, 1.0), 1.0, 0.0));
  fcn1.add_subfunction(factory.bump(2.0, (-1.0, 1.0), 1.0, 0.0));
  fcn1.add_subfunction(factory.bump(4.0, (-1.0, 1.0), 1.0, 0.0));
  fcn1.add_subfunction(factory.bump(8.0, (-1.0, 1.0), 1.0, 0.0));

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/bump_duration.csv".to_string(),
  );
}

/// Helping to understand interval
///
/// The interval definition is different than it is for the polynomial
/// because the bump function is only valid between -1 and 1 and all values
/// outside of that domain are set to 0. That is why the last sub-function
/// looks like a compressed version of the first sub-function, the bump is
/// only valid between 1/3 and 2/3 of the 1.0 duration.
///
#[test]
fn bump_interval() {
  let factory = pw::factory::Factory;
  let mut fcn1 = pw::function::Function::new();
  fcn1.add_subfunction(factory.bump(1.0, (-1.0, 1.0), 1.0, 0.0));
  fcn1.add_subfunction(factory.bump(1.0, (-1.0, 0.0), 1.0, 0.0));
  fcn1.add_subfunction(factory.bump(1.0, (0.0, 1.0), 1.0, 0.0));
  fcn1.add_subfunction(factory.bump(1.0, (-2.0, 2.0), 1.0, 0.0));

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/bump_interval.csv".to_string(),
  );
}

/// Helping to understand scale/offset
///
/// Scale and offset do exactly what they sound like on the y-axis and they are
/// combined here so the user can see their order-of-operations. The function is
/// always scaled first and then the offset is applied.
///
#[test]
fn bump_scale_offset() {
  let factory = pw::factory::Factory;
  let mut fcn1 = pw::function::Function::new();
  fcn1.add_subfunction(factory.bump(1.0, (-1.0, 1.0), 1.0, 0.0));
  fcn1.add_subfunction(factory.bump(1.0, (-1.0, 1.0), 2.0, 0.0));
  fcn1.add_subfunction(factory.bump(1.0, (-1.0, 1.0), 1.0, 1.0));
  fcn1.add_subfunction(factory.bump(1.0, (-1.0, 1.0), 2.0, 1.0));

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/bump_scale_offset.csv".to_string(),
  );
}
