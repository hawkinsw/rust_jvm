use jvm::methodarea::MethodArea;
use jvm::method::Method;
use jvm::frame::Frame;
use jvm::typevalues::JvmTypeValue;
use jvm::typevalues::JvmPrimitiveTypeValue;
use jvm::typevalues::JvmPrimitiveType;
use enum_primitive::FromPrimitive;
use jvm::opcodes::OperandCodes;

pub struct JvmThread {
	debug: bool,
	methodarea: MethodArea,
	pc: usize,
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
			if let Some(method) = class.get_method(method_name) {
				let mut frame = Frame::new();
				/*
				 * Load up the frame's stack with the CLI arguments.
				 */
				frame.operand_stack.push(JvmTypeValue::Primitive(JvmPrimitiveTypeValue::new(JvmPrimitiveType::Boolean, 0)));
				frame.constant_pool = Some(class.get_constant_pool());

				if self.debug {
					println!("Frame: {}", frame);
				}

				return self.execute_method(method, frame);
			}
		}
		false
	}

	fn execute_method<'b>(&mut self, method: &Method, frame: Frame<'b>) -> bool {
		if let Some(code)=method.get_code_attribute(frame.constant_pool.unwrap()) {
			if self.debug {
				println!("Method's code attribute:\n{}", code);
			}
			self.pc = code.code_offset + 0;
			let mut pc_incr = self.execute_opcode(&code.bytes[self.pc ..], &frame);
			while pc_incr != 0 {
				if self.debug {
					print!("Doing next opcode\n");
				}
				self.pc += pc_incr;
				pc_incr = self.execute_opcode(&code.bytes[self.pc ..], &frame);
			}
		}
		return true
	}

	fn execute_opcode(&self, bytes: &[u8], frame: &Frame) -> usize {
		let mut pc_incr: usize;

		let opcode = bytes[0];
		if self.debug {
			print!("code: 0x{:X}\n", opcode);
		}
		match OperandCodes::from_u8(opcode) {
			Some(OperandCodes::OPCODE_invokestatic) => {
				if self.debug {
					print!("invokestatic\n");
				}
				let method_index: u16 = ((bytes[1] as u16)<<8)|(bytes[2] as u16);
				print!("method_index: {:x}\n", method_index);
				let constant = frame.constant_pool.unwrap().get(method_index as usize);
				if self.debug {
					println!("constant: {}", constant);
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
}
