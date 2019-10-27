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
mod attribute;
mod class;
mod classpath;
mod constant;
mod constantpool;
pub mod debug;
mod environment;
mod error;
mod exceptions;
mod field;
mod frame;
mod jvmthread;
mod method;
mod methodarea;
mod object;
mod opcodes;
mod typevalues;

use jvm::debug::Debug;
use jvm::debug::DebugLevel;
use jvm::methodarea::MethodArea;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Jvm {
	debug_level: DebugLevel,
}

impl Jvm {
	pub fn new(debug_level: DebugLevel) -> Option<Jvm> {
		Some(Jvm {
			debug_level: debug_level,
		})
	}

	pub fn run(
		&self,
		start_class: &String,
		start_function: &String,
		classpath: &[&str],
		args: &[&str],
	) -> bool {
		/*
		 * Create a VM and start running!
		 */
		let env = environment::Environment::new(classpath, args, self.debug_level.clone());
		let methodarea = Arc::new(Mutex::new(MethodArea::new(self.debug_level.clone(), env)));
		let mut thread = jvmthread::JvmThread::new(self.debug_level.clone(), methodarea);
		if thread.run(start_class, start_function) {
			Debug(
				format!("Success running {}.{}", start_class, start_function),
				&self.debug_level,
				DebugLevel::Info,
			);
			return true;
		}
		eprintln!("Failure running {}.{}", start_class, start_function);
		false
	}
}

impl fmt::Display for Jvm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "debug_level: {}\n", &self.debug_level)
	}
}
