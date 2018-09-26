use std::fmt;
use std::iter::repeat;
use jvm::attribute::Attributes;
use jvm::attribute::Attribute;

#[derive(Default,Clone)]
pub struct Field {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub attributes: Attributes,
}

impl Field {
	pub fn new(attribute_count: usize) -> Field {
		Field{attributes : Attributes::new(attribute_count), .. Default::default()}
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

#[derive(Default)]
pub struct Fields {
	fields: Vec<Field>,
}

impl Fields{
	pub fn new(field_count : usize) -> Fields {
		Fields{fields: repeat(Field::new(0)).take(field_count).collect()}
	}

	pub fn set(&mut self, index: usize, field: Field) {
		self.fields[index] = field;
	}

	pub fn get(&self, index: usize) -> Field {
		self.fields[index].clone()
	}
}

impl fmt::Display for Fields {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result = Ok(());
		for i in 0 .. self.fields.len() {
			result = write!(f, "{}\n", self.get(i))
		}
		result
	}
}
