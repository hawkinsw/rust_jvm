use std::fmt;
use std::iter::repeat;
use jvm::attribute::Attributes;
use jvm::constantpool::ConstantPool;
use jvm::constant::Constant;
use jvm::attribute::Attribute;
use jvm::attribute::codeattributes::CodeAttribute;

#[derive(Default,Clone)]
pub struct Method {
	byte_len: usize,
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub attributes: Attributes
}

impl Method {
	pub fn get_code_attribute(&self, cp: &ConstantPool) -> Option<CodeAttribute>{
		for i in 0 .. self.attributes.len() {
			let attribute = self.attributes.get(i);
			if let Constant::Utf8(_,_,value) = cp.get(attribute.attribute_name_index as usize) {
				if "Code".to_string() == value {
					return Some(CodeAttribute::from(attribute.info));
				}
			}
		}
		None
	}

	pub fn byte_len(&self) -> usize {
		self.byte_len
	}
}

impl<'l> From<&'l Vec<u8>> for Method {
	fn from(bytes: &'l Vec<u8>) -> Self {
			let mut offset = 0;
			let access_flags: u16;
			let name_index: u16;
			let descriptor_index: u16;
			let attributes_count: u16;
			let attributes: Attributes;

			access_flags = (bytes[offset] as u16) << 8 |
			               (bytes[offset+1] as u16) << 0;
			offset+=2;
			name_index = (bytes[offset] as u16) << 8 |
			             (bytes[offset+1] as u16) << 0;
			offset+=2;
			descriptor_index = (bytes[offset] as u16) << 8 |
			                   (bytes[offset+1] as u16) << 0;
			offset+=2;

			attributes = Attributes::from(&bytes[offset..].to_vec());
			offset += attributes.byte_len();

			Method{byte_len: offset, 
			       access_flags,
						 name_index,
						 descriptor_index,
						 attributes_count:
						 attributes.attributes_count(),
						 attributes}
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
	byte_len: usize,
	methods: Vec<Method>,
}

impl Methods {
	pub fn set(&mut self, index: usize, method: Method) {
		self.methods[index] = method;
	}

	pub fn get(&self, index: usize) -> Method {
		self.methods[index].clone()
	}

	pub fn methods_count(&self) -> u16 {
		self.methods.len() as u16 
	}

	pub fn byte_len(&self) -> usize {
		self.byte_len
	}

	pub fn get_by_name(&self, method_name: &String, cp: &ConstantPool) -> Option<&Method> {
		for i in 0 .. self.methods.len() {
			match cp.get(self.methods[i].name_index as usize) {
				Constant::Utf8(_, _, value) => {
					print!("value: {}\n", value);
					if value == *method_name {
						return Some(&self.methods[i])
					}
				},
				_ => () 
			}
		}
		None
	}
}

impl<'l> From<&'l Vec<u8>> for Methods {
	fn from(bytes: &'l Vec<u8>) -> Self {
		let mut offset = 0;
		let mut methods: Vec<Method>;
		let methods_count = (bytes[offset] as u16) << 8 |
		                    (bytes[offset+1] as u16) << 0;

		offset+=2;
		methods = repeat(Method{..Default::default()}).
		          take(methods_count as usize).
							collect();
		for method_index in 0 .. methods_count as usize {
			methods[method_index] = Method::from(&bytes[offset..].to_vec());
			offset+=methods[method_index].byte_len();
		}
		Methods{byte_len: offset, methods: methods}
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
