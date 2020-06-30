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
use jvm::array::JvmArray;
use jvm::class::Class;
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use jvm::object::JvmObject;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Clone)]
pub enum JvmPrimitiveType {
	Void,
	Byte,
	Char,
	Double,
	Float,
	Integer,
	LongInteger,
	Short,
	Boolean,
	Invalid,
}

impl fmt::Display for JvmPrimitiveType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			JvmPrimitiveType::Void => write!(f, "Void"),
			JvmPrimitiveType::Byte => write!(f, "Byte"),
			JvmPrimitiveType::Char => write!(f, "Char"),
			JvmPrimitiveType::Double => write!(f, "Double"),
			JvmPrimitiveType::Float => write!(f, "Float"),
			JvmPrimitiveType::Integer => write!(f, "Integer"),
			JvmPrimitiveType::LongInteger => write!(f, "LongInteger"),
			JvmPrimitiveType::Short => write!(f, "Short"),
			JvmPrimitiveType::Boolean => write!(f, "Boolean"),
			JvmPrimitiveType::Invalid => write!(f, "Invalid"),
		}
	}
}

#[derive(Clone)]
pub enum JvmReferenceTargetType {
	Null,
	Array(Arc<Mutex<JvmArray>>),
	Object(Arc<Mutex<JvmObject>>),
	Class(Class),
}

#[derive(Clone)]
pub enum JvmReferenceType {
	Null,
	Array(Rc<JvmType>, u64),
	Class(String),
	Interface(String),
}

#[derive(Clone)]
pub enum JvmValue {
	Primitive(JvmPrimitiveType, u64, u16),
	Reference(JvmReferenceType, JvmReferenceTargetType, u16),
}

pub fn create_null_value() -> JvmValue {
	JvmValue::Reference(JvmReferenceType::Null, JvmReferenceTargetType::Null, 0)
}

#[derive(Clone)]
pub enum JvmType {
	Primitive(JvmPrimitiveType),
	Reference(JvmReferenceType),
}

impl Default for JvmType {
	fn default() -> Self {
		JvmType::Primitive(JvmPrimitiveType::Invalid)
	}
}

impl fmt::Display for JvmReferenceType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			JvmReferenceType::Null => write!(f, "Null"),
			JvmReferenceType::Array(_, len) => write!(f, "Array: {}", len),
			JvmReferenceType::Class(_) => write!(f, "Class"),
			JvmReferenceType::Interface(_) => write!(f, "Interface"),
		}
	}
}

impl fmt::Display for JvmValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			JvmValue::Primitive(tipe, value, access) => {
				return write!(f, "Value: {}: {} (access: {:x})", tipe, value, access)
			}
			JvmValue::Reference(tipe, value, access) => match value {
				JvmReferenceTargetType::Array(array) => {
					if let Ok(exclusive_array) = array.lock() {
						return write!(
							f,
							"Reference: {} (access: {:x} of {})",
							tipe, access, exclusive_array
						);
					} else {
						return write!(f, "Reference: {} (access: {:x})", tipe, access);
					}
				}
				_ => return write!(f, "Reference: {} (access: {:x})", tipe, access),
			},
		}
	}
}

impl fmt::Display for JvmType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			JvmType::Primitive(p) => write!(f, "Primitive: {}", p),
			JvmType::Reference(r) => write!(f, "Reference: {}", r),
		}
	}
}

impl PartialEq for JvmReferenceType {
	fn eq(&self, other: &Self) -> bool {
		assert!(false, "TODO: Implement PartialEq for JvmReferenceType");
		false
	}
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
					println!("Comparing two references.");
					o == s
				}
				_ => {
					println!("Comparing a reference with a non-reference.");
					false
				}
			},
		}
	}
}

/*
 * PartialEq -- type and value must equal; the
 * access flags do not.
 */
impl PartialEq for JvmValue {
	fn eq(&self, other: &Self) -> bool {
		match self {
			JvmValue::Primitive(t, v, _) => match other {
				JvmValue::Primitive(ot, ov, _) => ot == t && ov == v,
				_ => false,
			},
			JvmValue::Reference(t, v, _) => match other {
				JvmValue::Reference(ot, ov, _) => ot == t && ov == v,
				_ => false,
			},
		}
	}
}

impl PartialEq for JvmReferenceTargetType {
	fn eq(&self, other: &Self) -> bool {
		match self {
			JvmReferenceTargetType::Array(v) => {
				if let JvmReferenceTargetType::Array(ov) = other {
					Arc::ptr_eq(ov, v)
				} else {
					false
				}
			}
			JvmReferenceTargetType::Object(v) => {
				if let JvmReferenceTargetType::Object(ov) = other {
					Arc::ptr_eq(ov, v)
				} else {
					false
				}
			}
			JvmReferenceTargetType::Class(v) => {
				if let JvmReferenceTargetType::Class(ov) = other {
					v.get_class_name() == ov.get_class_name()
				} else {
					false
				}
			}
			_ => false,
		}
	}
}

impl From<&[u8]> for JvmType {
	fn from(from: &[u8]) -> Self {
		let mut result = JvmType::Primitive(JvmPrimitiveType::Invalid);
		match from[0] as char {
			'V' => {
				result = JvmType::Primitive(JvmPrimitiveType::Void);
			}
			'B' => {
				result = JvmType::Primitive(JvmPrimitiveType::Byte);
			}
			'C' => {
				result = JvmType::Primitive(JvmPrimitiveType::Char);
			}
			'F' => {
				result = JvmType::Primitive(JvmPrimitiveType::Float);
			}
			'D' => {
				result = JvmType::Primitive(JvmPrimitiveType::Double);
			}
			'I' => {
				result = JvmType::Primitive(JvmPrimitiveType::Integer);
			}
			'J' => {
				result = JvmType::Primitive(JvmPrimitiveType::LongInteger);
			}
			'L' => {
				/*
				 * Walk through the end of the string
				 * and make that our class name.
				 */
				let mut index = 1;
				while from[index] != ';' as u8 {
					index = index + 1;
				}
				if let Ok(classname) = std::str::from_utf8(&from[1..index]) {
					result = JvmType::Reference(JvmReferenceType::Class(classname.to_string()))
				} else {
					FatalError::new(FatalErrorType::InvalidFieldType('L')).call();
				}
			}
			'S' => {
				result = JvmType::Primitive(JvmPrimitiveType::Short);
			}
			'Z' => {
				result = JvmType::Primitive(JvmPrimitiveType::Boolean);
			}
			'[' => {
				result = JvmType::Reference(JvmReferenceType::Array(
					Rc::<JvmType>::new(JvmType::from(&from[1..])),
					0,
				));
			}

			_ => {
				FatalError::new(FatalErrorType::InvalidFieldType(from[0] as char)).call();
			}
		}
		result
	}
}
