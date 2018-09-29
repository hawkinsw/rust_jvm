use std::fmt;
use std::iter::repeat;

pub mod codeattributes;

#[derive(Default,Clone)]
pub struct Attribute {
	pub attribute_name_index: u16,
	pub attribute_length: u32,
	pub info: Vec<u8>,
}

impl Attribute {
	pub fn new(attribute_count: usize) -> Attribute {
		Attribute{info : repeat(0 as u8).
		                 take(attribute_count).
										 collect(), .. Default::default()}
	}
}

impl fmt::Display for Attribute {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		result = write!(f, "attribute_name_index: {}, attribute_length: {}, attributes: ", 
			self.attribute_name_index,
			self.attribute_length);
		for i in 0 .. self.attribute_length {
			result = write!(f, "{:X} ", self.info[i as usize]);
		}
		result
	}
}

#[derive(Default,Clone)]
pub struct Attributes {
	byte_len: usize,
	attributes: Vec<Attribute>,
}

impl Attributes {
	pub fn set(&mut self, index: usize, attribute: Attribute) {
		self.attributes[index] = attribute;
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
		let attributes_count: u16 = (bytes[offset+0] as u16) << 8 |
		                            (bytes[offset+1] as u16) << 0;

		offset+=2;

		attributes = repeat(Attribute{.. Default::default()}).
		             take(attributes_count as usize).
								 collect();

		for attribute_index in 0 .. attributes_count as usize {
			let attribute_name_index: u16;
			let attribute_length: u32;
			let mut attribute: Attribute;

			attribute_name_index = (bytes[offset+0] as u16) << 8 |
			                       (bytes[offset+1] as u16);
			offset+=2;
			attribute_length = (bytes[offset+0] as u32) << 24 |
			                   (bytes[offset+1] as u32) << 16 |
			                   (bytes[offset+2] as u32) << 8  |
			                   (bytes[offset+3] as u32);
			offset+=4;
			attribute = Attribute::new(attribute_length as usize);
			attribute.attribute_name_index = attribute_name_index;
			attribute.attribute_length = attribute_length;
			/*
			 * Parse the attributes
			 */
			for info in 0 .. attribute_length {
				attribute.info[info as usize] = bytes[offset];
				offset+=1;
			}
			/*
			 * Assign the completed method attribute
			 */
			attributes[attribute_index as usize] = attribute;
		}

		Attributes{byte_len: offset, attributes: attributes}
	}
}

impl fmt::Display for Attributes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		result = Ok(());
		for i in 0 .. self.attributes.len() {
			result = write!(f, "{}\n", self.attributes[i]);
		}
		result
	}
}
