pub struct Environment<'a> {
	pub classpath: &'a [String],
	pub args: &'a [String],
}

impl<'a> Environment<'a> {
	pub fn new(cp: &'a [String], args: &'a [String]) -> Self {
		Environment {
			classpath: cp,
			args: args,
		}
	}
}
