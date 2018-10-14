use jvm::class::Class;
use std::collections::HashMap;

pub struct MethodArea {
	pub classes: HashMap<String, Box<Class>>,
}

impl MethodArea {
	pub fn new() -> Self{
		MethodArea{classes: HashMap::new()}
	}

	pub fn add_class(&mut self, class: & Box<Class>) {
	}
}
