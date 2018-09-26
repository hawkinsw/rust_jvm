use std::fmt;
use std::iter::repeat;
use jvm::attribute::Attributes;

#[derive(Default,Clone)]
pub struct Method {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub attributes: Attributes
}

impl Method {
	pub fn new(attribute_count: usize) -> Method {
		Method{attributes: Attributes::new(attribute_count), .. Default::default()}
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

#[derive(Default)]
pub struct Methods{
	methods: Vec<Method>,
}

impl Methods {
	pub fn new(method_count : usize) -> Methods{
		Methods{methods: repeat(Method::new(0)).take(method_count).collect()}
	}

	pub fn set(&mut self, index: usize, method: Method) {
		self.methods[index] = method;
	}

	pub fn get(&self, index: usize) -> Method {
		self.methods[index].clone()
	}
}

impl fmt::Display for Methods {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		result = Ok(());
		for i in 0 .. self.methods.len() {
			result = write!(f, "{}\n", self.methods[i as usize]);
		}
		result
	}
}
