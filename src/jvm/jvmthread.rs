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
use jvm::class::Class;
use jvm::constant::Constant;
use jvm::debug::Debug;
use jvm::debug::DebugLevel;
use jvm::environment::Environment;
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use jvm::frame::Frame;
use jvm::method::Method;
use jvm::method::MethodAccessFlags;
use jvm::methodarea::LoadedClass;
use jvm::methodarea::MethodArea;
use jvm::opcodes::OperandCode;
use jvm::typevalues::JvmPrimitiveType;
use jvm::typevalues::JvmType;
use jvm::typevalues::JvmValue;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::LockResult;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct JvmThread {
	debug_level: DebugLevel,
	methodarea: Arc<Mutex<MethodArea>>,
	pc: usize,
}

enum OpcodeResult {
	Incr(usize),
	Value(JvmValue),
}

impl JvmThread {
	pub fn new(debug_level: DebugLevel, methodarea: Arc<Mutex<MethodArea>>) -> Self {
		JvmThread {
			debug_level: debug_level,
			methodarea: methodarea,
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
								Debug(
									format!("Loading class file {}", class_filename),
									&self.debug_level,
									DebugLevel::Info,
								);
								if let Ok(mut methodarea) = self.methodarea.lock() {
									(*methodarea).load_class_from_file(&class_filename);
								}
							}
						}
					}
				}
			}
		}

		let mut class_or: Option<Rc<Class>> = None;
		if let Ok(methodarea) = self.methodarea.lock() {
			class_or = (*methodarea).get_class_rc(class_name);
		}
		if let Some(class) = class_or {
			Debug(
				format!("Loaded class {}.\n", class),
				&self.debug_level,
				DebugLevel::Info,
			);
			if let Some(method) = class.get_method_ref_by_name(method_name) {
				if method.access_flags
					!= ((MethodAccessFlags::Public as u16) | (MethodAccessFlags::Static as u16))
				{
					FatalError::new(FatalErrorType::MainMethodNotPublicStatic).call();
				}
				if JvmType::Primitive(JvmPrimitiveType::Void)
					!= method.get_return_type(class.get_constant_pool_ref())
				{
					println!("Main method is not void.");
					FatalError::new(FatalErrorType::MainMethodNotVoid).call();
				}
				let mut frame = Frame::new();
				frame.class = Some(Rc::clone(&class));
				/*
				 * Load up the frame's stack with the CLI arguments.
				 */
				frame
					.operand_stack
					.push(JvmValue::Primitive(JvmPrimitiveType::Boolean, 0));

				Debug(
					format!("Frame: {}", frame),
					&self.debug_level,
					DebugLevel::Info,
				);

				if let Some(v) = self.execute_method(method, frame) {
					if JvmValue::Primitive(JvmPrimitiveType::Void, 0) != v {
						FatalError::new(FatalErrorType::VoidMethodReturnedValue).call();
					}
				}
				return true;
			} else {
				FatalError::new(FatalErrorType::MethodNotFound(
					method_name.clone(),
					class_name.clone(),
				))
				.call()
			}
		} else {
			FatalError::new(FatalErrorType::ClassNotFound(class_name.clone())).call()
		}
		false
	}

	fn execute_method(&mut self, method: &Method, mut frame: Frame) -> Option<JvmValue> {
		let class = frame.class().unwrap();
		if let Some(code) = method.get_code(class.get_constant_pool_ref()) {
			let mut pc = 0;
			while {
				let mut pc_incr = 0;
				Debug(
					format!("Doing next opcode\n"),
					&self.debug_level,
					DebugLevel::Info,
				);
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
		Debug(
			format!("code: 0x{:X}\n", opcode),
			&self.debug_level,
			DebugLevel::Info,
		);
		match OperandCode::from_u8(opcode) {
			Some(OperandCode::Iconst_m1) => {
				Debug(format!("iconst_m1"), &self.debug_level, DebugLevel::Info);
				self.execute_iconst_x(-1, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iconst_0) => {
				Debug(format!("iconst_0"), &self.debug_level, DebugLevel::Info);
				self.execute_iconst_x(0, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iconst_1) => {
				Debug(format!("iconst_1"), &self.debug_level, DebugLevel::Info);
				self.execute_iconst_x(1, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iconst_2) => {
				Debug(format!("iconst_2"), &self.debug_level, DebugLevel::Info);
				self.execute_iconst_x(2, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iconst_3) => {
				Debug(format!("iconst_3"), &self.debug_level, DebugLevel::Info);
				self.execute_iconst_x(3, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iconst_4) => {
				Debug(format!("iconst_4"), &self.debug_level, DebugLevel::Info);
				self.execute_iconst_x(4, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iconst_5) => {
				Debug(format!("iconst_5"), &self.debug_level, DebugLevel::Info);
				self.execute_iconst_x(5, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iload_0) => {
				Debug(format!("iload_0"), &self.debug_level, DebugLevel::Info);
				self.execute_iload_x(0, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iload_1) => {
				Debug(format!("iload_1"), &self.debug_level, DebugLevel::Info);
				self.execute_iload_x(1, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iload_2) => {
				Debug(format!("iload_2"), &self.debug_level, DebugLevel::Info);
				self.execute_iload_x(2, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Iload_3) => {
				Debug(format!("iload_3"), &self.debug_level, DebugLevel::Info);
				self.execute_iload_x(3, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Ireturn) => {
				Debug(format!("ireturn"), &self.debug_level, DebugLevel::Info);
				return OpcodeResult::Value(frame.operand_stack.pop().unwrap());
			}
			Some(OperandCode::r#Return) => {
				Debug(format!("return"), &self.debug_level, DebugLevel::Info);
				return OpcodeResult::Value(JvmValue::Primitive(JvmPrimitiveType::Void, 0));
			}
			Some(OperandCode::Invokestatic) => {
				Debug(format!("invokestatic"), &self.debug_level, DebugLevel::Info);
				/*
				 * Start by assuming failure.
				 */
				pc_incr = 0;

				let invokestatic_result = self.execute_invokestatic(bytes, frame);
				pc_incr = self.handle_invoke_result(invokestatic_result, frame, 3);
			}
			Some(OperandCode::Pop) => {
				Debug(format!("pop"), &self.debug_level, DebugLevel::Info);
				frame.operand_stack.pop();
				pc_incr = 1;
			}
			Some(OperandCode::Iadd) => {
				Debug(format!("iadd"), &self.debug_level, DebugLevel::Info);
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

	fn handle_invoke_result(
		&self,
		result: Option<OpcodeResult>,
		frame: &mut Frame,
		step: usize,
	) -> usize {
		if let Some(OpcodeResult::Value(tv)) = result {
			/*
			 * Push the result of the invocation onto
			 * the operand stack. Do not push anything
			 * on to the stack if the return is Void.
			 */
			match tv.clone() {
				/*
				 * The JvmTypeValue::Primitive with tipe == JvmPrimitiveType::Void
				 * is a sentinel value that indicates a return from a Void function.
				 */
				JvmValue::Primitive(t, v) => {
					if t == JvmPrimitiveType::Void {
						Debug(
							format!("Not pushing a void onto the caller's stack."),
							&self.debug_level,
							DebugLevel::Info,
						);
					} else {
						/*
						 * Any JvmTypeValue::Primitive other than a JvmPrimitive::Void
						 * gets pushed on to the stack.
						 */
						frame.operand_stack.push(tv);
					}
				}
				/*
				 * A non-JvmTypeValue::Primitive value always gets pushed
				 * on to the stack.
				 */
				_ => {
					frame.operand_stack.push(tv);
				}
			}
			return step;
		}
		return 0;
	}

	fn execute_iadd(&mut self, frame: &mut Frame) {
		if let Some(JvmValue::Primitive(JvmPrimitiveType::Integer, op1)) = frame.operand_stack.pop()
		{
			if let Some(JvmValue::Primitive(JvmPrimitiveType::Integer, op2)) =
				frame.operand_stack.pop()
			{
				frame
					.operand_stack
					.push(JvmValue::Primitive(JvmPrimitiveType::Integer, op1 + op2));
			}
		}
	}

	fn execute_iload_x(&mut self, x: usize, frame: &mut Frame) {
		frame.operand_stack.push(frame.locals[x].clone());
	}

	fn execute_iconst_x(&mut self, x: i64, frame: &mut Frame) {
		frame
			.operand_stack
			.push(JvmValue::Primitive(JvmPrimitiveType::Integer, x as u64));
	}

	fn execute_invokestatic(
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
						if let Constant::Utf8(_, _, _, invoked_class_name) =
							constant_pool.get_constant_ref(*class_name_index as usize)
						{
							if let Constant::Utf8(_, _, _, method_name) =
								constant_pool.get_constant_ref(*method_name_index as usize)
							{
								Debug(
									format!(
										"Invoke Static: {}.{}",
										invoked_class_name, method_name
									),
									&self.debug_level,
									DebugLevel::Info,
								);
								/*
								 * Steps:
								 * 1. Get the class containing the method.
								 * 2. Get the method.
								 * 3. Create a frame.
								 * 4. Populate the frame.
								 * 5. Execute the method
								 */
								let mut invoked_class: Option<Rc<Class>> = None;
								if let Ok(mut methodarea) = self.methodarea.lock() {
									invoked_class = (*methodarea).get_class_rc(invoked_class_name);
								}
								if let Some(invoked_class) = invoked_class {
									if let Some(method) =
										invoked_class.get_methods_ref().get_by_name(
											method_name,
											invoked_class.get_constant_pool_ref(),
										) {
										Debug(
											format!("method: {}", method),
											&self.debug_level,
											DebugLevel::Info,
										);

										/*
										 * Check to see if we need to initialize the class
										 * before invoking a method on it.
										 *
										 * To do that, we have to lock the method area to make
										 * sure that the class doesn't go away from underneath
										 * us. Then, we need to get exclusive access to the
										 * class itself. Once we have that, we can initialize
										 * the class!
										 */
										let mut invoked_loaded_class: Option<
											Arc<Mutex<LoadedClass>>> = None;
										if let Ok(methodarea) = self.methodarea.lock() {
											invoked_loaded_class =
												(*methodarea).get_loaded_class(invoked_class_name);
										}

										if let Some(invoked_loaded_class) = invoked_loaded_class {
											if let Ok(mut invoked_loaded_class) =
												invoked_loaded_class.lock()
											{
												if !(*invoked_loaded_class).is_initialized() {
													(*invoked_loaded_class).initialize();
												}

												let clinit: String = "<clinit>".into();

												/*
												 * We must invoke the clinit method, if one exists.
												 */
												if let Some(clinit_method) =
													invoked_class.get_methods_ref().get_by_name(
														&clinit,
														invoked_class.get_constant_pool_ref(),
													) {
													Debug(
														format!("clinit Method: {}", clinit_method),
														&self.debug_level,
														DebugLevel::Info,
													);

													let mut clinit_frame = Frame::new();
													clinit_frame.class =
														Some(Rc::clone(&invoked_class));

													Debug(
														format!("clinit Frame: {}", clinit_frame),
														&self.debug_level,
														DebugLevel::Info,
													);

													if let Some(v) = self
														.execute_method(clinit_method, clinit_frame)
													{
														if JvmValue::Primitive(
															JvmPrimitiveType::Void,
															0,
														) != v
														{
															FatalError::new(
															FatalErrorType::ClassInitMethodReturnedValue,
														)
														.call();
														}
													}
												}
											}
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
												  invoked_class_name,
												  method_name);
											}
										}
										Debug(
											format!("Parameter count: {}", parameter_count),
											&self.debug_level,
											DebugLevel::Info,
										);
										Debug(
											format!("invoked_frame: {}", invoked_frame),
											&self.debug_level,
											DebugLevel::Info,
										);

										if let Some(v) = self.execute_method(&method, invoked_frame)
										{
											println!("Returning from a method: {}!", v);
											return Some(OpcodeResult::Value(v));
										}
									}
								} else {
									FatalError::new(FatalErrorType::ClassNotFound(
										invoked_class_name.clone(),
									))
									.call()
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
