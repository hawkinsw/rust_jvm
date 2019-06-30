use enum_primitive::FromPrimitive;
use jvm::methodarea::MethodArea;
use jvm::class::Class;
use std::collections::HashMap;

enum_from_primitive! {
	enum OperandCodes {
		OPCODE_invokestatic = 0xb8,
		OPCODE_pop = 0x57,
	}
}

pub struct Vm {
	classes: HashMap<String, Class>,
	pc: usize,
}

impl Vm {
	pub fn new() -> Self {
		/*
		let mut methodarea = MethodArea::new();
		methodarea.add_class(main);
		*/
		Vm{classes: HashMap::new(), pc: 0}
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

	/* TODO: Start here by making it more real than it is now.
	*/
/*
	pub fn execute_method(&mut self, method: &str) -> bool {
		if let Some(method) = self.main.get_method(method.to_string()) {
			print!("Found method: {}", method);
			let mut pc_incr = self.execute_opcode();
			while pc_incr != 0 {
				print!("Doing next opcode\n");
				self.pc += pc_incr;
				pc_incr = self.execute_opcode();
			}
			true
		} else {
			false
		}
	}
*/
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
