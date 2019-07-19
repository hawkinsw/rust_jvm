use jvm::class::Class;
use std::collections::HashMap;
use std::rc::Rc;

pub struct MethodArea {
	pub debug: bool,
	pub classes: HashMap<String, Rc<Class>>,
}

impl MethodArea {
	pub fn new(debug: bool) -> Self {
		MethodArea {
			debug: debug,
			classes: HashMap::new(),
		}
	}

	pub fn is_class_loaded(&self, class_name: &String) -> bool {
		self.classes.contains_key(class_name)
	}

	pub fn get_class_rc(&self, class_name: &String) -> Option<Rc<Class>> {
		if let Some(class_rc_ref) = self.classes.get(class_name) {
			Some(Rc::clone(class_rc_ref))
		} else {
			None
		}
	}

	pub fn load_class_from_file(&mut self, class_filename: &String) -> Option<Rc<Class>> {
		if let Some(class) = Class::load(class_filename) {
			if let Some(class_name) = class.get_class_name() {
				self.classes.insert(class_name.to_string(), Rc::new(class));
				return Some(Rc::clone(self.classes.get(&class_name).unwrap()));
			}
		}
		None
	}
}
