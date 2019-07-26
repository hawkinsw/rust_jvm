use std::fmt;

pub enum FatalErrorType {
	ClassNotLoaded(String),
	MainMethodNotPublicStatic,
	MainMethodNotVoid,
	InvalidFieldType,
	InvalidMethodDescriptor,
	VoidMethodReturnedValue,
}

impl fmt::Display for FatalErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			FatalErrorType::ClassNotLoaded(s) => {
				write!(f, "Class {} is required, but couldn't be found.", s)
			}
			FatalErrorType::MainMethodNotPublicStatic => {
				write!(f, "Main method is not public or not static.")
			}
			FatalErrorType::MainMethodNotVoid => write!(f, "Main method is not void."),
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
