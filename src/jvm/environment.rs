use jvm::classpath::ClassLocation;
use jvm::classpath::ClassPath;
use jvm::debug::DebugLevel;

pub struct Environment {
	pub classpath: ClassPath,
	pub arguments: Vec<String>,
}

impl Environment {
	pub fn new(cp: &[&str], args: &[&str], debug_level: DebugLevel) -> Self {
		let classpath = ClassPath::new(cp, debug_level);
		let mut arguments = Vec::<String>::new();

		for arg in args {
			arguments.push((**arg).to_string());
		}

		Environment {
			classpath,
			arguments,
		}
	}

	pub fn class_location_for_class(&self, class: &str) -> Option<ClassLocation> {
		self.classpath.class_location_for_class(class)
	}
}
