#[macro_use] extern crate enum_primitive;
extern crate clap;
use clap::{Arg,App, SubCommand};
mod jvm;
use jvm::vm::Vm;

fn main() {
	let mut debug: bool = false;
	let cli_matches = App::new("Java Virtual Machine")
	                      .version("1.0.0")
	                      .arg(Arg::with_name("class")
												         .help("Class to execute.")
																 .required(true)
																 .index(1))
	                      .arg(Arg::with_name("debug")
												         .help("Class to execute.")
																 .short("d")).get_matches();

	let class_name = format!("{}.class", cli_matches.value_of("class").unwrap());
	if cli_matches.is_present("debug") {
		debug = true;
	}

	if debug {
		print!("Opening {}\n", class_name);
	}

	if let Some(jvm) = jvm::Jvm::new(&class_name, debug) {
		if debug {
			print!("{}\n", jvm.class());
		}
		let mut vm = Vm::new(&mut jvm.class());
		vm.execute_main();
	}
}
