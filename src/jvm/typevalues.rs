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
use jvm::constantpool::ConstantPool;
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use jvm::object::JvmObject;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub enum JvmPrimitiveType {
	Boolean,
	Integer,
	Void,
	Invalid,
}

impl fmt::Display for JvmPrimitiveType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			JvmPrimitiveType::Boolean => write!(f, "Boolean"),
			JvmPrimitiveType::Integer => write!(f, "Integer"),
			JvmPrimitiveType::Void => write!(f, "Void"),
			JvmPrimitiveType::Invalid => write!(f, "Invalid"),
		}
	}
}

#[derive(Clone)]
pub enum JvmReferenceType {
	Array(Rc<JvmValue>, u64),
	Class(Rc<JvmObject>),
	Interface(String),
}

#[derive(Clone)]
pub enum JvmValue {
	Primitive(JvmPrimitiveType, u64, u16),
	Reference(JvmReferenceType, u64, u16),
}

#[derive(Clone)]
pub enum JvmType {
	Primitive(JvmPrimitiveType),
	Reference(JvmReferenceType),
}
impl fmt::Display for JvmReferenceType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			JvmReferenceType::Array(_, _) => write!(f, "Array"),
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
			JvmValue::Reference(tipe, value, access) => {
				return write!(f, "Reference: {}: {} (access: {:x})", tipe, value, access)
			}
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
