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
use std::fmt;
use std::iter::repeat;

#[derive(Default, Clone)]
pub struct Exception {
	start_pc: u16,
	end_pc: u16,
	handler_pc: u16,
	catch_type: u16,
}

impl Exception {
	pub fn byte_len(&self) -> usize {
		8 as usize
	}
}

impl<'l> From<&'l Vec<u8>> for Exception {
	fn from(bytes: &'l Vec<u8>) -> Self {
		let mut offset: usize = 0;
		let start_pc: u16;
		let end_pc: u16;
		let handler_pc: u16;
		let catch_type: u16;

		start_pc = (bytes[offset] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;

		end_pc = (bytes[offset] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;

		handler_pc = (bytes[offset] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;

		catch_type = (bytes[offset] as u16) << 8 | (bytes[offset + 1] as u16) << 0;

		Exception {
			start_pc,
			end_pc,
			handler_pc,
			catch_type,
		}
	}
}

#[derive(Default, Clone)]
pub struct ExceptionTable {
	byte_len: usize,
	exceptions: Vec<Exception>,
}

impl ExceptionTable {
	pub fn byte_len(&self) -> usize {
		self.byte_len
	}
	pub fn exceptions_table_count(&self) -> u16 {
		self.exceptions.len() as u16
	}
}

impl<'l> From<&'l Vec<u8>> for ExceptionTable {
	fn from(bytes: &'l Vec<u8>) -> Self {
		let mut offset: usize = 0;
		let exceptions_count: u16;
		let mut exceptions: Vec<Exception>;
		exceptions_count =
			(bytes[offset + 0] as u16) << 8 as u16 | (bytes[offset + 1] as u16) << 0 as u16;
		offset += 2;
		exceptions = repeat(Exception {
			..Default::default()
		})
		.take(exceptions_count as usize)
		.collect();

		for i in 0..exceptions_count as usize {
			exceptions[i] = Exception::from(&bytes[offset..].to_vec());
			offset += exceptions[i].byte_len();
		}
		ExceptionTable {
			byte_len: offset,
			exceptions,
		}
	}
}

impl fmt::Display for ExceptionTable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result = Ok(());
		for i in 0..self.exceptions.len() {
			let exception = &self.exceptions[i];
			result = write!(f, "Exception: ");
			result = write!(
				f,
				"start_pc: {}, end_pc: {}, handler_pc: {}, catch_type: {}",
				exception.start_pc, exception.end_pc, exception.handler_pc, exception.catch_type
			);
		}
		result
	}
}
