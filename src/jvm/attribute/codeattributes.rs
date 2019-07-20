/*
 * FILE: XXXXX
 * DESCRIPTION:
 *
 * Copyright (c) 2019, Will Hawkins
 *
 * This file is part of Rust-JVM.
 *
 * Rust-JVM is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rust-JVM is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Rust-JVM.  If not, see <https://www.gnu.org/licenses/>.
 */
use jvm::exceptions::ExceptionTable;
use std::fmt;

pub struct CodeAttribute {
	pub bytes: Vec<u8>,
	pub code_offset: usize,
	max_stack: u16,
	max_locals: u16,
	code_length: u32,
	exceptions_table_count: u16,
	exceptions: ExceptionTable,
}

impl From<Vec<u8>> for CodeAttribute {
	fn from(bytes: Vec<u8>) -> Self {
		let mut offset: usize = 0;
		let code_offset: usize;
		let max_stack = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;
		let max_locals = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;
		let code_length = (bytes[offset + 0] as u32) << 24
			| (bytes[offset + 1] as u32) << 16
			| (bytes[offset + 2] as u32) << 8
			| (bytes[offset + 3] as u32) << 0;
		offset += 4;

		code_offset = offset;

		offset += (code_length as usize) * 1;

		let exceptions = ExceptionTable::from(&bytes[offset..].to_vec());

		exceptions.byte_len();

		CodeAttribute {
			bytes: bytes,
			max_stack: max_stack,
			max_locals: max_locals,
			code_length: code_length,
			code_offset: code_offset,
			exceptions_table_count: exceptions.exceptions_table_count(),
			exceptions: exceptions,
		}
	}
}

impl fmt::Display for CodeAttribute {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		write!(f, "max_stack: {}\n", self.max_stack);
		write!(f, "max_locals: {}\n", self.max_locals);
		write!(f, "code_length: {}\n", self.code_length);
		write!(f, "exceptions: {}\n", self.exceptions);
		result = write!(f, "bytes: ");
		for byte in &self.bytes {
			result = write!(f, "{:x} ", byte);
		}
		result
	}
}
