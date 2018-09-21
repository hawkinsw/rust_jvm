use std::fmt;
pub mod class;
pub mod constantpool;
pub mod constant;

pub struct Jvm {
	class_name: String,
	class: class::Class,
}

impl Jvm {
	pub fn new(class_with_path: &str) -> Jvm {
		return Jvm{class_name: class_with_path.to_string(),
		           class: class::Class::load(&class_with_path)
		          };
	}
	pub fn class(&self) -> &class::Class {
		&self.class
	}
}

impl fmt::Display for Jvm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "class: {}\n", self.class_name)
	}
}
