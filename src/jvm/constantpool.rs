use jvm::constant::Constant;
use std::iter::repeat;

#[derive(Default)]
pub struct ConstantPool {
	constants: Vec<Constant>,
}

impl ConstantPool {
	pub fn new(constant_count : usize) -> ConstantPool {
		ConstantPool{constants: repeat(Constant::Default()).take(constant_count).collect()}
	}

	pub fn set(&mut self, index: usize, constant: Constant) {
		self.constants[index] = constant;
	}

	pub fn get(&self, index: usize) -> Constant {
		self.constants[index].clone()
	}
}
