use piecewise_fcn_lib as pw;

/// Helping to understand duration
///
/// The duration value that is used to implement all of the factory outputs
/// is essentially a scaling function for the x-axis. This simple demonstration
/// of the same polynomial (x^2) from -1 to 1 scaled for different durations
/// will hopefully make that clear.
///
#[test]
fn polynomial_duration() {
  let factory = pw::Factory;
  let mut fcn1 = pw::Function::new();
  fcn1.add_subfunction(factory.polynomial(0.5, (-1.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (-1.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.polynomial(2.0, (-1.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.polynomial(4.0, (-1.0, 1.0), vec![0.0, 0.0, 1.0], false));

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/polynomial_duration.csv".to_string(),
  );
}

/// Helping to understand domain
///
/// The domain is the region that the polynomial is definied over. For this
/// example we are still using the (x^2) polynomial but each sub-function is
/// using a different domain of that function and mapping it to a consistent
/// region.
#[test]
fn polynomial_domain() {
  let factory = pw::Factory;
  let mut fcn1 = pw::Function::new();
  fcn1.add_subfunction(factory.polynomial(1.0, (-1.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (-1.0, 0.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (-5.0, 5.0), vec![0.0, 0.0, 1.0], false));

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/polynomial_domain.csv".to_string(),
  );
}

/// Helping to understand coefficients
///
/// The index of each coefficient is the power it corresponds to. So [1.0,2.0,3.0]
/// results in 1.0*x^0 + 2.0*x^1 + 3.0*x^2 and so on.
#[test]
fn polynomial_coefficients() {
  let factory = pw::Factory;
  let mut fcn1 = pw::Function::new();
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0, 2.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0, 2.0, 3.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0, 2.0, 3.0, 4.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![1.0, 2.0, 3.0, 4.0, 5.0], false));
  fcn1.add_subfunction(factory.polynomial(
    1.0,
    (0.0, 1.0),
    vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    false,
  ));

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/polynomial_coefficients.csv".to_string(),
  );
}

/// Helping to understand the flip
///
/// The flip flag was meant to make life easier on the user by allowing them to
/// reverse the generation of the polynomial. We'll see if it is useful moving
/// forward.
#[test]
fn polynomial_flip() {
  let factory = pw::Factory;
  let mut fcn1 = pw::Function::new();
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.polynomial(1.0, (0.0, 1.0), vec![0.0, 0.0, 1.0], true));

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/polynomial_flip.csv".to_string(),
  );
}
