use std::fmt;
pub mod class;
pub mod constantpool;
pub mod constant;
pub mod attribute;
pub mod field;
pub mod method;
pub mod exceptions;
pub mod vm;
pub mod methodarea;
pub mod frame;
pub mod opcodes;
pub mod typevalues;

pub struct Jvm {
	debug: bool,
}

impl Jvm {
	pub fn new(debug: bool) -> Option<Jvm> {
		Some(Jvm{debug: debug})
	}

	pub fn run(&self, start_class_filename: &String,
	                  start_function: &String,
	                  args: &[String]) -> bool {
		/*
		 * Create a VM and start running!
		 */
		let mut vm = vm::Vm::new(self.debug);
		if vm.run(start_class_filename, start_function, args) {
			if self.debug {
				println!("Success running {}.{}", start_class_filename, start_function);
			}
			return true
		}
		if self.debug {
			println!("Failure running {}.{}", start_class_filename, start_function);
		}
		false
	}
}

impl fmt::Display for Jvm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "debug: {}\n", self.debug)
	}
}
