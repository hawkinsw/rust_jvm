/*
 * FILE: XXXXX
 * DESCRIPTION:
 *
 * Copyright (c) 2019, Will Hawkins
 *
 * This file is part of Rust-JVM.
 *
 * Rust-JVM is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rust-JVM is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Rust-JVM.  If not, see <https://www.gnu.org/licenses/>.
 */
use enum_primitive::FromPrimitive;
use jvm::constant::Constant;
use jvm::environment::Environment;
use jvm::frame::Frame;
use jvm::method::Method;
use jvm::method::MethodAccessFlags;
use jvm::methodarea::MethodArea;
use jvm::opcodes::OperandCodes;
use jvm::typevalues::JvmPrimitiveType;
use jvm::typevalues::JvmPrimitiveTypeValue;
use jvm::typevalues::JvmTypeValue;
use std::fs;
use std::path::Path;
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
		JvmThread {
			debug: debug,
			methodarea: MethodArea::new(debug),
			pc: 0,
		}
	}

	pub fn run(
		&mut self,
		class_name: &String,
		method_name: &String,
		environment: &Environment,
	) -> bool {
		/*
		 * 1. Create a method area.
		 * 2. Load classes in the classpath  into the method area.
		 * 3. Load the method.
		 * 4. Create a frame.
		 * 5. Load the frame with arguments.
		 * 6. Execute the method.
		 */

		for path in environment.classpath {
			if let Ok(dir_list) = fs::read_dir(Path::new(path)) {
				for class_entry in dir_list {
					if let Ok(class_entry) = class_entry {
						if let Some(class_filename) = class_entry.path().to_str() {
							let class_filename = class_filename.to_string();
							if class_filename.ends_with("class") {
								if self.debug {
									println!("Loading class file {}", class_filename);
								}
								self.methodarea.load_class_from_file(&class_filename);
							}
						}
					}
				}
			}
		}

		if let Some(class) = self.methodarea.get_class_rc(class_name) {
			if self.debug {
				println!("Loaded class {}.\n", class);
			}
			if let Some(method) = class.get_method_ref_by_name(method_name) {
				if method.access_flags
					!= ((MethodAccessFlags::Public as u16) | (MethodAccessFlags::Static as u16))
				{
					if self.debug {
						println!("Main method is not public and static.");
					}
					assert!(false, "Main method is not public and static.");
				}

				let mut frame = Frame::new();
				frame.class = Some(Rc::clone(&class));
				/*
				 * Load up the frame's stack with the CLI arguments.
				 */
				frame
					.operand_stack
					.push(JvmTypeValue::Primitive(JvmPrimitiveTypeValue::new(
						JvmPrimitiveType::Boolean,
						0,
					)));

				if self.debug {
					println!("Frame: {}", frame);
				}

				self.execute_method(method, frame);
				return true;
			}
		}
		false
	}

	fn execute_method(&mut self, method: &Method, mut frame: Frame) -> Option<JvmTypeValue> {
		let class = frame.class().unwrap();
		if let Some(code) = method.get_code(class.get_constant_pool_ref()) {
			let mut pc = 0;
			while {
				let mut pc_incr = 0;
				if self.debug {
					print!("Doing next opcode\n");
				}
				match self.execute_opcode(&code[pc..], &mut frame) {
					OpcodeResult::Incr(incr) => pc_incr = incr,
					OpcodeResult::Value(v) => return Some(v),
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
			}
			Some(OperandCodes::OPCODE_iconst_0) => {
				if self.debug {
					println!("iconst_0");
				}
				self.execute_iconst_x(0, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iconst_1) => {
				if self.debug {
					println!("iconst_1");
				}
				self.execute_iconst_x(1, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iconst_2) => {
				if self.debug {
					println!("iconst_2");
				}
				self.execute_iconst_x(2, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iconst_3) => {
				if self.debug {
					println!("iconst_3");
				}
				self.execute_iconst_x(3, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iconst_4) => {
				if self.debug {
					println!("iconst_4");
				}
				self.execute_iconst_x(4, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iconst_5) => {
				if self.debug {
					println!("iconst_5");
				}
				self.execute_iconst_x(5, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iload_0) => {
				if self.debug {
					println!("iload_0");
				}
				self.execute_iload_x(0, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iload_1) => {
				if self.debug {
					println!("iload_1");
				}
				self.execute_iload_x(1, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iload_2) => {
				if self.debug {
					println!("iload_2");
				}
				self.execute_iload_x(2, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iload_3) => {
				if self.debug {
					println!("iload_3");
				}
				self.execute_iload_x(3, frame);
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_ireturn) => {
				if self.debug {
					println!("ireturn");
				}
				return OpcodeResult::Value((frame.operand_stack.pop().unwrap()));
			}
			Some(OperandCodes::OPCODE_invokestatic) => {
				if self.debug {
					println!("invokestatic");
				}

				/*
				 * Start by assuming failure.
				 */
				pc_incr = 0;

				let invokestatic_result = self.execute_invokestatic(bytes, frame);
				if let Some(OpcodeResult::Value(v)) = invokestatic_result {
					/*
					 * Push the result of the invocation onto
					 * the operand stack.
					 */
					frame.operand_stack.push(v);
					pc_incr = 3;
				}
			}
			Some(OperandCodes::OPCODE_pop) => {
				if self.debug {
					println!("pop");
				}
				frame.operand_stack.pop();
				pc_incr = 1;
			}
			Some(OperandCodes::OPCODE_iadd) => {
				if self.debug {
					println!("iadd");
				}
				self.execute_iadd(frame);
				pc_incr = 1;
			}
			_ => {
				assert!(false, "Unrecognized opcode: 0x{:x}", opcode);
				pc_incr = 0;
			}
		}
		OpcodeResult::Incr(pc_incr)
	}

	pub fn execute_iadd(&mut self, frame: &mut Frame) {
		if let Some(JvmTypeValue::Primitive(op1_primitive)) = frame.operand_stack.pop() {
			if let Some(JvmTypeValue::Primitive(op2_primitive)) = frame.operand_stack.pop() {
				frame
					.operand_stack
					.push(JvmTypeValue::Primitive(JvmPrimitiveTypeValue::new(
						JvmPrimitiveType::Integer,
						op1_primitive.value + op2_primitive.value,
					)));
			}
		}
	}

	pub fn execute_iload_x(&mut self, x: usize, frame: &mut Frame) {
		frame.operand_stack.push(frame.locals[x].clone());
	}

	pub fn execute_iconst_x(&mut self, x: i64, frame: &mut Frame) {
		frame
			.operand_stack
			.push(JvmTypeValue::Primitive(JvmPrimitiveTypeValue::new(
				JvmPrimitiveType::Integer,
				x,
			)));
	}

	pub fn execute_invokestatic(
		&mut self,
		bytes: &[u8],
		source_frame: &mut Frame,
	) -> Option<OpcodeResult> {
		let class = source_frame.class().unwrap();
		let constant_pool = class.get_constant_pool_ref();
		let method_index = (((bytes[1] as u16) << 8) | (bytes[2] as u16)) as usize;

		match constant_pool.get_constant_ref(method_index) {
			Constant::Methodref(_, class_index, method_index) => {
				if let Constant::Class(_, class_name_index) =
					constant_pool.get_constant_ref(*class_index as usize)
				{
					if let Constant::NameAndType(_, method_name_index, _) =
						constant_pool.get_constant_ref(*method_index as usize)
					{
						if let Constant::Utf8(_, _, _, class_name) =
							constant_pool.get_constant_ref(*class_name_index as usize)
						{
							if let Constant::Utf8(_, _, _, method_name) =
								constant_pool.get_constant_ref(*method_name_index as usize)
							{
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
									assert!(false, "We do need to load the class!");
									/*
									 * TODO
									 */
								}

								if let Some(invoked_class) =
									self.methodarea.get_class_rc(&class_name)
								{
									if let Some(method) =
										invoked_class.get_methods_ref().get_by_name(
											method_name,
											invoked_class.get_constant_pool_ref(),
										) {
										if self.debug {
											println!("method: {}", method);
										}

										let mut invoked_frame = Frame::new();
										invoked_frame.class = Some(Rc::clone(&invoked_class));

										/*
										 * Move the parameters from the source stack to the
										 * invoked stack.
										 */
										let parameter_count = method.get_parameter_count(
											invoked_class.get_constant_pool_ref(),
										);
										for i in 0..parameter_count {
											if let Some(parameter) =
												source_frame.operand_stack.pop()
											{
												invoked_frame.locals.push(parameter);
											} else {
												assert!(false,
												  "Not enough parameters on the stack to call {}.{}.",
												  class_name,
												  method_name);
											}
										}

										if self.debug {
											println!("Parameter count: {}", parameter_count);
											println!("invoked_frame: {}", invoked_frame);
										}

										if let Some(v) = self.execute_method(&method, invoked_frame)
										{
											println!("Returning from a method: {}!", v);
											return Some(OpcodeResult::Value(v));
										}
									}
								} else {
									assert!(
										false,
										"Error: Could not execute method {}.{}",
										class_name, method_name
									);
								}
							}
						}
					}
				}
			}
			_ => (),
		};
		None
	}
}
