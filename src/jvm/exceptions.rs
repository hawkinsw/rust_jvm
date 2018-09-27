use std::iter;

#[derive(Default, Clone)]
pub struct Exception {
	start_pc: u16,
	end_pc: u16,
	handler_pc: u16,
	catch_type: u16,
}

#[derive(Default,Clone)]
pub struct ExceptionTable {
	exceptions: Vec<Exception>,
}

impl ExceptionTable {
	pub fn load(bytes: &Vec<u8>, exceptions_count: usize) -> ExceptionTable {
		let mut offset: usize = 0;
		let mut table = ExceptionTable{exceptions: iter::repeat(Exception{.. Default::default()}).take(exceptions_count as usize).collect()};

		for i in 0 .. exceptions_count {
			let start_pc:u16;
			let end_pc:u16;
			let handler_pc: u16;
			let catch_type: u16;

			start_pc = (bytes[offset] as u16) << 8 |
			           (bytes[offset+1] as u16) <<0;
			offset+=2;

			end_pc = (bytes[offset] as u16) << 8 |
			         (bytes[offset+1] as u16) <<0;
			offset+=2;

			handler_pc = (bytes[offset] as u16) << 8 |
			             (bytes[offset+1] as u16) <<0;
			offset+=2;

			catch_type = (bytes[offset] as u16) << 8 |
			             (bytes[offset+1] as u16) <<0;
			offset+=2;
			table.exceptions[i] = Exception{start_pc: start_pc,
			                     end_pc: end_pc,
			                     handler_pc: handler_pc,
			                     catch_type: catch_type};
		}
		table
	}
}
