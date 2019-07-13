use jvm::typevalues::JvmTypeValue;
use jvm::constantpool::ConstantPool;
use std::fmt;

#[derive(Clone,Default)]
pub struct Frame<'a> {
	pub constant_pool: Option<&'a ConstantPool>,
	pub operand_stack: Vec<JvmTypeValue<'a>>,
}

impl<'a> Frame<'a> {
	pub fn new() -> Self {
		Frame{operand_stack: Vec::<JvmTypeValue<'a>>::new(), constant_pool: None}
	}
}

impl<'a> fmt::Display for Frame<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		result = write!(f,"");
		for entry in &self.operand_stack {
			match entry {
				JvmTypeValue::Primitive(primitive) => result = write!(f, "Primitive: {}", primitive),
				JvmTypeValue::Reference(reference) => result = write!(f, "Reference: PASS" ),
			}
		}
		result
	}
}
