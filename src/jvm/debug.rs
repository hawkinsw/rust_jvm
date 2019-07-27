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
use std::fmt::Display;

#[derive(PartialOrd, PartialEq, Clone)]
pub enum DebugLevel {
	Verbose,
	Info,
	Warning,
	Error,
}

impl fmt::Display for DebugLevel {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			DebugLevel::Verbose => write!(f, "Verbose"),
			DebugLevel::Info => write!(f, "Info"),
			DebugLevel::Warning => write!(f, "Warning"),
			DebugLevel::Error => write!(f, "Error"),
		}
	}
}

pub fn Debug<I: Display>(message: I, output_level: &DebugLevel, filter_level: DebugLevel) {
	if *output_level >= filter_level {
		eprintln!("{}", message);
	}
}
