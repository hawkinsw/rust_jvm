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
use jvm::constantpool::ConstantPool;
use jvm::typevalues::JvmValue;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct JvmObject {
	spr: Option<Rc<JvmObject>>,
	class: Rc<Class>,
	fields: HashMap<String, Rc<JvmValue>>,
}

impl JvmObject {
	pub fn new(class: Rc<Class>) -> Self {
		JvmObject {
			spr: None,
			class: class,
			fields: HashMap::<String, Rc<JvmValue>>::new(),
		}
	}
	pub fn instantiate(&mut self) -> bool {
		let fields = self.class.get_fields_ref();
		for i in 0..fields.fields_count() {
			let field = fields.get(i as usize);
			/*
			 * TODO: START HERE!\n
			 */
		}
		true
	}
}

impl fmt::Display for JvmObject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Object of type {}", self.class.get_class_name().unwrap())
	}
}
