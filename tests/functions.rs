use piecewise_fcn_lib as pw;

/// Helping to understand multi-type sub-function functions
///
/// Any sub-function available from the factory can be added to the function
/// in any order. They can be chained together such that the generated values
/// look continuous or they can discontinuous like with this example.
///
#[test]
fn function_chain() {
  let factory = pw::factory::Factory;
  let mut fcn1 = pw::function::Function::new();
  fcn1.add_subfunction(factory.polynomial(2.0, (-1.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn1.add_subfunction(factory.bump(4.0, (-2.0, 2.0), 1.0, 0.0));

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/function_chain.csv".to_string(),
  );
}

/// Helping to understand stacked functions
///
///
#[test]
fn function_stacked1() {
  let factory = pw::factory::Factory;
  let mut fcn1 = pw::function::Function::new();
  let mut fcn2 = pw::function::Function::new();
  fcn1.add_subfunction(factory.polynomial(2.0, (-1.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn2.add_subfunction(factory.bump(4.0, (-2.0, 2.0), 1.0, 0.0));
  fcn1.add_function(fcn2);

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/function_stacked1.csv".to_string(),
  );
}

/// Helping to understand stacked functions
///
///
#[test]
fn function_stacked2() {
  let factory = pw::factory::Factory;
  let mut fcn1 = pw::function::Function::new();
  let mut fcn2 = pw::function::Function::new();
  fcn1.add_subfunction(factory.polynomial(2.0, (-1.0, 1.0), vec![0.0, 0.0, 1.0], false));
  fcn2.add_subfunction(factory.bump(4.0, (-2.0, 2.0), 1.0, 0.0));
  fcn2.add_function(fcn1);

  pw::write_to_file(
    Box::new(fcn2),
    1000.0,
    "tests/output/function_stacked2.csv".to_string(),
  );
}
