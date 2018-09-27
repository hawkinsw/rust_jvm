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
		Attribute{info : repeat(0 as u8).take(attribute_count).collect(), .. Default::default()}
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
	attributes: Vec<Attribute>,
}

impl Attributes {
	pub fn new(attributes_count: usize) -> Attributes {
		Attributes{attributes: repeat(Attribute::new(0)).take(attributes_count).collect()}
	}
	pub fn set(&mut self, index: usize, attribute: Attribute) {
		self.attributes[index] = attribute;
	}

	pub fn get(&self, index: usize) -> Attribute {
		self.attributes[index].clone()
	}

	pub fn len(&self) -> usize {
		self.attributes.len()
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


