use jvm::methodarea::MethodArea;
use jvm::method::Method;
use jvm::class::Class;
use std::collections::HashMap;
use jvm::frame::Frame;
use jvm::typevalues::JvmTypeValue;
use jvm::typevalues::JvmPrimitiveTypeValue;
use jvm::typevalues::JvmPrimitiveType;

pub struct Vm {
	debug: bool
}

impl Vm {
	pub fn new(debug: bool) -> Self {
		Vm{debug: debug}
	}

	pub fn run(&self, class_filename: &String, method_name: &String, args: &[String]) -> bool {
		/*
		 * 1: Create a method area.
		 * 3. Load the class into the method area.
		 * 5. Go!
		 */
		let mut frame = Frame::new();
		let mut method_area = MethodArea::new(self.debug);
		if let Some(class_name) = method_area.load_class_from_file(class_filename) {
			if self.debug {
				println!("Loaded class {}.\n", class_name);
			}

			/*
			 * Load up the frame's stack with the CLI arguments.
			 */
			frame.operand_stack.push(JvmTypeValue::Primitive(JvmPrimitiveTypeValue::new(JvmPrimitiveType::Boolean, 0)));

			if self.debug {
				println!("Frame: {}", frame);
			}

			return method_area.execute_method(&class_name, method_name, &frame);
		}
		true
	}
}
