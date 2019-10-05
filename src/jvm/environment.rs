#[derive(Clone)]
pub struct Environment {
	pub classpath: Vec<String>,
	pub arguments: Vec<String>,
}

impl Environment {
	pub fn new(cp: &[&str], args: &[&str]) -> Self {
		let mut classpath = Vec::<String>::new();
		let mut arguments = Vec::<String>::new();

		for dir in cp {
			classpath.push((**dir).to_string());
		}
		for arg in args {
			arguments.push((**arg).to_string());
		}

		Environment {
			classpath,
			arguments,
		}
	}
}
