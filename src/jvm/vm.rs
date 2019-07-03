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
	debug: bool
}

impl Vm {
	pub fn new(debug: bool) -> Self {
		/*
		let mut methodarea = MethodArea::new();
		methodarea.add_class(main);
		*/
		Vm{debug: debug}
	}

	pub fn run(&self, class_filename: &String, method_name: &String) -> bool {
		/*
		 * 1: Get a method area. 
		 * 2: Get a stack.
		 * 3. Load the class into the method area.
		 * 4. Load the method from the class in the method area.
		 * 5. Go!
		 */
		let mut method_area = MethodArea::new();
		let mut stack = Stack::new();

		if let Some(class_name) = method_area.load_class_from_file(class_filename) {
			println!("Loaded: {}\n", class_name);
			if let Some(class) = method_area.get_class_by_name(&class_name) {
				if let Some(method) = class.get_method(method_name) {
					return self.go(&method_area, stack, method)
				}
			}
		}
		false
	}

	pub fn go(&self, mut methodarea: &MethodArea, mut stack: Stack, method: &Method) -> bool {
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
