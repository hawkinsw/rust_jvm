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
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use jvm::typevalues::JvmPrimitiveType;
use jvm::typevalues::JvmReferenceType;
use jvm::typevalues::JvmType;
use jvm::typevalues::JvmValue;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct JvmObject {
	spr: Option<Rc<JvmObject>>,
	class: Rc<Class>,
	fields: HashMap<String, Rc<JvmValue>>,
}

impl JvmObject {
	pub fn new(class: Rc<Class>) -> Self {
		JvmObject {
			spr: None,
			class: class,
			fields: HashMap::<String, Rc<JvmValue>>::new(),
		}
	}

	pub fn get_class(&self) -> Rc<Class> {
		Rc::clone(&self.class)
	}

	pub fn set_field(&mut self, field_name: &String, value: Rc<JvmValue>) {
		self.fields.insert(field_name.clone(), value);
	}

	pub fn get_field(&mut self, field_name: &String, value: Rc<JvmValue>) -> Option<Rc<JvmValue>> {
		if let Some(field_value) = self.fields.get(field_name) {
			Some(Rc::clone(field_value))
		} else {
			None
		}
	}

	pub fn instantiate(&mut self) -> bool {
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
		true
	}
}

impl fmt::Display for JvmObject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Object of type {}", self.class.get_class_name().unwrap())
	}
}
