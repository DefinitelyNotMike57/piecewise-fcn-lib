use piecewise_fcn_lib as pw;

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
    fcn1.add_subfunction( factory.polynomial( 1.0,(0.0,1.0),vec![0.0,0.0,1.0],false) );
    fcn1.add_subfunction( factory.bump( 1.0,(0.0,1.0),2.0,0.0) );

    let sample_rate_hz: f64 = 1000.0;
    let num_samples: u64 = (fcn1.get_duration() * sample_rate_hz).floor() as u64;

    let path = Path::new( "tests/output/example1.csv" );
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    for step in 0..num_samples {
        let time = step as f64 / sample_rate_hz;
        let amp = fcn1.generate( time );

        match file.write_all( format!("{},{}\n", time, amp.unwrap()).as_bytes() ) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => (),
        }
    }
}
