use std::fmt;

pub enum FatalErrorType {
	ClassNotLoaded(String),
}

impl fmt::Display for FatalErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			FatalErrorType::ClassNotLoaded(s) => {
				write!(f, "Class {} is required, but couldn't be found.", s)
			}
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
