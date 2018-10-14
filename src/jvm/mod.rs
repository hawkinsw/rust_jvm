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
	class_name: String,
	main_class: Box<class::Class>,
	debug: bool,
}

impl Jvm {
	pub fn new(class_with_path: &str, debug: bool) -> Option<Jvm> {
		if let Some(class) = class::Class::load(&class_with_path) {
			Some(Jvm{class_name: class_with_path.to_string(),
		           main_class: class,
			         debug: debug})
		} else {
			None
		}
	}
	pub fn class(&self) -> &Box<class::Class> {
		&self.main_class
	}
}

impl fmt::Display for Jvm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "class: {}\n", self.class_name)
	}
}
