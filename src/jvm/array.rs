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

use jvm::typevalues::JvmValue;
use std::fmt;

enum_from_primitive! {
	pub enum JvmArrayType {
		Boolean = 0x4,
		Char = 0x5,
		Float = 0x6,
		Double = 0x7,
		Byte = 0x8,
		Short = 0x9,
		Integer = 0x10,
		Long = 0x11,
	}
}
pub struct JvmArray {
	dimension: usize,
	values: Vec<Option<JvmValue>>,
}

impl JvmArray {
	pub fn new(dimension: usize) -> Self {
		let mut res = JvmArray {
			dimension,
			values: vec![],
		};
		res.values.resize(dimension as usize, None);
		res
	}

	pub fn inbounds(&self, index: usize) -> bool {
		index < self.dimension
	}

	pub fn push(&mut self, value: JvmValue) {
		self.values.push(Some(value));
	}

	pub fn set_at(&mut self, index: usize, value: JvmValue) {
		self.values[index] = Some(value);
	}

	pub fn get_at(&mut self, index: usize) -> &Option<JvmValue> {
		&self.values[index]
	}
}

impl fmt::Display for JvmArray {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "(Array with dimension {} with values ", self.dimension);
		for i in &self.values {
			if let Some(value) = i {
				write!(f, "{},", value);
			} else {
				write!(f, "None/Empty,");
			}
		}
		write!(f, ".")
	}
}
