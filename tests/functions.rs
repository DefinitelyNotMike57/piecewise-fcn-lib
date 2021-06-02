use piecewise_fcn_lib as pw;

/// Helping to understand multi-type sub-function functions
///
/// Any sub-function available from the factory can be added to the function
/// in any order. They can be chained together such that the generated values
/// look continuous or they can discontinuous like with this example.
///
#[test]
fn function_chain() {
  let factory = pw::Factory;
  let mut fcn1 = pw::Function::new();
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
/// When functions are added together, their overall duration is the super-set of all
/// functions. This example has a base function of length 2.0 and an additional function
/// of length 4.0 with a delay of -1.0
///
#[test]
fn function_stacked1() {
  let factory = pw::Factory;
  let mut fcn1 = pw::Function::new();
  let mut fcn2 = pw::Function::new_delay(-1.0);
  fcn1.add_subfunction(factory.polynomial(2.0, (-1.0, 1.0), vec![1.0], false));
  fcn2.add_subfunction(factory.polynomial(4.0, (-1.0, 1.0), vec![1.0], false));
  fcn1.add_function(fcn2);

  pw::write_to_file(
    Box::new(fcn1),
    1000.0,
    "tests/output/function_stacked1.csv".to_string(),
  );
}
