
/// This defines the common interface for all piecewise types
///
///
pub trait SubfunctionOutput {
    // Subfunction should be able to report how long it is
    fn get_duration(&self) -> f64;
    // Actually generate the function
    fn generate(&self, x: f64) -> Option<f64>;
}

