#[macro_use] extern crate enum_primitive;
extern crate clap;
use clap::{Arg,App};
mod jvm;
use jvm::vm::Vm;

fn main() {
	let mut debug: bool = false;
	/*
	 * TODO: Update this so that we use a yaml file to
	 * generate these options!
	 */
	let cli_matches = App::new("Java Virtual Machine")
	                      .version("1.0.0")
	                      .arg(Arg::with_name("file")
												         .help("Class file to load.")
																 .required(true)
																 .index(1))
	                      .arg(Arg::with_name("class")
												         .help("Entry point class.")
																 .required(true)
																 .index(2))
	                      .arg(Arg::with_name("method")
												         .help("Method to execute at start."))
	                      .arg(Arg::with_name("debug")
												         .help("Class to execute.")
																 .short("d")).get_matches();

	let file_name = format!("{}.class", cli_matches.value_of("file").unwrap());
	let class_name = cli_matches.value_of("class").unwrap().to_string();
	let method = cli_matches.value_of("method").unwrap_or("main").to_string();
	if cli_matches.is_present("debug") {
		debug = true;
	}

	if debug {
		print!("Opening {}\n", file_name);
	}

	if let Some(jvm) = jvm::Jvm::new(debug) {
		jvm.run(&file_name, &file_name, &method);
	}
}
