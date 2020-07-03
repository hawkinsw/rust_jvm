use std::fmt;

pub enum FatalErrorType {
	ClassNotFound(String),
	MethodNotFound(String, String),
	CouldNotLock(String, String),
	ClassNotLoaded(String),
	ClassInstantiationFailed(String),
	ClassNoName,
	WrongType(String, String),
	NotEnough(String, usize, String),
	MethodResolutionFailed,
	MethodSelectionFailed,
	MainMethodNotPublicStatic,
	MainMethodNotVoid,
	InvalidFieldType(char),
	InvalidMethodDescriptor,
	InvalidConstantReference(String, String, u16),
	VoidMethodReturnedValue,
	ClassInitMethodReturnedValue,
	RecursiveClassInitialization(String, String),
	MethodExecutionFailed(String),
	NotImplemented(String),
	RequiredStackValueNotFound(String),
	Exception(String),
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
			FatalErrorType::ClassNoName => write!(f, "Class has no name!"),
			FatalErrorType::WrongType(instruction, expected) => {
				write!(f, "Wrong type: {} requires {}.", instruction, expected)
			}
			FatalErrorType::NotEnough(instruction, needed, from) => {
				write!(f, "{} needs {} {}.", instruction, needed, from)
			}
			FatalErrorType::MethodResolutionFailed => write!(f, "Method resolution failed!"),
			FatalErrorType::MethodSelectionFailed => write!(f, "Method resolution failed!"),
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
			FatalErrorType::InvalidFieldType(field) => write!(f, "Invalid field type: {}.", field),
			FatalErrorType::MethodExecutionFailed(method) => {
				write!(f, "Method {} failed to execute.", method)
			}
			FatalErrorType::RecursiveClassInitialization(starting, inprogress) => write!(
				f,
				"Recursive initialization of {} not possible while initializing {}\n",
				starting, inprogress
			),
			FatalErrorType::NotImplemented(what) => write!(f, "{} not implemented.", what),
			FatalErrorType::RequiredStackValueNotFound(requirement) => {
				write!(f, "{} needs a stack value that was not found.", requirement)
			}
			FatalErrorType::Exception(exception_type) => {
				write!(f, "Exception: {}", exception_type)
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
