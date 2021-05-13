
// Include the subfunctions here for the factory
pub use crate::subfunctions::polynomial::*;
pub use crate::subfunctions::bump::*;


/// This is the factory interface for making subfunctions
pub trait SubfunctionFactory {
    make_polynomial(&self,dur: f64, interval: (f64,f64), coeff: Vec<f64>, reverse: bool) -> Box<dyn SubfunctionOutput>;
    make_bump(&self, dur:f64, interval: (f64,f64), scale:f64, offset:f64 ) -> Box<dyn SubfunctionOutput>;
}
