use jvm::class::Class;
use jvm::frame::Frame;
use std::collections::HashMap;
use jvm::method::Method;

pub struct MethodArea {
	pub classes: HashMap<String, Class>,
	pub stack: Vec<Frame>,
}

impl MethodArea {
	pub fn new() -> Self{
		MethodArea{classes: HashMap::new(), stack: Vec::<Frame>::new()}
	}

	pub fn get_class_by_name(&self, class_name: &String) -> Option<&Class> {
		self.classes.get(class_name)
	}

	pub fn load_class_from_file(&mut self, class_filename: &String) -> Option<String> {
		if let Some(class) = Class::load(class_filename) {
			if let Some(class_name) = class.get_name() {
				self.classes.insert(class_name.to_string(), class);
				return Some(class_name.to_string())
			}
		}
		None
	}
}
