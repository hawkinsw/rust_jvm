#[macro_use] extern crate enum_primitive;
mod jvm;

fn main() {
	let jvm = jvm::Jvm::new("java_test/Hello.class");
	print!("{}\n", jvm.class());
}
