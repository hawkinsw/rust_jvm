use jvm::class::Class;
use jvm::frame::Frame;
use std::collections::HashMap;
use jvm::method::Method;
use jvm::method::Methods;
use jvm::method::MethodIterator;
use jvm::opcodes::OperandCodes;
use enum_primitive::FromPrimitive;

pub struct MethodArea<'a> {
	pub debug: bool,
	pub classes: HashMap<String, Class>,
	pub stack: Vec<&'a Frame<'a>>,
	pub pc: usize
}

impl<'a> MethodArea<'a> {
	pub fn new(debug: bool) -> Self {
		MethodArea{debug: debug, classes: HashMap::new(), stack: Vec::<&'a Frame<'a>>::new(), pc: 0}
	}

	pub fn execute_method(&mut self, class_name: &String, method_name: &String, frame: &'a Frame<'a>) -> bool {
		if let Some(class) = self.classes.get(class_name) {
			if let Some(method) = class.get_method(method_name) {
				if self.debug {
					println!("Method: {}", method);
				}
				self.stack.push(frame);
				if let Some(code)=method.get_code_attribute(class.get_constant_pool()) {
					if self.debug {
						println!("Method {}'s code attribute:\n{}", method_name, code);
					}
					self.pc = code.code_offset + 0;
					let mut pc_incr = self.execute_opcode(&code.bytes[self.pc ..]);
					while pc_incr != 0 {
						if self.debug {
							print!("Doing next opcode\n");
						}
						self.pc += pc_incr;
						pc_incr = self.execute_opcode(&code.bytes[self.pc ..]);
					}
				}
				self.stack.pop();
				return true
			}
		}
		false
	}

	pub fn execute_opcode(&self, bytes: &[u8]) -> usize {
		let mut pc_incr: usize = 0;

		let opcode = bytes[0];
		if self.debug {
			print!("code: 0x{:X}\n", opcode);
		}
		match OperandCodes::from_u8(opcode) {
			Some(OperandCodes::OPCODE_invokestatic) => {
				if self.debug {
					print!("invokestatic\n");
				}
				pc_incr = 3;
			},
			Some(OperandCodes::OPCODE_pop) => {
				if self.debug {
					print!("pop\n");
				}
				pc_incr = 1;
			},
			_ => {
				pc_incr = 0;
			}
		}
		pc_incr
	}

	pub fn load_class_from_file(&mut self, class_filename: &String) -> Option<String> {
		if let Some(class) = Class::load(class_filename) {
			if let Some(class_name) = class.get_name() {
				self.classes.insert(class_name.to_string(), class);
				return Some(class_name);
			}
		}
		None
	}
}
