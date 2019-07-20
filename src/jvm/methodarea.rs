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
use std::collections::HashMap;
use std::rc::Rc;

pub struct MethodArea {
	pub debug: bool,
	pub classes: HashMap<String, Rc<Class>>,
}

impl MethodArea {
	pub fn new(debug: bool) -> Self {
		MethodArea {
			debug: debug,
			classes: HashMap::new(),
		}
	}

	pub fn is_class_loaded(&self, class_name: &String) -> bool {
		self.classes.contains_key(class_name)
	}

	pub fn get_class_rc(&self, class_name: &String) -> Option<Rc<Class>> {
		if let Some(class_rc_ref) = self.classes.get(class_name) {
			Some(Rc::clone(class_rc_ref))
		} else {
			None
		}
	}

	pub fn load_class_from_file(&mut self, class_filename: &String) -> Option<Rc<Class>> {
		if let Some(class) = Class::load(class_filename) {
			if let Some(class_name) = class.get_class_name() {
				self.classes.insert(class_name.to_string(), Rc::new(class));
				return Some(Rc::clone(self.classes.get(&class_name).unwrap()));
			}
		}
		None
	}
}
