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
use jvm::attribute::Attributes;
use jvm::constant::Constant;
use jvm::constant::Utf8Reserved;
use jvm::constantpool::ConstantPool;
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use jvm::typevalues::JvmPrimitiveType;
use jvm::typevalues::JvmType;
use std::fmt;
use std::iter::repeat;

#[repr(u16)]
pub enum MethodAccessFlags {
	Public = 0x0001,
	Private = 0x0002,
	Protected = 0x0004,
	Static = 0x0008,
	Final = 0x0010,
	Synchronized = 0x0020,
	Bridge = 0x0040,
	VarArgs = 0x0080,
	Native = 0x0100,
	Abstract = 0x0400,
	Strict = 0x0800,
	Synthetic = 0x1000,
}

#[derive(Default, Clone)]
pub struct Method {
	byte_len: usize,
	pub access_flags: u16,
	pub name_index: u16,
	pub class_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub max_locals: usize,
	pub parameter_count: usize,
	pub return_type: JvmType,
	pub attributes: Attributes,
}

impl Method {
	pub fn get_code(&self, cp: &ConstantPool) -> Option<&[u8]> {
		for i in 0..self.attributes.len() {
			let attribute = self.attributes.get_ref(i);
			if let Constant::Utf8(_, reserved, _, _) =
				cp.get_constant_ref(attribute.attribute_name_index as usize)
			{
				if let Utf8Reserved::Code = reserved {
					return Some(&attribute.info[8..]);
				}
			}
		}
		None
	}

	pub fn byte_len(&self) -> usize {
		self.byte_len
	}
}

impl<'l> From<(&'l Vec<u8>, &'l ConstantPool)> for Method {
	fn from(f: (&'l Vec<u8>, &'l ConstantPool)) -> Self {
		let (bytes, cp) = f;
		let mut offset = 0;
		let access_flags: u16;
		let name_index: u16;
		let descriptor_index: u16;
		let attributes: Attributes;
		let max_locals: usize;
		let parameter_count: usize;
		let return_type: JvmType;

		access_flags = (bytes[offset] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;
		name_index = (bytes[offset] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;
		descriptor_index = (bytes[offset] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;

		attributes = Attributes::from(&bytes[offset..].to_vec());
		offset += attributes.byte_len();

		/*
		 * Get the number of max locals.
		 */
		max_locals = {
			let mut max_locals: usize = 0;
			for i in 0..attributes.len() {
				let attribute = attributes.get_ref(i);
				if let Constant::Utf8(_, reserved, _, _) =
					cp.get_constant_ref(attribute.attribute_name_index as usize)
				{
					if let Utf8Reserved::Code = reserved {
						max_locals =
							u16::from_le_bytes([attribute.info[3], attribute.info[2]]) as usize;
					}
				}
			}
			max_locals
		};

		/*
		 * Get the parameter count.
		 */
		parameter_count =
			if let Constant::Utf8(_, _, _, s) = cp.get_constant_ref(descriptor_index as usize) {
				let mut parameter_count: usize = 0;
				let signature = s.as_bytes();
				if signature[0] == '(' as u8 {
					let mut i = 1;
					while i < signature.len() && signature[i] != ')' as u8 {
						if signature[i] == 'L' as u8 {
							/*
							 * Lsome/class/name;
							 * means a reference to a class of that name.
							 */
							while signature[i] != ';' as u8 {
								i = i + 1;
							}
						} else if signature[i] == '[' as u8 {
							i = i + 1;
						}
						i = i + 1;
						parameter_count += 1;
					}
				}
				parameter_count
			} else {
				0
			};

		/*
		 * Get the return type.
		 */
		return_type = {
			let mut return_type: JvmType = JvmType::Primitive(JvmPrimitiveType::Invalid);
			if let Constant::Utf8(_, _, _, s) = cp.get_constant_ref(descriptor_index as usize) {
				let signature = s.as_bytes();
				if let Some(index) = s.find(')') {
					return_type = JvmType::from(&signature[index + 1..])
				}
			}
			return_type
		};

		Method {
			byte_len: offset,
			access_flags,
			name_index,
			class_index: 0,
			descriptor_index,
			attributes_count: attributes.attributes_count(),
			max_locals: max_locals,
			parameter_count: parameter_count,
			return_type: return_type,
			attributes,
		}
	}
}

impl fmt::Display for Method {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "access_field: {:x}, name_index: {}, descriptor_index: {}, attributes_count: {}, attributes: {}",
			self.access_flags,
			self.name_index,
			self.descriptor_index,
			self.attributes_count,
			self.attributes)
	}
}

#[derive(Clone, Default)]
pub struct Methods {
	byte_len: usize,
	methods: Vec<Method>,
}

impl Methods {
	pub fn set(&mut self, index: usize, method: Method) {
		self.methods[index] = method;
	}

	pub fn get(&self, index: usize) -> &Method {
		&self.methods[index]
	}

	pub fn methods_count(&self) -> u16 {
		self.methods.len() as u16
	}

	pub fn byte_len(&self) -> usize {
		self.byte_len
	}

	pub fn get_by_name_and_type(
		&self,
		method_name: &String,
		method_type: &String,
		cp: &ConstantPool,
	) -> Option<&Method> {
		for i in 0..self.methods.len() {
			if let Constant::Utf8(_, _, _, value) =
				cp.get_constant_ref(self.methods[i].name_index as usize)
			{
				if *value == *method_name {
					if let Constant::Utf8(_, _, _, value) =
						cp.get_constant_ref(self.methods[i].descriptor_index as usize)
					{
						if *value == *method_type {
							return Some(&self.methods[i]);
						}
					}
				}
			}
		}
		None
	}
}

impl<'l> From<(&'l Vec<u8>, &'l ConstantPool)> for Methods {
	fn from(f: (&'l Vec<u8>, &'l ConstantPool)) -> Self {
		let (bytes, cp) = f;
		let mut offset = 0;
		let mut methods: Vec<Method>;
		let methods_count = (bytes[offset] as u16) << 8 | (bytes[offset + 1] as u16) << 0;

		offset += 2;
		methods = repeat(Method {
			..Default::default()
		})
		.take(methods_count as usize)
		.collect();
		for method_index in 0..methods_count as usize {
			methods[method_index] = Method::from((&bytes[offset..].to_vec(), cp));
			offset += methods[method_index].byte_len();
		}
		Methods {
			byte_len: offset,
			methods: methods,
		}
	}
}

impl fmt::Display for Methods {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		result = Ok(());
		for i in 0..self.methods.len() {
			result = write!(f, "{}\n", self.methods[i as usize]);
		}
		result
	}
}

pub struct MethodIterator<'a> {
	curr: usize,
	max: usize,
	methods: &'a Methods,
}

impl<'a> MethodIterator<'a> {
	pub fn new(methods: &'a Methods) -> Self {
		MethodIterator {
			curr: 0,
			max: methods.methods_count() as usize,
			methods,
		}
	}
}

impl<'a> Iterator for MethodIterator<'a> {
	type Item = &'a Method;

	fn next(&mut self) -> Option<&'a Method> {
		if self.curr < self.max {
			self.curr += 1;
			Some(self.methods.get(self.curr - 1))
		} else {
			None
		}
	}
}
