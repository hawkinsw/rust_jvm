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
#[macro_use]
extern crate enum_primitive;
extern crate clap;
use clap::{App, Arg};
mod jvm;
use jvm::jvmthread::JvmThread;

fn main() {
	let mut debug: bool = false;
	/*
	 * TODO: Update this so that we use a yaml file to
	 * generate these options!
	 */
	let cli_matches = App::new("Java Virtual Machine")
		.version("1.0.0")
		.arg(
			Arg::with_name("file")
				.help("Class file to load.")
				.required(true)
				.index(1),
		)
		.arg(Arg::with_name("method").help("Method to execute at start."))
		.arg(Arg::with_name("debug").help("Class to execute.").short("d"))
		.get_matches();

	let file_name = format!("{}.class", cli_matches.value_of("file").unwrap());
	let method = cli_matches.value_of("method").unwrap_or("main").to_string();
	if cli_matches.is_present("debug") {
		debug = true;
	}

	if debug {
		print!("Opening {}\n", file_name);
	}

	if let Some(jvm) = jvm::Jvm::new(debug) {
		jvm.run(&file_name, &method, &["testing".to_string()]);
	}
}
