use std::fmt;
use std::iter::repeat;
use jvm::exceptions::ExceptionTable;

pub struct CodeAttribute<'l> {
	bytes: &'l Vec<u8>,
	attribute_name_index: u16,
	attribute_length: u32,
	max_stack: u16,
	max_locals: u16,
	code_length: u32,
	exceptions_table_count: u16,
	exceptions: ExceptionTable,
}

impl<'l> CodeAttribute<'l> {
	pub fn load(bytes: &Vec<u8>) -> CodeAttribute {
		let mut offset: usize = 0;

		let attribute_name_index = (bytes[offset+0] as u16) << 8|
		                           (bytes[offset+1] as u16) << 0;
		offset+=2;
		let attribute_length = (bytes[offset+0] as u32) << 24|
		                       (bytes[offset+1] as u32) << 16|
		                       (bytes[offset+2] as u32) << 8|
		                       (bytes[offset+3] as u32) << 0;
		offset+=4;

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

		let exceptions_table_count = (bytes[offset+0] as u16) << 8|
		                             (bytes[offset+1] as u16) << 0;
		offset+=2;

		let exceptions=ExceptionTable::load(&bytes[offset..].to_vec(), exceptions_table_count as usize);

		CodeAttribute{bytes: bytes,
		              attribute_name_index: attribute_name_index,
		              attribute_length: attribute_length,
		              max_stack: max_stack,
		              max_locals: max_locals,
		              code_length: code_length,
		              exceptions_table_count: exceptions_table_count,
		              exceptions: exceptions}
	}
}

impl<'l> fmt::Display for CodeAttribute<'l> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Ok(())
	}
}
