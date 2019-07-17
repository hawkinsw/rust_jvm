use jvm::methodarea::MethodArea;
use jvm::method::Method;
use jvm::frame::Frame;
use jvm::typevalues::JvmTypeValue;
use jvm::typevalues::JvmPrimitiveTypeValue;
use jvm::typevalues::JvmPrimitiveType;
use enum_primitive::FromPrimitive;
use jvm::opcodes::OperandCodes;
use jvm::constant::Constant;
use std::rc::Rc;

pub struct JvmThread {
	debug: bool,
	methodarea: MethodArea,
	pc: usize,
}

enum OpcodeResult {
	Incr(usize),
	Value(JvmTypeValue),
}

impl JvmThread {
	pub fn new(debug: bool) -> Self {
		JvmThread{debug: debug, methodarea: MethodArea::new(debug), pc: 0}
	}

	pub fn run(&mut self, class_filename: &String, method_name: &String, args: &[String]) -> bool {
		/*
		 * 1. Create a method area.
		 * 2. Load the class into the method area.
		 * 3. Load the method.
		 * 4. Create a frame.
		 * 5. Load the frame with arguments.
		 * 6. Execute the method.
		 */
		if let Some(class) = self.methodarea.load_class_from_file(class_filename) {
			if self.debug {
				println!("Loaded class {}.\n", class);
			}
			if let Some(method) = class.get_method_ref_by_name(method_name) {
				let mut frame = Frame::new();
				frame.class = Some(Rc::clone(&class));
				/*
				 * Load up the frame's stack with the CLI arguments.
				 */
				frame.operand_stack.push(JvmTypeValue::Primitive(JvmPrimitiveTypeValue::new(JvmPrimitiveType::Boolean, 0)));

				if self.debug {
					println!("Frame: {}", frame);
				}

				self.execute_method(method, frame);
				return true
			}
		}
		false
	}

	fn execute_method(&mut self, method: &Method, mut frame: Frame) -> Option<JvmTypeValue> {
		let class = frame.class().unwrap();
		if let Some(code)=method.get_code(class.get_constant_pool_ref()) {
			let mut pc = 0;
			while {
				let mut pc_incr = 0;
				if self.debug {
					print!("Doing next opcode\n");
				}
				match self.execute_opcode(&code[pc ..], &mut frame) {
					OpcodeResult::Incr(incr) => pc_incr = incr,
					OpcodeResult::Value(v) => return Some(v)
				};
				pc += pc_incr;
				pc_incr != 0
			} {}
		}
		None
	}

	fn execute_opcode(&mut self, bytes: &[u8], frame: &mut Frame) -> OpcodeResult {
		let mut pc_incr: usize;
		let class = frame.class().unwrap();
		let constant_pool = class.get_constant_pool_ref();

		let opcode = bytes[0];
		if self.debug {
			print!("code: 0x{:X}\n", opcode);
		}
		match OperandCodes::from_u8(opcode) {
			Some(OperandCodes::OPCODE_iconst_m1) => {
				if self.debug {
					println!("iconst_m1");
				}
				self.execute_iconst_x(-1, frame);
				pc_incr = 1;
			},
			Some(OperandCodes::OPCODE_iconst_0) => {
				if self.debug {
					println!("iconst_0");
				}
				self.execute_iconst_x(0, frame);
				pc_incr = 1;
			},
			Some(OperandCodes::OPCODE_iconst_1) => {
				if self.debug {
					println!("iconst_1");
				}
				self.execute_iconst_x(1, frame);
				pc_incr = 1;
			},
			Some(OperandCodes::OPCODE_iconst_2) => {
				if self.debug {
					println!("iconst_2");
				}
				self.execute_iconst_x(2, frame);
				pc_incr = 1;
			},
			Some(OperandCodes::OPCODE_iconst_3) => {
				if self.debug {
					println!("iconst_3");
				}
				self.execute_iconst_x(3, frame);
				pc_incr = 1;
			},
			Some(OperandCodes::OPCODE_iconst_4) => {
				if self.debug {
					println!("iconst_4");
				}
				self.execute_iconst_x(4, frame);
				pc_incr = 1;
			},
			Some(OperandCodes::OPCODE_iconst_5) => {
				if self.debug {
					println!("iconst_5");
				}
				self.execute_iconst_x(5, frame);
				pc_incr = 1;
			},
			Some(OperandCodes::OPCODE_invokestatic) => {
				if self.debug {
					println!("invokestatic");
				}
				self.execute_invokestatic(bytes, frame);
				pc_incr = 3;
			},
			Some(OperandCodes::OPCODE_pop) => {
				if self.debug {
					println!("pop");
				}
				pc_incr = 1;
			},
			_ => {
				pc_incr = 0;
			}
		}
		OpcodeResult::Incr(pc_incr)
	}

	pub fn execute_iconst_x(&mut self, x: i64, frame: &mut Frame) {
		frame.operand_stack.push(JvmTypeValue::Primitive(
			JvmPrimitiveTypeValue::new(
				JvmPrimitiveType::Integer, x)));
	}

	pub fn execute_invokestatic(&mut self, bytes: &[u8], frame: &mut Frame) {
		let class = frame.class().unwrap();
		let constant_pool = class.get_constant_pool_ref();
		let method_index = (((bytes[1] as u16)<<8)|(bytes[2] as u16)) as usize;

		match constant_pool.get_constant_ref(method_index) {
			Constant::Methodref(_, class_index, method_index,) => {
				if let Constant::Class(_, class_name_index) = constant_pool.get_constant_ref(*class_index as usize) {
					if let Constant::NameAndType(_, method_name_index, _) = constant_pool.get_constant_ref(*method_index as usize) {
						if let Constant::Utf8(_, _, _, class_name) = constant_pool.get_constant_ref(*class_name_index as usize) {
							if let Constant::Utf8(_, _, _, method_name) = constant_pool.get_constant_ref(*method_name_index as usize) {
								if self.debug {
									println!("Invoke Static: {}.{}", class_name, method_name);
								}
								/*
								 * Steps:
								 * 1. Get the class containing the method.
								 * 2. Get the method.
								 * 3. Create a frame.
								 * 4. Populate the frame.
								 * 5. Execute the method
								 */
								if !self.methodarea.is_class_loaded(&class_name) {
									println!("We do need to load the class!");
									/*
									 * TODO
									 */
								} 

								if let Some(invoked_class) = self.methodarea.get_class_rc(&class_name) {
									if let Some(method) = invoked_class.get_methods_ref().get_by_name(method_name, invoked_class.get_constant_pool_ref()) {
										if self.debug {
											println!("method: {}", method);
										}
										let mut frame = Frame::new();
										frame.class = Some(Rc::clone(&invoked_class));
										self.execute_method(&method, frame);
									}
								} else {
									println!("Error: Could not execute static method {}.{}", class_name, method_name);
								}
							}
						}
					}
				}
			},
			_ => ()
		};
	}
}