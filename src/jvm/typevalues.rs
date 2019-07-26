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
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub enum JvmPrimitiveType {
	Boolean,
	Integer,
	Void,
	Invalid,
}

#[derive(Clone)]
pub enum JvmReferenceType {
	Array(Rc<JvmTypeValue>, u64),
	Class(String),
	Interface(String),
}

#[derive(Clone)]
pub struct JvmPrimitiveTypeValue {
	pub tipe: JvmPrimitiveType,
	pub value: i64,
}

impl JvmPrimitiveTypeValue {
	pub fn new(tipe: JvmPrimitiveType, value: i64) -> Self {
		Self {
			tipe: tipe,
			value: value,
		}
	}
}

#[derive(Clone)]
pub struct JvmReferenceTypeValue {
	pub tipe: JvmReferenceType,
	reference: u64,
}

impl JvmReferenceTypeValue {
	pub fn new_array(dimension: u64, component_type: JvmTypeValue, reference: u64) -> Self {
		JvmReferenceTypeValue {
			tipe: JvmReferenceType::Array(Rc::new(component_type), dimension),
			reference: reference,
		}
	}

	pub fn new_class(name: String, reference: u64) -> Self {
		JvmReferenceTypeValue {
			tipe: JvmReferenceType::Class(name),
			reference: reference,
		}
	}

	pub fn new_interface(name: String, reference: u64) -> Self {
		JvmReferenceTypeValue {
			tipe: JvmReferenceType::Interface(name),
			reference: reference,
		}
	}
}

impl fmt::Display for JvmPrimitiveTypeValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let type_name = match self.tipe {
			JvmPrimitiveType::Boolean => "Boolean",
			JvmPrimitiveType::Integer => "Integer",
			JvmPrimitiveType::Void => "Void",
			JvmPrimitiveType::Invalid => "Invalid",
		};
		write!(f, "{}: {}", type_name, self.value)
	}
}

impl fmt::Display for JvmTypeValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			JvmTypeValue::Primitive(p) => return write!(f, "{}", p),
			_ => (),
		};
		return write!(f, "Can't print references yet.");
	}
}

#[derive(Clone)]
pub enum JvmTypeValue {
	Primitive(JvmPrimitiveTypeValue),
	Reference(JvmReferenceTypeValue),
}

#[derive(Clone)]
pub enum JvmType {
	Primitive(JvmPrimitiveType),
	Reference(JvmReferenceType),
}

impl PartialEq for JvmType {
	fn eq(&self, other: &Self) -> bool {
		match self {
			JvmType::Primitive(s) => match other {
				JvmType::Primitive(o) => {
					println!("Comparing two primitives.");
					s == o
				}
				_ => {
					println!("Comparing a primitive with a non-primitive.");
					false
				}
			},
			JvmType::Reference(s) => match other {
				JvmType::Reference(o) => {
					/*
					 * TODO: Compare two reference types
					 */
					assert!(false, "TODO: Compare two reference types.");
					false
				}
				_ => {
					println!("Comparing a reference with a non-reference.");
					false
				}
			},
		}
	}
}

impl From<&[u8]> for JvmType {
	fn from(from: &[u8]) -> Self {
		if from[0] == 'V' as u8 {
			JvmType::Primitive(JvmPrimitiveType::Void)
		} else if from[0] == 'I' as u8 {
			JvmType::Primitive(JvmPrimitiveType::Integer)
		} else if from[0] == 'Z' as u8 {
			JvmType::Primitive(JvmPrimitiveType::Boolean)
		} else {
			FatalError::new(FatalErrorType::InvalidFieldType).call();
			JvmType::Primitive(JvmPrimitiveType::Invalid)
		}
	}
}
