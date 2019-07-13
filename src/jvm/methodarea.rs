use jvm::class::Class;
use std::rc::Rc;
use jvm::frame::Frame;
use std::collections::HashMap;
use jvm::method::Method;
use jvm::method::Methods;
use jvm::method::MethodIterator;
use jvm::opcodes::OperandCodes;
use enum_primitive::FromPrimitive;

pub struct MethodArea {
	pub debug: bool,
	pub classes: HashMap<String, Rc<Class>>,
}

impl MethodArea {
	pub fn new(debug: bool) -> Self {
		MethodArea{debug: debug, classes: HashMap::new()}
	}

	pub fn load_class_from_file(&mut self, class_filename: &String) -> Option<Rc<Class>> {
		if let Some(class) = Class::load(class_filename) {
			if let Some(class_name) = class.get_name() {
				self.classes.insert(class_name.to_string(), Rc::new(class));
				return Some(Rc::clone(self.classes.get(&class_name).unwrap()));
			}
		}
		None
	}
}
