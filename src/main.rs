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
extern crate clap;
extern crate enum_primitive;
extern crate jvm;
use clap::{App, Arg};

use jvm::jvm::debug::DebugLevel;

fn main() {
	let mut debug = DebugLevel::Error;
	/*
	 * TODO: Update this so that we use a yaml file to
	 * generate these options!
	 */
	let cli_matches = App::new("Java Virtual Machine")
		.version("1.0.0")
		.arg(
			Arg::with_name("class")
				.help("Main class.")
				.required(true)
				.index(1),
		)
		.arg(Arg::with_name("method").help("Main method to execute."))
		.arg(
			Arg::with_name("debug")
				.help("Enable debugging output.")
				.short("d"),
		)
		.arg(
			Arg::with_name("classpath")
				.help("Class path.")
				.short("c")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("args")
				.help("Java application arguments.")
				.short("a")
				.takes_value(true)
				.multiple(true),
		)
		.get_matches();

	if cli_matches.is_present("debug") {
		debug = DebugLevel::Info;
	}

	let class = format!("{}", cli_matches.value_of("class").unwrap());
	let method = cli_matches.value_of("method").unwrap_or("main").to_string();

	let classpath_arg = cli_matches.value_of("classpath").unwrap_or("");
	let classpath: Vec<&str> = classpath_arg.split(":").collect();

	let args: Vec<&str> = cli_matches
		.values_of("args")
		.unwrap_or(clap::Values::default())
		.collect();

	if let Some(jvm) = jvm::jvm::Jvm::new(debug) {
		jvm.run(&class, &method, &classpath, &args);
	}
}
