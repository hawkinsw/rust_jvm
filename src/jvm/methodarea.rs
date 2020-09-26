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
use jvm::class::ClassInitializationStatus;
use jvm::classpath::ClassLocation;
use jvm::classpath::ClassPath;
use jvm::debug::Debug;
use jvm::debug::DebugLevel;
use jvm::environment::Environment;
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use jvm::method::Method;
use jvm::method::MethodAccessFlags;
use jvm::typevalues::JvmValue;
use rjar::Jar;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::LockResult;
use std::sync::Mutex;
use std::sync::MutexGuard;

/// A LoadedClass holds *the* first reference to
/// `class` and can be used to determine whether the
/// `class` has been initialized.
pub struct LoadedClass {
	/// Whether or not `class` is initialized.
	/// This is known as LC in the spec.
	pub lc: Mutex<ClassInitializationStatus>,
	pub lc_waitq: Condvar,

	/// The base reference to class.
	pub class: Rc<Class>,
}

impl LoadedClass {
	fn new(class: Class) -> Self {
		LoadedClass {
			class: Rc::new(class),
			lc: Mutex::new(ClassInitializationStatus::VerifiedPreparedNotInitialized),
			lc_waitq: Condvar::new(),
		}
	}
}

pub struct MethodArea {
	debug_level: DebugLevel,
	environment: Environment,
	classes: HashMap<String, Arc<LoadedClass>>,
}

impl MethodArea {
	pub fn new(debug_level: DebugLevel, environment: Environment) -> Self {
		let mut result = Self {
			debug_level,
			environment: environment,
			classes: HashMap::new(),
		};
		result
	}

	/// If the class named `class_name` is loaded into the method area,
	/// this function will increase its reference count by one and move
	/// that reference count to the caller.
	/// This must be called with the methodarea locked.
	/// # Arguments
	/// `class_name`: The name of the class to which caller wants a reference.
	/// # Return value:
	/// Optionally, a reference to the class named `class_name`. None if
	/// the class is not loaded into the methodarea.
	pub fn get_class_rc(&self, class_name: &String) -> Option<Rc<Class>> {
		if let Some(loaded_class) = self.classes.get(class_name) {
			Some(Rc::clone(&(loaded_class.class)))
		} else {
			None
		}
	}

	///
	/// Must call this with the methodarea locked.
	///
	pub fn maybe_load_class(&mut self, class_name: &String) {
		if let None = self.classes.get(class_name) {
			match self.environment.class_location_for_class(class_name) {
				Some(ClassLocation::ClassFile(location)) => {
					self.load_class_from_file(&location);
				}
				Some(ClassLocation::JarFile(jarfile, location)) => {
					if let Ok(mut jar) = Jar::open(&jarfile) {
						if let Ok(bytes) = jar.file_contents_by_name(&location) {
							self.load_class_from_bytes(bytes);
						}
					}
				}
				None => {
					println!("error: no path to {}", class_name);
				}
			}
		}
	}

	pub fn get_loaded_class(&mut self, class_name: &String) -> Option<Arc<LoadedClass>> {
		if let Some(loaded_class) = self.classes.get(class_name) {
			Some(Arc::clone(loaded_class))
		} else {
			FatalError::new(FatalErrorType::ClassNotFound(class_name.to_string())).call();
			None
		}
	}

