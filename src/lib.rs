//! # Piecewise Functions
//!
//! This library provides tools for the user to synthesize piecewise-functions
//! using stacked sub-functions. Current sub-functions include user defined
//! the bump function.
//!
//! # Hello World
//!
//! ```
//!
//!
//! ```
//!

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

/// Module that contains factory for generating subfunctions
pub mod factory;

/// Module that manages functions
pub mod function;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Utility for writing functions to a file
///
///
pub fn write_to_file(
  fcn: Box<dyn factory::interface::FunctionOutput>,
  sample_rate_hz: f64,
  file_name: String,
) {
  // Use floor because a value >= to the exact duration will return None
  // and floor is the safer bet.
  let num_samples: u64 = (fcn.get_duration() * sample_rate_hz).floor() as u64;

  let path = Path::new(&file_name);
  let display = path.display();

  let mut file = match File::create(&path) {
    Err(why) => panic!("Couldn't create {}: {}", display, why),
    Ok(file) => file,
  };

  for step in 0..num_samples {
    let time = step as f64 / sample_rate_hz;
    let amp = fcn.generate(time);

    match file.write_all(format!("{},{}\n", time, amp.unwrap()).as_bytes()) {
      Err(why) => panic!("Couldn't write to {}: {}", display, why),
      Ok(_) => (),
    }
  }
}
