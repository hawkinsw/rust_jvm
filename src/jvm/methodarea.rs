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
use jvm::debug::Debug;
use jvm::debug::DebugLevel;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::LockResult;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct LoadedClass {
	initialized: bool,
	pub class: Rc<Class>,
}

impl LoadedClass {
	fn new(class: Class) -> Self {
		LoadedClass {
			class: Rc::new(class),
			initialized: false,
		}
	}
	pub fn is_initialized(&self) -> bool {
		self.initialized
	}
	pub fn initialize(&mut self) {
		self.initialized = true;
	}
}

pub struct MethodArea {
	debug_level: DebugLevel,
	classes: HashMap<String, Arc<Mutex<LoadedClass>>>,
}

impl MethodArea {
	pub fn new(debug_level: DebugLevel) -> Self {
		MethodArea {
			debug_level: debug_level,
			classes: HashMap::new(),
		}
	}

	pub fn is_class_loaded(&self, class_name: &String) -> bool {
		self.classes.contains_key(class_name)
	}

	pub fn get_class_rc(&self, class_name: &String) -> Option<Rc<Class>> {
		if let Some(loaded_class) = self.classes.get(class_name) {
			let mut result: Option<Rc<Class>> = None;
			if let Ok(loaded_class) = loaded_class.lock() {
				result = Some(Rc::clone(&(loaded_class.class)));
			}
			result
		} else {
			None
		}
	}

	pub fn get_loaded_class(&self, class_name: &String) -> Option<Arc<Mutex<LoadedClass>>> {
		if let Some(loaded_class) = self.classes.get(class_name) {
			Some(Arc::clone(loaded_class))
		} else {
			None
		}
	}

	/*
	 * TODO: We cannot test this (really) until we implement
	 * invoke virtual.
	 */
	pub fn resolve_method(
		&mut self,
		invoking_class: &Rc<Class>,
		invoked_class: &Rc<Class>,
		method_name: &String,
		method_type: &String,
	) -> Option<Rc<Class>> {
		let mut target_class = invoked_class;
		let mut result: Option<Rc<Class>> = None;

		/*
		 * TODO: Check whether class is an interface. This is an
		 * IncompatibleClassChangeError.
		 */

		while {
			/*
			 * TODO:
			 * If [target_class] declares exactly one method with the name
			 * specified by the method reference, and the declaration
			 * is a signature polymorphic method (§2.9), then method
			 * lookup succeeds. All the class names mentioned in the
			 * descriptor are resolved (§5.4.3.1).
			 */

			/*
			 * ... if [class] declares a method with the name and
			 * descriptor specified by the method reference, method
			 * lookup succeeds.
			 */
			if let Some(_) = target_class.get_methods_ref().get_by_name_and_type(
				method_name,
				method_type,
				target_class.get_constant_pool_ref(),
			) {
				Debug(
					format!(
						"Method {} resolved to {}.",
						method_name,
						target_class.get_class_name().unwrap()
					),
					&self.debug_level,
					DebugLevel::Info,
				);

				result = Some(Rc::clone(&target_class));
				false /* this will break the loop */
			} else {
				/*
				 * ...  if [target_class] has a superclass, step 2 of method
				 * lookup is recursively invoked on the direct superclass
				 * of [class].
				 */
				assert!(false, "Must look in the super class");
				target_class = target_class;
				true
			}
		} {}

		/*
		 * If we didn't find anything there, then let's look in
		 * the superinterfaces.
		 */
		match result {
			None => assert!(false, "TODO: Look in the superinterfaces."),
			_ => {}
		}

		/*
		 * TODO: Check loading constraints!
		 */

		result
	}

	pub fn load_class_from_file(&mut self, class_filename: &String) -> Option<Rc<Class>> {
		if let Some(class) = Class::load(class_filename) {
			if let Some(class_name) = class.get_class_name() {
				if let Some(_) = self.classes.insert(
					class_name.to_string(),
					Arc::new(Mutex::new(LoadedClass::new(class))),
				) {
					/*
					 * This is a fatal error -- loading the same class twice!
					 */
				}
				/*
				 * loaded_class is an Arc
				 */
				let loaded_class = self.classes.get(&class_name).unwrap();
				let mut result: Option<Rc<Class>> = None;
				if let Ok(loaded_class) = loaded_class.lock() {
					result = Some(Rc::clone(&loaded_class.class));
				}
				return result;
			}
		}
		None
	}
}