	pub fn select_method(
		&mut self,
		invoked_class: &Rc<Class>,
		method_name: &String,
		method_type: &String,
	) -> Option<(Rc<Class>, Rc<Method>)> {
		let mut target_class = Rc::clone(invoked_class);
		let mut result: Option<(Rc<Class>, Rc<Method>)> = None;

		while {
			if let Some(target_method) = target_class.get_methods_ref().get_by_name_and_type(
				method_name,
				method_type,
				target_class.get_constant_pool_ref(),
			) {
				if (target_method.access_flags & (MethodAccessFlags::Private as u16) == 0)
					&& ((target_method.access_flags & (MethodAccessFlags::Public as u16) != 0)
						|| (target_method.access_flags & (MethodAccessFlags::Protected as u16)
							!= 0) || false/* TODO: mA is marked neither ACC_PUBLIC nor ACC_PROTECTED nor ACC_PRIVATE, and either (a) the declaration of mA appears in the same run-time package as the declaration of mC, or (b) if mA is declared in a class A and mC is declared in a class C, then there exists a method mB declared in a class B such that C is a subclass of B and B is a subclass of A and mC can override mB and mB can override mA. */)
				{
					Debug(
						format!(
							"Method {} selected to {}.",
							method_name,
							target_class.get_class_name().unwrap()
						),
						&self.debug_level,
						DebugLevel::Info,
					);
					result = Some((target_class.clone(), target_method));
				}
			}

			/*
			 * If we did not find a result (either because it doesn't exist or
			 * because it did not qualify as an override), then we are going
			 * to set ourselves up to look in the superclass.
			 */
			match result {
				None => {
					let mut traversed_to_superclass = false;
					if let Some(superclass_name) = target_class.resolve_superclass() {
						if let Some(superclass) = self.get_class_rc(&superclass_name) {
							target_class = superclass;
							traversed_to_superclass = true;
						}
					} else {
						/*
						 * TODO: We walked to the top of the class hierarchy
						 * and couldn't find a method. This will not happen because
						 * we already resolved a method that the compiler guaranteed
						 * is somewhere in the hierarchy and we are now just looking
						 * for something that overrides it. So, it is a fatal error.
						 */
					}
					traversed_to_superclass
				}
				_ => false,
			}
		} {}

		/*
		 * If we didn't find anything there, then let's look in
		 * the superinterfaces:
		 * Otherwise, the maximally-specific superinterface methods of C are determined (§5.4.3.3). If exactly one matches mR's name and descriptor and is not abstract, then it is the selected method.
		 */
		match result {
			None => assert!(false, "TODO: Look in the superinterfaces."),
			_ => {}
		}
		result
	}

	// Return the name of the class that contains the referenced field.
	// field_class: The class where the reference originally occured.
	// field_name: The name of the field.
	// field_type: The type of the field.
	pub fn resolve_field(
		&mut self,
		field_class: &Rc<Class>,
		field_name: &String,
		field_type: &String,
	) -> Option<String> {
		let mut target_class = field_class;
		while {
			if target_class
				.get_fields_ref()
				.contains_field_with_name_and_type(
					field_name,
					field_type,
					target_class.get_constant_pool_ref(),
				) {
				return target_class.get_class_name();
			} else {
				assert!(false, "Must look in the super class for a field.");
				target_class = target_class;
				true // go again
			}
		} {}
		None
	}

	pub fn resolve_method(
		&mut self,
		invoking_class: &Rc<Class>,
		invoked_class: &Rc<Class>,
		method_name: &String,
		method_type: &String,
	) -> Option<Rc<Method>> {
		let mut target_class = invoked_class;
		let mut result: Option<Rc<Method>> = None;

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
			if let Some(target_method) = target_class.get_methods_ref().get_by_name_and_type(
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

				result = Some(target_method);
				false /* this will break the loop */
			} else {
				/*
				 * ...  if [target_class] has a superclass, step 2 of method
				 * lookup is recursively invoked on the direct superclass
				 * of [class].
				 */
				assert!(false, "Must look in the super class for a method");
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
	pub fn load_class_from_bytes(&mut self, class_bytes: Vec<u8>) -> Option<Rc<Class>> {
		if let Some(class) = Class::load_from_bytes(class_bytes) {
			if let Some(class_name) = class.get_class_name() {
				if let Some(_) = self
					.classes
					.insert(class_name.to_string(), Arc::new(LoadedClass::new(class)))
				{
					/*
					 * This is a fatal error -- loading the same class twice!
					 */
				}
				/*
				 * loaded_class is an Arc
				 */
				let loaded_class = self.classes.get(&class_name).unwrap();
				return Some(Rc::clone(&loaded_class.class));
			}
		}
		None
	}

	pub fn load_class_from_file(&mut self, class_filename: &String) -> Option<Rc<Class>> {
		if let Some(class) = Class::load_from_file(class_filename) {
			if let Some(class_name) = class.get_class_name() {
				println!("load_class_from_file: {}", class_name);
				if let Some(_) = self
					.classes
					.insert(class_name.to_string(), Arc::new(LoadedClass::new(class)))
				{
					/*
					 * This is a fatal error -- loading the same class twice!
					 */
				}
				/*
				 * loaded_class is an Arc
				 */
				let loaded_class = self.classes.get(&class_name).unwrap();
				return Some(Rc::clone(&loaded_class.class));
			}
		}
		None
	}
}
