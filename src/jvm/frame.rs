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
use jvm::class::Class;
use jvm::typevalues::JvmValue;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct Frame {
	pub operand_stack: Vec<JvmValue>,
	pub class: Option<Rc<Class>>,
	pub locals: Vec<JvmValue>,
}

impl Frame {
	pub fn new() -> Self {
		Frame {
			operand_stack: Vec::<JvmValue>::new(),
			class: None,
			locals: Vec::<JvmValue>::new(),
		}
	}

	pub fn class(&self) -> Option<Rc<Class>> {
		if let Some(class) = &self.class {
			Some(Rc::clone(class))
		} else {
			None
		}
	}
}

impl fmt::Display for Frame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: fmt::Result;
		result = write!(f, "Operand stack:\n");
		for entry in &self.operand_stack {
			result = write!(f, "{}\n", entry);
		}
		result = write!(f, "==============\n");
		result = write!(f, "Locals:\n");
		for entry in &self.locals {
			result = write!(f, "{}\n", entry);
		}
		result = write!(f, "==============\n");

		result
	}
}
