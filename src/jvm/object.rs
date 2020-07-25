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
use jvm::class::Class;
use jvm::constant::Constant;
use jvm::constantpool::ConstantPool;
use jvm::debug::{Debug, DebugLevel};
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use jvm::jvmthread::JvmThread;
use jvm::methodarea::MethodArea;
use jvm::typevalues::JvmPrimitiveType;
use jvm::typevalues::JvmReferenceTargetType;
use jvm::typevalues::JvmReferenceType;
use jvm::typevalues::JvmType;
use jvm::typevalues::JvmValue;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct JvmObject {
	spr: Option<Rc<JvmObject>>,
	class: Rc<Class>,
	fields: HashMap<String, Rc<JvmValue>>,
	debug_level: DebugLevel,
}

pub fn create_static_string_object(
	value: String,
	thread: &JvmThread,
	methodarea: Arc<Mutex<MethodArea>>,
) -> Option<JvmObject> {
	if let Ok(mut methodarea) = methodarea.lock() {
		let string_class_name = format!("java/lang/String");
		if let Some(string_class) = methodarea.get_class_rc(&string_class_name) {
			let mut string_object = JvmObject::new(Rc::clone(&string_class), thread.debug_level());
			println!(
				"get_field: value: {}",
				string_object.get_field(&"value".to_string()).unwrap()
			);
			return Some(string_object);
		}
	}

	None
}

impl JvmObject {
	pub fn new(class: Rc<Class>, debug_level: DebugLevel) -> Self {
		JvmObject {
			spr: None,
			class: class,
			fields: HashMap::<String, Rc<JvmValue>>::new(),
			debug_level,
		}
	}

	pub fn get_class(&self) -> Rc<Class> {
		Rc::clone(&self.class)
	}

	pub fn set_field(&mut self, field_name: &String, value: Rc<JvmValue>) {
		self.fields.insert(field_name.clone(), value);
	}

	pub fn get_field(&mut self, field_name: &String) -> Option<Rc<JvmValue>> {
		if let Some(field_value) = self.fields.get(field_name) {
			Some(Rc::clone(field_value))
		} else {
			None
		}
	}

	pub fn hierarchy(&self) -> String {
		let mut result = self.class.get_class_name().unwrap();
		if let Some(spr) = &self.spr {
			result = format!("{}, {}", result, spr.hierarchy());
		}
		result
	}

	pub fn is_type_of(&self, r#type: &String) -> bool {
		if self.class.get_class_name().unwrap() == *r#type {
			true
		} else if let Some(spr) = &self.spr {
			spr.is_type_of(r#type)
		} else {
			false
		}
	}

	pub fn instantiate(
		&mut self,
		initializing_thread: &mut JvmThread,
		methodarea: Arc<Mutex<MethodArea>>,
	) -> bool {
		let fields = self.class.get_fields_ref();
		let constantpool = self.class.get_constant_pool_ref();

		for i in 0..fields.fields_count() {
			let field = fields.get(i as usize);
			/*
			 * Get the field type.
			 */
			let r#type: JvmType =
				match constantpool.get_constant_ref(field.descriptor_index as usize) {
					Constant::Utf8(_, _, _, d) => {
						let descriptor = d.as_bytes();
						JvmType::from(descriptor)
					}
					_ => {
						FatalError::new(FatalErrorType::InvalidConstantReference(
							self.class.get_class_name().unwrap(),
							"Utf8".to_string(),
							field.descriptor_index,
						))
						.call();
						JvmType::Primitive(JvmPrimitiveType::Void)
					}
				};

			/*
			 * Get the field access modifiers.
			 */
			let access_flags = field.access_flags;

			/*
			 * Get the default field value.
			 */
			let value = match r#type {
				JvmType::Primitive(primitive) => JvmValue::Primitive(primitive, 0, access_flags),
				JvmType::Reference(reference) => {
					assert!(false, "TODO: Handle fields that are reference types.");
					JvmValue::Primitive(JvmPrimitiveType::Void, 0, access_flags)
				}
			};

			/*
			 * Get the field name.
			 */
			let name = match constantpool.get_constant_ref(field.name_index as usize) {
				Constant::Utf8(_, _, _, name) => name.clone(),
				_ => {
					FatalError::new(FatalErrorType::InvalidConstantReference(
						self.class.get_class_name().unwrap(),
						"Utf8".to_string(),
						field.name_index,
					))
					.call();
					"".to_string()
				}
			};

			/*
			 * Now, put it in our field table.
			 */
			self.fields.insert(name, Rc::new(value));
		}

		/*
		 * TODO: Handle superclass instantiation!
		 */
		if let Some(superclass_name) = self.class.resolve_superclass() {
			// We have a superclass and we know it's name.
			Debug(
				format!("Make a new superclass of {}.", superclass_name),
				&self.debug_level,
				DebugLevel::Info,
			);

			/*
			 * TODO: Let this go all the way to object!
			 */

			if superclass_name == format!("java/lang/Object") {
				Debug(
					format!("Not making the base class java/lang/Object"),
					&self.debug_level,
					DebugLevel::Info,
				);
				return true;
			}

			let mut instantiated_class: Option<Rc<Class>> = None;
			if let Ok(mut methodarea) = methodarea.lock() {
				(*methodarea).maybe_load_class(&superclass_name);
				instantiated_class = (*methodarea).get_class_rc(&superclass_name);
			} else {
				FatalError::new(FatalErrorType::CouldNotLock(
					"Method Area.".to_string(),
					"instantiate".to_string(),
				))
				.call();
			}
			if let Some(instantiated_class) = instantiated_class {
				initializing_thread.maybe_initialize_class(&instantiated_class);

				let mut object = JvmObject::new(instantiated_class, self.debug_level.clone());

				object.instantiate(initializing_thread, Arc::clone(&methodarea));
				self.spr = Some(Rc::new(object));
				Debug(
					format!("Made a new superclass of {}.", superclass_name),
					&self.debug_level,
					DebugLevel::Info,
				);
			} else {
				FatalError::new(FatalErrorType::ClassNotLoaded(superclass_name.to_string())).call();
			}
		}
		true
	}
}

impl fmt::Display for JvmObject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Object of type {}", self.class.get_class_name().unwrap())
	}
}
