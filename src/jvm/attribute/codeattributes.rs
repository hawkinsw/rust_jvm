use std::fmt;
use std::iter::repeat;
use jvm::exceptions::ExceptionTable;

pub struct CodeAttribute {
	bytes: Vec<u8>,
	max_stack: u16,
	max_locals: u16,
	code_length: u32,
	exceptions_table_count: u16,
	exceptions: ExceptionTable,
}

impl From<Vec<u8>> for CodeAttribute {
	fn from(bytes: Vec<u8>) -> Self {
		let mut offset: usize = 0;
		
		let max_stack = (bytes[offset+0] as u16) << 8|
		                (bytes[offset+1] as u16) << 0;
		offset+=2;
		let max_locals = (bytes[offset+0] as u16) << 8|
		                 (bytes[offset+1] as u16) << 0;
		offset+=2;
		let code_length = (bytes[offset+0] as u32) << 24|
		                  (bytes[offset+1] as u32) << 16|
		                  (bytes[offset+2] as u32) << 8|
		                  (bytes[offset+3] as u32) << 0;
		offset+=4;

		offset+=(code_length as usize*1);

		let exceptions=ExceptionTable::from(&bytes[offset..].to_vec());

		offset+=exceptions.byte_len();

		CodeAttribute{bytes: bytes,
		              max_stack: max_stack,
		              max_locals: max_locals,
		              code_length: code_length,
		              exceptions_table_count: exceptions.exceptions_table_count(),
		              exceptions: exceptions}
	}
}

impl fmt::Display for CodeAttribute {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result = Ok(());
		result = write!(f, "max_stack: {}\n", self.max_stack);
		result = write!(f, "max_locals: {}\n", self.max_locals);
		result = write!(f, "code_length: {}\n", self.code_length);
		result = write!(f, "exceptions: {}\n", self.exceptions);
		result
	}
}
