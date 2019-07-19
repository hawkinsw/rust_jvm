use std::fmt;
pub mod attribute;
pub mod class;
pub mod constant;
pub mod constantpool;
pub mod exceptions;
pub mod field;
pub mod frame;
pub mod jvmthread;
pub mod method;
pub mod methodarea;
pub mod opcodes;
pub mod typevalues;

pub struct Jvm {
	debug: bool,
}

impl Jvm {
	pub fn new(debug: bool) -> Option<Jvm> {
		Some(Jvm { debug: debug })
	}

	pub fn run(
		&self,
		start_class_filename: &String,
		start_function: &String,
		args: &[String],
	) -> bool {
		/*
		 * Create a VM and start running!
		 */
		let mut thread = jvmthread::JvmThread::new(self.debug);
		if thread.run(start_class_filename, start_function, args) {
			if self.debug {
				println!(
					"Success running {}.{}",
					start_class_filename, start_function
				);
			}
			return true;
		}
		if self.debug {
			println!(
				"Failure running {}.{}",
				start_class_filename, start_function
			);
		}
		false
	}
}

impl fmt::Display for Jvm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "debug: {}\n", self.debug)
	}
}
