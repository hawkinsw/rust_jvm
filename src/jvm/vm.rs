use enum_primitive::FromPrimitive;
use jvm::methodarea::MethodArea;
use jvm::method::Method;
use jvm::class::Class;
use jvm::stack::Stack;
use std::collections::HashMap;

enum_from_primitive! {
	enum OperandCodes {
		OPCODE_invokestatic = 0xb8,
		OPCODE_pop = 0x57,
	}
}

pub struct Vm {
	classes: HashMap<String, Class>,
	debug: bool
}

impl Vm {
	pub fn new(debug: bool) -> Self {
		/*
		let mut methodarea = MethodArea::new();
		methodarea.add_class(main);
		*/
		Vm{classes: HashMap::new(), debug: debug}
	}

	pub fn load_class(&mut self, name: &str, filename: &str) -> bool {
		println!("filename: {}", filename);
		if let Some(class) = Class::load(&filename) {
			self.classes.insert(name.to_string(), class);
			true
		} else {
			false
		}
	}

	pub fn run(&self, class_name: &String, method_name: &String) -> bool {
		if let Some(class) = self.classes.get(class_name) {
			if let Some(method) = class.get_method(method_name) {
				if self.debug {
					println!("Found method: {}", method);
				}
				/*
				 * We need some stack.
				 */
				let stack = Stack::new();
				self.execute_method_by_method(method)
			} else {
				false
			}
		}
		else {
			false
		}
	}

	pub fn execute_method_by_method(&self, method: &Method) -> bool {
		true
	/*
		let mut pc_incr = self.execute_opcode();
		while pc_incr != 0 {
			print!("Doing next opcode\n");
			self.pc += pc_incr;
			pc_incr = self.execute_opcode();
		}
	*/
	}

/*
	pub fn execute_opcode(&mut self) -> usize {
		let mut pc_incr: usize = 0;
		if let Some(main_method) = self.main.get_method("main".to_string()) {
			if let Some(code) = main_method.get_code_attribute(&(*self.main).get_constant_pool()) {
				let opcode = code.bytes[code.code_offset + self.pc];
				print!("code: 0x{:X}\n", opcode);
				match OperandCodes::from_u8(opcode) {
					Some(OperandCodes::OPCODE_invokestatic) => {
						print!("invokestatic\n");
						pc_incr = 3;
					},
					Some(OperandCodes::OPCODE_pop) => {
						print!("pop\n");
						pc_incr = 1;
					},
					_ => {
						pc_incr = 0;
					}
				}
			}
		}
		pc_incr
	}
*/
	pub fn execute(&mut self) -> bool {
		true
	}
}
