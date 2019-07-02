use jvm::frame::Frame;

#[derive(Clone,Default)]
pub struct Stack {
	frames: Vec<Frame>,
}

impl Stack {
	pub fn new() -> Self {
		Stack{frames: Vec::<Frame>::new()}
	}
}
