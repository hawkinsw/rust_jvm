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
use jvm::constantpool::ConstantPool;
use jvm::typevalues::JvmValue;
use std::fmt;
use std::iter::repeat;
use std::rc::Rc;

#[repr(u16)]
pub enum FieldAccessFlags {
	Public = 0x0001,
	Private = 0x0002,
	Protected = 0x0004,
	Static = 0x0008,
	Final = 0x0010,
	Volatile = 0x0040,
	Transient = 0x0080,
	Synthetic = 0x0100,
	Enum = 0x0400,
}

#[derive(Default, Clone)]
pub struct Field {
	pub byte_len: usize,
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub attributes: Attributes,
	pub value: Option<Rc<JvmValue>>,
}

impl Field {
	pub fn byte_len(&self) -> usize {
		self.byte_len
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "access_flags: {}, name_index: {}, descriptor_index: {}, attributes_count: {}, attributes: {}",
			self.access_flags,
			self.name_index,
			self.descriptor_index,
			self.attributes_count,
			self.attributes)
	}
}

impl<'l> From<&'l Vec<u8>> for Field {
	fn from(bytes: &'l Vec<u8>) -> Self {
		let mut offset = 0;
		let access_flags: u16;
		let name_index: u16;
		let descriptor_index: u16;
		let attributes: Attributes;

		access_flags = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
		offset += 2;
		name_index = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
		offset += 2;
		descriptor_index = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
		offset += 2;

		attributes = Attributes::from(&bytes[offset..].to_vec());
		offset += attributes.byte_len();

		Field {
			byte_len: offset,
			access_flags,
			name_index,
			descriptor_index,
			attributes_count: attributes.attributes_count(),
			attributes,
			value: None,
		}
	}
}

#[derive(Clone, Default)]
pub struct Fields {
	byte_len: usize,
	fields: Vec<Field>,
}

impl Fields {
	pub fn set(&mut self, index: usize, field: Field) {
		self.fields[index] = field;
	}

	pub fn get(&self, index: usize) -> Field {
		self.fields[index].clone()
	}

	pub fn byte_len(&self) -> usize {
		self.byte_len
	}

	pub fn fields_count(&self) -> u16 {
		self.fields.len() as u16
	}

	pub fn get_field_ref(&self, name: &str, r#type: &str, cp: &ConstantPool) -> Option<&Field> {
		for field in &self.fields
		{
			if let Constant::Utf8(_, _, _, current_name) = cp.get_constant_ref(field.name_index as usize)
			{
				if name != current_name {
					continue;
				}
				if let Constant::Utf8(_, _, _, current_descriptor) =
					cp.get_constant_ref(field.descriptor_index as usize)
				{
					if r#type == current_descriptor {
						println!("Found the field!");
						return Some(field);
					}
				}
			}
		}
		None
	}
	
	pub fn set_field_value(&mut self, name: &str, cp: &ConstantPool, value: &Rc<JvmValue>) -> Option<&Field> {
		for field in &mut self.fields
		{
			if let Constant::Utf8(_, _, _, current_name) = cp.get_constant_ref(field.name_index as usize)
			{
				if name != current_name {
					continue;
				}
				if let Constant::Utf8(_, _, _, current_descriptor) =
					cp.get_constant_ref(field.descriptor_index as usize)
				{
					field.value = Some(Rc::clone(value));
				}
			}
		}
		None
	}

	pub fn contains_field_with_name_and_type(
		&self,
		name: &String,
		r#type: &String,
		cp: &ConstantPool,
	) -> bool {
		for Field {
			byte_len: _,
			access_flags: _,
			name_index,
			descriptor_index,
			..
		} in &self.fields
		{
			if let Constant::Utf8(_, _, _, current_name) = cp.get_constant_ref(*name_index as usize)
			{
				if name != current_name {
					continue;
				}
				if let Constant::Utf8(_, _, _, current_descriptor) =
					cp.get_constant_ref(*descriptor_index as usize)
				{
					if r#type == current_descriptor {
						println!("Found the field!");
						return true;
					}
				}
			}
		}
		false
	}
}

impl<'l> From<&'l Vec<u8>> for Fields {
	fn from(bytes: &'l Vec<u8>) -> Self {
		let mut offset = 0;
		let mut fields: Vec<Field>;
		let fields_count: u16 = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
		offset += 2;

		fields = repeat(Field {
			..Default::default()
		})
		.take(fields_count as usize)
		.collect();
		for field_index in 0..fields_count as usize {
			fields[field_index] = Field::from(&bytes[offset..].to_vec());
			offset += fields[field_index].byte_len();
		}
		Fields {
			byte_len: offset,
			fields: fields,
		}
	}
}

impl fmt::Display for Fields {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result = Ok(());
		for i in 0..self.fields.len() {
			result = write!(f, "{}\n", self.get(i))
		}
		result
	}
}
