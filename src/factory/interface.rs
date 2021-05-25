/// This defines the common interface for all subfunction types
///
/// All subfunctions should be able to report how long they are and based on some input should be
/// able to generate an output.
pub trait SubfunctionOutput {
  /// Subfunction should be able to report how long it is
  fn get_duration(&self) -> f64;
  /// Actually generate the function
  fn generate(
    &self,
    x: f64,
  ) -> Option<f64>;
}
