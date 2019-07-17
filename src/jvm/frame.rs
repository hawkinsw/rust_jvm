use jvm::typevalues::JvmTypeValue;
use jvm::constantpool::ConstantPool;
use jvm::class::Class;
use std::rc::Rc;
use std::fmt;

#[derive(Clone,Default)]
pub struct Frame {
	pub operand_stack: Vec<JvmTypeValue>,
	pub class: Option<Rc<Class>>
}

impl Frame {
	pub fn new() -> Self {
		Frame{operand_stack: Vec::<JvmTypeValue>::new(), class: None}
	}

	pub fn class(&self) -> Option<Rc<Class>> {
		if let Some(class) = &self.class {
			Some(Rc::clone(class))
		} else {
			None
		}
	}
}

impl fmt::Display for Frame {
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
