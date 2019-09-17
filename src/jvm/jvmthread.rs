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

use arm_and_handler::arm;
use arm_and_handler::arm_and_handler;
use arm_and_handler::handler;

use enum_primitive::FromPrimitive;
use jvm::class::Class;
use jvm::class::ClassAccessFlags;
use jvm::constant::Constant;
use jvm::constantpool::ConstantPool;
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
use jvm::object::JvmObject;
use jvm::opcodes::OperandCode;
use jvm::typevalues::JvmPrimitiveType;
use jvm::typevalues::JvmReferenceType;
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

pub fn move_parameters_to_locals(
	method: &Method,
	invoking_frame: &mut Frame,
	invoked_frame: &mut Frame,
) -> bool {
	for i in 0..method.parameter_count {
		if let Some(parameter) = invoking_frame.operand_stack.pop() {
			invoked_frame.locals.insert(0, parameter);
		} else {
			return false;
		}
	}
	true
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
									if let Some(class) =
										(*methodarea).load_class_from_file(&class_filename)
									{
										Debug(
											format!("Loaded class {}.\n", class),
											&self.debug_level,
											DebugLevel::Info,
										);
									} else {
										/*
										 * TODO: Warn that we couldn't load this class for some reason.
										 */
									}
								}
							}
						}
					}
				}
			}
		}

		let mut main_class: Option<Rc<Class>> = None;
		if let Ok(methodarea) = self.methodarea.lock() {
			main_class = (*methodarea).get_class_rc(class_name);
		}
		if let Some(main_class) = main_class {
			Debug(
				format!("Loaded class {}.\n", main_class),
				&self.debug_level,
				DebugLevel::Info,
			);

			/*
			 * Per the spec, it is required that we initialize the main
			 * class before calling the main method inside that class.
			 */
			self.execute_clinit(&main_class, class_name);

			if let Some(main_method) = main_class
				.get_method_rc_by_name_and_type(method_name, &"([Ljava/lang/String;)V".to_string())
			{
				if main_method.access_flags
					!= ((MethodAccessFlags::Public as u16) | (MethodAccessFlags::Static as u16))
				{
					FatalError::new(FatalErrorType::MainMethodNotPublicStatic).call();
				}
				if JvmType::Primitive(JvmPrimitiveType::Void) != main_method.return_type {
					FatalError::new(FatalErrorType::MainMethodNotVoid).call();
				}
				let mut frame = Frame::new();
				frame.class = Some(Rc::clone(&main_class));
				/*
				 * Load up the frame's stack with the CLI arguments.
				 */
				frame
					.operand_stack
					.push(JvmValue::Primitive(JvmPrimitiveType::Boolean, 0, 0));

				Debug(
					format!("Frame: {}", frame),
					&self.debug_level,
					DebugLevel::Info,
				);

				if let Some(v) = self.execute_method(&main_method, frame) {
					if JvmValue::Primitive(JvmPrimitiveType::Void, 0, 0) != v {
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
		/*
		 * The locals are only going to have enough size for the parameters.
		 * Resize as appropriate.
		 */
		let class = frame.class().unwrap();

		Debug(
			format!(
				"Resizing local parameter array from {} to {}\n",
				frame.locals.len(),
				method.max_locals
			),
			&self.debug_level,
			DebugLevel::Info,
		);

		frame.locals.resize(
			method.max_locals,
			JvmValue::Primitive(JvmPrimitiveType::Void, 0, 0),
		);

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
			Some(OperandCode::Bipush) => {
				Debug(format!("bipush"), &self.debug_level, DebugLevel::Info);
				frame.operand_stack.push(JvmValue::Primitive(
					JvmPrimitiveType::Integer,
					bytes[1] as u64,
					0,
				));
				pc_incr = 2;
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
			Some(OperandCode::Aload_0) => {
				Debug(format!("aload_0"), &self.debug_level, DebugLevel::Info);
				self.execute_aload_x(0, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Aload_1) => {
				Debug(format!("aload_1"), &self.debug_level, DebugLevel::Info);
				self.execute_aload_x(1, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Aload_2) => {
				Debug(format!("aload_2"), &self.debug_level, DebugLevel::Info);
				self.execute_aload_x(2, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Aload_3) => {
				Debug(format!("aload_3"), &self.debug_level, DebugLevel::Info);
				self.execute_aload_x(3, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Istore_0) => {
				Debug(format!("istore_0"), &self.debug_level, DebugLevel::Info);
				self.execute_istore_x(0, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Istore_1) => {
				Debug(format!("istore_1"), &self.debug_level, DebugLevel::Info);
				self.execute_istore_x(1, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Istore_2) => {
				Debug(format!("istore_2"), &self.debug_level, DebugLevel::Info);
				self.execute_istore_x(2, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Istore_3) => {
				Debug(format!("istore_3"), &self.debug_level, DebugLevel::Info);
				self.execute_istore_x(3, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Astore_0) => {
				Debug(format!("astore_0"), &self.debug_level, DebugLevel::Info);
				self.execute_astore_x(0, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Astore_1) => {
				Debug(format!("astore_1"), &self.debug_level, DebugLevel::Info);
				self.execute_astore_x(1, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Astore_2) => {
				Debug(format!("astore_2"), &self.debug_level, DebugLevel::Info);
				self.execute_astore_x(2, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Astore_3) => {
				Debug(format!("astore_3"), &self.debug_level, DebugLevel::Info);
				self.execute_astore_x(3, frame);
				pc_incr = 1;
			}
			Some(OperandCode::Pop) => {
				Debug(format!("pop"), &self.debug_level, DebugLevel::Info);
				frame.operand_stack.pop();
				pc_incr = 1;
			}
			Some(OperandCode::Dup) => {
				Debug(format!("dup"), &self.debug_level, DebugLevel::Info);
				/*
				 * TODO: The type on the stack must be a "category 1
				 * computational type."
				 */
				if let Some(top) = frame.operand_stack.last() {
					frame.operand_stack.push(top.clone());
				}
				pc_incr = 1;
			}
			Some(OperandCode::Iadd) => {
				Debug(format!("iadd"), &self.debug_level, DebugLevel::Info);
				self.execute_iadd(frame);
				pc_incr = 1;
			}
			Some(OperandCode::Imul) => {
				Debug(format!("imul"), &self.debug_level, DebugLevel::Info);
				self.execute_imul(frame);
				pc_incr = 1;
			}
			Some(OperandCode::Ireturn) => {
				Debug(format!("ireturn"), &self.debug_level, DebugLevel::Info);
				return OpcodeResult::Value(frame.operand_stack.pop().unwrap());
			}
			Some(OperandCode::r#Return) => {
				Debug(format!("return"), &self.debug_level, DebugLevel::Info);
				return OpcodeResult::Value(JvmValue::Primitive(JvmPrimitiveType::Void, 0, 0));
			}
			Some(OperandCode::Invokevirtual) => {
				Debug(
					format!("invokevirtual"),
					&self.debug_level,
					DebugLevel::Info,
				);
				/*
				 * Start by assuming failure.
				 */
				pc_incr = 0;

				let invokevirtual_result = self.execute_invokevirtual(bytes, frame);
				pc_incr = self.handle_invoke_result(invokevirtual_result, frame, 3);
			}
			Some(OperandCode::Invokespecial) => {
				Debug(
					format!("invokespecial"),
					&self.debug_level,
					DebugLevel::Info,
				);
				/*
				 * Start by assuming failure.
				 */
				pc_incr = 0;

				let invokespecial_result = self.execute_invokespecial(bytes, frame);
				pc_incr = self.handle_invoke_result(invokespecial_result, frame, 3);
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
			Some(OperandCode::New) => {
				Debug(format!("New"), &self.debug_level, DebugLevel::Info);
				if let Some(object) = self.execute_new(bytes, frame) {
					frame.operand_stack.push(object);
					Debug(
						format!("frame after new: {}", frame),
						&self.debug_level,
						DebugLevel::Info,
					);
				}
				pc_incr = 3;
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
				JvmValue::Primitive(t, _, _) => {
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
		if let Some(JvmValue::Primitive(JvmPrimitiveType::Integer, op1, _)) =
			frame.operand_stack.pop()
		{
			if let Some(JvmValue::Primitive(JvmPrimitiveType::Integer, op2, _)) =
				frame.operand_stack.pop()
			{
				frame.operand_stack.push(JvmValue::Primitive(
					JvmPrimitiveType::Integer,
					op1 + op2,
					0,
				));
			}
		}
	}
	fn execute_imul(&mut self, frame: &mut Frame) {
		if let Some(JvmValue::Primitive(JvmPrimitiveType::Integer, op1, _)) =
			frame.operand_stack.pop()
		{
			if let Some(JvmValue::Primitive(JvmPrimitiveType::Integer, op2, _)) =
				frame.operand_stack.pop()
			{
				frame.operand_stack.push(JvmValue::Primitive(
					JvmPrimitiveType::Integer,
					op1 * op2,
					0,
				));
			}
		}
	}

	fn execute_astore_x(&self, x: usize, frame: &mut Frame) {
		println!("Frame before store: {}\n", frame);
		if x < frame.locals.len() {
			if let Some(top) = frame.operand_stack.pop() {
				if let JvmValue::Reference(rt, reference, access) = top {
					frame.locals[x] = JvmValue::Reference(rt, reference, access);
				} else {
					assert!(false, "Wrong type.");
				}
			} else {
				assert!(false, "Not enough on the top of the stack.");
			}
		} else {
			assert!(
				false,
				"Not enough locals available: {}.",
				frame.locals.len()
			);
		}
	}

	fn execute_aload_x(&self, x: usize, frame: &mut Frame) {
		if x < frame.locals.len() {
			if let JvmValue::Reference(_, _, _) = frame.locals[x] {
				frame.operand_stack.push(frame.locals[x].clone());
			} else {
				FatalError::new(FatalErrorType::WrongType(
					format!("aload_{}", x),
					"Reference".to_string(),
				))
				.call();
			}
		} else {
			FatalError::new(FatalErrorType::NotEnough(
				format!("aload_{}", x),
				x,
				"locals".to_string(),
			))
			.call();
		}
	}

	fn execute_iload_x(&mut self, x: usize, frame: &mut Frame) {
		frame.operand_stack.push(frame.locals[x].clone());
	}

	fn execute_istore_x(&self, x: usize, frame: &mut Frame) {
		println!("Frame before store: {}\n", frame);
		if x < frame.locals.len() {
			if let Some(top) = frame.operand_stack.pop() {
				if let JvmValue::Primitive(pt, value, access) = top {
					frame.locals[x] = JvmValue::Primitive(pt, value, access);
				} else {
					assert!(false, "Wrong type.");
				}
			} else {
				assert!(false, "Not enough on the top of the stack.");
			}
		} else {
			assert!(
				false,
				"Not enough locals available: {}.",
				frame.locals.len()
			);
		}
	}

	fn execute_iconst_x(&mut self, x: i64, frame: &mut Frame) {
		frame
			.operand_stack
			.push(JvmValue::Primitive(JvmPrimitiveType::Integer, x as u64, 0));
	}

	fn execute_clinit(&mut self, class: &Rc<Class>, class_name: &String) {
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
		let mut loaded_class: Option<Arc<Mutex<LoadedClass>>> = None;
		if let Ok(methodarea) = self.methodarea.lock() {
			loaded_class = (*methodarea).get_loaded_class(class_name);
		} else {
			FatalError::new(FatalErrorType::CouldNotLock(
				"Method Area.".to_string(),
				"execute_clinit".to_string(),
			))
			.call();
		}

		if let Some(loaded_class) = loaded_class {
			if let Ok(mut loaded_class) = loaded_class.lock() {
				if (*loaded_class).is_initialized() {
					return;
				}

				(*loaded_class).initialize();

				let clinit: String = "<clinit>".into();

				/*
				 * We must invoke the clinit method, if one exists.
				 */
				if let Some(clinit_method) = class.get_methods_ref().get_by_name_and_type(
					&clinit,
					&"()V".to_string(),
					class.get_constant_pool_ref(),
				) {
					Debug(
						format!("clinit Method: {}", clinit_method),
						&self.debug_level,
						DebugLevel::Info,
					);

					let mut clinit_frame = Frame::new();
					clinit_frame.class = Some(Rc::clone(&class));

					Debug(
						format!("clinit Frame: {}", clinit_frame),
						&self.debug_level,
						DebugLevel::Info,
					);

					if let Some(v) = self.execute_method(&clinit_method, clinit_frame) {
						if JvmValue::Primitive(JvmPrimitiveType::Void, 0, 0) != v {
							FatalError::new(FatalErrorType::ClassInitMethodReturnedValue).call();
						}
					}
				}
			} else {
				FatalError::new(FatalErrorType::CouldNotLock(
					class_name.to_string(),
					"execute_clinit".to_string(),
				))
				.call();
			}
		}
	}

	fn execute_new(&mut self, bytes: &[u8], source_frame: &mut Frame) -> Option<JvmValue> {
		let class = source_frame.class().unwrap();
		let constant_pool = class.get_constant_pool_ref();
		let instantiated_class_index = (((bytes[1] as u16) << 8) | (bytes[2] as u16)) as usize;

		match constant_pool.get_constant_ref(instantiated_class_index) {
			Constant::Class(_, instantiated_class_name_index) => {
				match constant_pool.get_constant_ref(*instantiated_class_name_index as usize) {
					Constant::Utf8(_, _, _, instantiated_class_name) => {
						Debug(
							format!("Make a new {}.", instantiated_class_name),
							&self.debug_level,
							DebugLevel::Info,
						);
						let mut result: Option<JvmValue> = None;
						let mut instantiated_class: Option<Rc<Class>> = None;
						if let Ok(methodarea) = self.methodarea.lock() {
							instantiated_class =
								(*methodarea).get_class_rc(instantiated_class_name);
						} else {
							FatalError::new(FatalErrorType::CouldNotLock(
								"Method Area.".to_string(),
								"execute_new".to_string(),
							))
							.call();
						}
						if let Some(instantiated_class) = instantiated_class {
							self.execute_clinit(&instantiated_class, instantiated_class_name);
							let mut object = JvmObject::new(instantiated_class);
							Debug(format!("{}", object), &self.debug_level, DebugLevel::Info);
							object.instantiate();
							result = Some(JvmValue::Reference(
								JvmReferenceType::Class(instantiated_class_name.to_string()),
								Rc::new(object),
								0,
							));
						} else {
							FatalError::new(FatalErrorType::ClassNotLoaded(
								instantiated_class_name.to_string(),
							))
							.call();
						}
						result
					}
					_ => {
						FatalError::new(FatalErrorType::InvalidConstantReference(
							class.get_class_name().unwrap(),
							"Utf8".to_string(),
							*instantiated_class_name_index,
						))
						.call();
						None
					}
				}
			}
			_ => {
				FatalError::new(FatalErrorType::InvalidConstantReference(
					class.get_class_name().unwrap(),
					"Classref".to_string(),
					instantiated_class_index as u16,
				))
				.call();
				None
			}
		}
	}

	fn execute_invokevirtual(
		&mut self,
		bytes: &[u8],
		source_frame: &mut Frame,
	) -> Option<OpcodeResult> {
		let class = source_frame.class().unwrap();
		let constant_pool = class.get_constant_pool_ref();
		let method_index = (((bytes[1] as u16) << 8) | (bytes[2] as u16)) as usize;

		if let Some((method_name, method_type, invoked_class_name)) =
			class.resolve_method_ref(method_index)
		{
			let mut invoked_class: Option<Rc<Class>> = None;
			let mut resolved_method: Option<Rc<Method>> = None;
			let mut invoked_frame: Frame = Frame::new();

			Debug(
				format!("Invoke Virtual: {}.{}", invoked_class_name, method_name),
				&self.debug_level,
				DebugLevel::Info,
			);

			if let Ok(mut methodarea) = self.methodarea.lock() {
				invoked_class = (*methodarea).get_class_rc(&invoked_class_name);
				resolved_method = if let Some(invoked_class) = &invoked_class {
					(*methodarea).resolve_method(&class, &invoked_class, &method_name, &method_type)
				} else {
					None
				};
			}

			if let (Some(invoked_class), Some(resolved_method)) = (invoked_class, resolved_method) {
				let mut object_class_name: Option<String> = None;

				/*
				 * Let's build a frame!
				 */
				if !move_parameters_to_locals(&resolved_method, source_frame, &mut invoked_frame) {
					FatalError::new(FatalErrorType::NotEnough(
						"invokevirtual".to_string(),
						resolved_method.parameter_count,
						"stack operands".to_string(),
					))
					.call();
				}
				/*
				 * The first value on the stack is an object reference. It becomes
				 * the 0th local variable to the special method.
				 */
				if let Some(top) = source_frame.operand_stack.pop() {
					if let JvmValue::Reference(JvmReferenceType::Class(ocn), _, _) = &top {
						object_class_name = Some(ocn.to_string());
						invoked_frame.locals.insert(0, top);
					} else {
						/*
						 * TODO: This is a fatal error: The first value on
						 * the stack at this point must be a reference.
						 */
					}
				}

				/*
				 * Check to see if the resolved method is private. If so, it's the one
				 * that we invoke.
				 */
				if ((MethodAccessFlags::Protected as u16) & resolved_method.access_flags) != 0 {
					invoked_frame.class = Some(invoked_class);
					if let Some(v) = self.execute_method(&resolved_method, invoked_frame) {
						Debug(
							format!("Returning from a method: {}", resolved_method.clone()),
							&self.debug_level,
							DebugLevel::Info,
						);
						return Some(OpcodeResult::Value(v));
					} else {
						FatalError::new(FatalErrorType::MethodExecutionFailed(method_name)).call();
					}
				} else if let Some(object_class_name) = object_class_name {
					let mut selected_class_method: Option<(Rc<Class>, Rc<Method>)> = None;
					let mut object_class: Option<Rc<Class>> = None;

					if let Ok(mut methodarea) = self.methodarea.lock() {
						object_class = (*methodarea).get_class_rc(&object_class_name);
						selected_class_method = if let Some(object_class) = &object_class {
							(*methodarea).select_method(&object_class, &method_name, &method_type)
						} else {
							FatalError::new(FatalErrorType::MethodSelectionFailed).call();
							None
						};
					}

					if let Some((selected_class, selected_method)) = selected_class_method {
						invoked_frame.class = Some(selected_class);
						if let Some(v) = self.execute_method(&selected_method, invoked_frame) {
							Debug(
								format!("Returning from a method: {}", resolved_method.clone()),
								&self.debug_level,
								DebugLevel::Info,
							);
							return Some(OpcodeResult::Value(v));
						} else {
							FatalError::new(FatalErrorType::MethodExecutionFailed(method_name))
								.call();
							assert!(false);
						}
					}
				} else {
					FatalError::new(FatalErrorType::MethodExecutionFailed(method_name)).call();
				}
			}
			/*
			 * TODO: This is a fatal error, but I'm not sure exactly
			 * how to qualify it.
			 */
		}
		FatalError::new(FatalErrorType::MethodResolutionFailed).call();
		None
	}

	fn execute_invokespecial(
		&mut self,
		bytes: &[u8],
		source_frame: &mut Frame,
	) -> Option<OpcodeResult> {
		let class = source_frame.class().unwrap();
		let constant_pool = class.get_constant_pool_ref();
		let method_index = (((bytes[1] as u16) << 8) | (bytes[2] as u16)) as usize;

		if let Some((method_name, method_type, invoked_class_name)) =
			class.resolve_method_ref(method_index)
		{
			let mut invoked_class: Option<Rc<Class>> = None;
			let mut resolved_method: Option<Rc<Method>> = None;

			Debug(
				format!("Invoke Special: {}.{}", invoked_class_name, method_name),
				&self.debug_level,
				DebugLevel::Info,
			);

			if let Ok(mut methodarea) = self.methodarea.lock() {
				invoked_class = (*methodarea).get_class_rc(&invoked_class_name);
				resolved_method = if let Some(invoked_class) = &invoked_class {
					(*methodarea).resolve_method(&class, &invoked_class, &method_name, &method_type)
				} else {
					None
				}
			}

			if let (Some(invoked_class), Some(resolved_method)) = (invoked_class, resolved_method) {
				if ((MethodAccessFlags::Protected as u16) & resolved_method.access_flags) != 0 {
					assert!(false, "TODO: Finally, if the resolved method is protected (§4.6), and it is a member of a superclass of the current class, and the method is not declared in the same run-time package (§5.3) as the current class, then the class of objectref must be either the current class or a subclass of the current class.");
				}
				/* TODO:
					Next, the resolved method is selected for invocation unless all of the following conditions are true:

				   The ACC_SUPER flag (Table 4.1) is set for the current class.

					 The resolved method is not an instance initialization method (§2.9).
				   ...
				*/

				if ((ClassAccessFlags::Super as u16) & class.access_flags) != 0
					&& method_name != "<init>"
				{
					/*
						...

						The class of the resolved method is a superclass of the current class.

						If the above conditions are true, the actual method to be invoked is selected by the following lookup procedure. Let C be the direct superclass of the current class:

						If C contains a declaration for an instance method with the same name and descriptor as the resolved method, then this method will be invoked. The lookup procedure terminates.

						Otherwise, if C has a superclass, this same lookup procedure is performed recursively using the direct superclass of C. The method to be invoked is the result of the recursive invocation of this lookup procedure.

						Otherwise, an AbstractMethodError is raised.
					*/
					assert!(false);
				}

				let mut invoked_frame = Frame::new();
				invoked_frame.class = Some(Rc::clone(&invoked_class));

				/*
				 * The other parameters are on the stack, too. Move the parameters
				 * from the source stack to the invoked stack.
				 */
				if !move_parameters_to_locals(&resolved_method, source_frame, &mut invoked_frame) {
					FatalError::new(FatalErrorType::NotEnough(
						"invokespecial".to_string(),
						resolved_method.parameter_count,
						"stack operands".to_string(),
					))
					.call();
				}
				/*
				 * The first value on the stack is an object reference. It becomes
				 * the 0th local variable to the special method.
				 */
				if let Some(top) = source_frame.operand_stack.pop() {
					if let JvmValue::Reference(_, _, _) = top {
						invoked_frame.locals.insert(0, top);
					} else {
						/*
						 * TODO: This is a fatal error: The first value on
						 * the stack at this point must be a reference.
						 */
					}
				}

				Debug(
					format!("Parameter count: {}", resolved_method.parameter_count),
					&self.debug_level,
					DebugLevel::Info,
				);
				Debug(
					format!("invoked_frame: {}", invoked_frame),
					&self.debug_level,
					DebugLevel::Info,
				);

				if let Some(v) = self.execute_method(&resolved_method, invoked_frame) {
					Debug(
						format!("Returning from a method: {}", resolved_method.clone()),
						&self.debug_level,
						DebugLevel::Info,
					);
					return Some(OpcodeResult::Value(v));
				} else {
					FatalError::new(FatalErrorType::MethodExecutionFailed(method_name)).call();
				}
			}
			FatalError::new(FatalErrorType::ClassNotFound(invoked_class_name.clone())).call()
		}
		FatalError::new(FatalErrorType::MethodResolutionFailed).call();
		None
	}

	fn execute_invokestatic(
		&mut self,
		bytes: &[u8],
		source_frame: &mut Frame,
	) -> Option<OpcodeResult> {
		let class = source_frame.class().unwrap();
		let constant_pool = class.get_constant_pool_ref();
		let method_index = (((bytes[1] as u16) << 8) | (bytes[2] as u16)) as usize;

		if let Some((method_name, method_type, invoked_class_name)) =
			class.resolve_method_ref(method_index)
		{
			Debug(
				format!("Invoke Static: {}.{}", invoked_class_name, method_name),
				&self.debug_level,
				DebugLevel::Info,
			);
			let mut invoked_class: Option<Rc<Class>> = None;
			if let Ok(mut methodarea) = self.methodarea.lock() {
				invoked_class = (*methodarea).get_class_rc(&invoked_class_name);
			}
			if let Some(invoked_class) = invoked_class {
				/*
				 * TODO: We need to follow the method resolution process here. See 5.4.3.3.
				 */
				if let Some(method) = invoked_class.get_methods_ref().get_by_name_and_type(
					&method_name,
					&method_type,
					invoked_class.get_constant_pool_ref(),
				) {
					Debug(
						format!("method: {}", method),
						&self.debug_level,
						DebugLevel::Info,
					);
					/*
					 * This is an operation that requires the target class
					 * be initialized.
					 */
					self.execute_clinit(&invoked_class, &invoked_class_name);

					let mut invoked_frame = Frame::new();
					invoked_frame.class = Some(Rc::clone(&invoked_class));

					/*
					 * Move the parameters from the source stack to the
					 * invoked stack.
					 */
					let parameter_count = method.parameter_count;
					for i in 0..parameter_count {
						if let Some(parameter) = source_frame.operand_stack.pop() {
							invoked_frame.locals.insert(0, parameter);
						} else {
							FatalError::new(FatalErrorType::NotEnough(
								"invokestatic".to_string(),
								i,
								"stack operands".to_string(),
							))
							.call();
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

					if let Some(v) = self.execute_method(&method, invoked_frame) {
						Debug(
							format!("Returning from a method{}", method.clone()),
							&self.debug_level,
							DebugLevel::Info,
						);
						return Some(OpcodeResult::Value(v));
					}
				}
			} else {
				FatalError::new(FatalErrorType::ClassNotFound(invoked_class_name.clone())).call()
			}
		}
		None
	}
}
