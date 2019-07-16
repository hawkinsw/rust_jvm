use std::fmt;

#[derive(Clone)]
pub enum JvmPrimitiveType {
	Boolean,
	Integer,
}

#[derive(Clone)]
pub enum JvmReferenceType<'a> {
	Array(&'a JvmTypeValue<'a>, u64),
	Class(String),
	Interface(String),
}

#[derive(Clone)]
pub struct JvmPrimitiveTypeValue {
	tipe: JvmPrimitiveType,
	value: i64,
}

impl JvmPrimitiveTypeValue {
	pub fn new(tipe: JvmPrimitiveType, value: i64) -> Self {
		Self{tipe: tipe, value: value}
	}
}

#[derive(Clone)]
pub struct JvmReferenceTypeValue<'a> {
	tipe: JvmReferenceType<'a>,
	reference: u64,
}

impl<'a> JvmReferenceTypeValue<'a> {
	pub fn new_array(dimension: u64, component_type: &'a JvmTypeValue<'a>, reference: u64) -> Self {
		JvmReferenceTypeValue{tipe: JvmReferenceType::Array(component_type, dimension), reference: reference}
	}

	pub fn new_class(name: String, reference: u64) -> Self {
		JvmReferenceTypeValue{tipe: JvmReferenceType::Class(name), reference: reference}
	}

	pub fn new_interface(name: String, reference: u64) -> Self {
		JvmReferenceTypeValue{tipe: JvmReferenceType::Interface(name), reference: reference}
	}
}

impl fmt::Display for JvmPrimitiveTypeValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let type_name = match self.tipe {
			JvmPrimitiveType::Boolean => "Boolean",
			JvmPrimitiveType::Integer => "Boolean",
		};
		write!(f, "{}: {}", type_name, self.value)
	}
}

#[derive(Clone)]
pub enum JvmTypeValue<'a> {
	Primitive(JvmPrimitiveTypeValue),
	Reference(JvmReferenceTypeValue<'a>),
}
