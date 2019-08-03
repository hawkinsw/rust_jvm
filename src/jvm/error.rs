use std::fmt;

pub enum FatalErrorType {
	ClassNotFound(String),
	MethodNotFound(String, String),
	CouldNotLock(String, String),
	ClassNotLoaded(String),
	ClassInstantiationFailed(String),
	MainMethodNotPublicStatic,
	MainMethodNotVoid,
	InvalidFieldType,
	InvalidMethodDescriptor,
	InvalidConstantReference(String, String, u16),
	VoidMethodReturnedValue,
	ClassInitMethodReturnedValue,
}

impl fmt::Display for FatalErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			FatalErrorType::ClassNotFound(c) => {
				write!(f, "Class {} is required, but couldn't be found.", c)
			}
			FatalErrorType::MethodNotFound(m, c) => write!(
				f,
				"Method {} in {} is required, but couldn't be found.",
				m, c
			),
			FatalErrorType::CouldNotLock(what, r#where) => {
				write!(f, "Could not lock {} at {}.", what, r#where)
			}
			FatalErrorType::ClassNotLoaded(class) => {
				write!(f, "Class {} instantiated but not loaded.", class)
			}
			FatalErrorType::ClassInstantiationFailed(class) => {
				write!(f, "Class {} could not be instantiated.", class)
			}
			FatalErrorType::MainMethodNotPublicStatic => {
				write!(f, "Main method is not public or not static.")
			}
			FatalErrorType::MainMethodNotVoid => write!(f, "Main method is not void."),
			FatalErrorType::ClassInitMethodReturnedValue => {
				write!(f, "Class initialization method returned a value.")
			}
			FatalErrorType::InvalidConstantReference(class, expected, index) => write!(
				f,
				"Invalid reference {} into Class {}'s constant pool; expected {}.",
				index, class, expected
			),
			FatalErrorType::InvalidFieldType => write!(f, "Main method is not void."),
			_ => write!(f, "Unhandled FatalErrorType."),
		}
	}
}

pub struct FatalError {
	error: FatalErrorType,
}

impl FatalError {
	pub fn new(error: FatalErrorType) -> Self {
		FatalError { error: error }
	}

	pub fn call(&self) {
		eprintln!("Fatal Error: {}", self.error);
		assert!(false);
	}
}
