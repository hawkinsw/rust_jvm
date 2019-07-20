pub struct Environment<'a> {
	pub classpath: &'a [&'a str],
	pub args: &'a [&'a str],
}

impl<'a> Environment<'a> {
	pub fn new(cp: &'a [&'a str], args: &'a [&'a str]) -> Self {
		Environment {
			classpath: cp,
			args: args,
		}
	}
}
