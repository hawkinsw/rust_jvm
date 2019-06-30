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

pub struct Jvm {
	debug: bool,
}

impl Jvm {
	pub fn new(debug: bool) -> Option<Jvm> {
		Some(Jvm{debug: debug})
	}

	pub fn run(&self, start_class_filename: &String,
	                  start_class: &String,
	                  start_function: &String) -> bool {
		let mut vm = vm::Vm::new();
		if (vm.load_class(start_class, start_class_filename)) {
			if (self.debug) {
				println!("Success executing {}.{}", start_class, start_function);
			}
			true
		} else {
			if (self.debug) {
				println!("Success executing {}.{}", start_class, start_function);
			}
			false
		}
	}
}

impl fmt::Display for Jvm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "debug: {}\n", self.debug)
	}
}
