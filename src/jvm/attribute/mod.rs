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
use std::fmt;
use std::iter::repeat;

pub mod codeattributes;

#[derive(Default, Clone)]
pub struct Attribute {
	pub byte_len: usize,
	pub attribute_name_index: u16,
	pub attribute_length: u32,
	pub info: Vec<u8>,
}

impl Attribute {
	pub fn new(attribute_count: usize) -> Attribute {
		Attribute {
			info: repeat(0 as u8).take(attribute_count).collect(),
			..Default::default()
		}
	}

	pub fn byte_len(&self) -> usize {
		self.byte_len
	}
}

impl fmt::Display for Attribute {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		result = write!(
			f,
			"attribute_name_index: {}, attribute_length: {}, attributes: ",
			self.attribute_name_index, self.attribute_length
		);
		for byte in &self.info {
			result = write!(f, "{:x} ", byte);
		}
		result
	}
}

impl<'l> From<&'l Vec<u8>> for Attribute {
	fn from(bytes: &'l Vec<u8>) -> Self {
		let mut offset = 0;
		let attribute_name_index: u16;
		let attribute_length: u32;
		let mut info: Vec<u8>;

		attribute_name_index = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
		offset += 2;
		attribute_length = (bytes[offset + 0] as u32) << 24
			| (bytes[offset + 1] as u32) << 16
			| (bytes[offset + 2] as u32) << 8
			| (bytes[offset + 3] as u32);
		offset += 4;
		info = repeat(0).take(attribute_length as usize).collect();
		/*
		 * Parse the attributes
		 */
		for ii in 0..attribute_length {
			info[ii as usize] = bytes[offset];
			offset += 1;
		}
		Attribute {
			byte_len: offset,
			attribute_name_index,
			attribute_length,
			info,
		}
	}
}

#[derive(Default, Clone)]
pub struct Attributes {
	byte_len: usize,
	attributes: Vec<Attribute>,
}

impl Attributes {
	pub fn set(&mut self, index: usize, attribute: Attribute) {
		self.attributes[index] = attribute;
	}

	pub fn get_ref(&self, index: usize) -> &Attribute {
		&self.attributes[index]
	}

	pub fn get(&self, index: usize) -> Attribute {
		self.attributes[index].clone()
	}

	pub fn len(&self) -> usize {
		self.attributes.len()
	}

	pub fn byte_len(&self) -> usize {
		self.byte_len
	}

	pub fn attributes_count(&self) -> u16 {
		self.attributes.len() as u16
	}
}

impl<'l> From<&'l Vec<u8>> for Attributes {
	fn from(bytes: &'l Vec<u8>) -> Self {
		let mut offset = 0;
		let mut attributes: Vec<Attribute>;
		let attributes_count: u16 =
			(bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16) << 0;

		offset += 2;

		attributes = repeat(Attribute {
			..Default::default()
		})
		.take(attributes_count as usize)
		.collect();

		for attribute_index in 0..attributes_count as usize {
			attributes[attribute_index as usize] = Attribute::from(&bytes[offset..].to_vec());
			offset += attributes[attribute_index as usize].byte_len();
		}

		Attributes {
			byte_len: offset,
			attributes,
		}
	}
}

impl fmt::Display for Attributes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		result = Ok(());
		for i in 0..self.attributes.len() {
			result = write!(f, "{}\n", self.attributes[i]);
		}
		result
	}
}
