use enum_primitive::FromPrimitive;

enum_from_primitive! {
	pub enum OperandCodes {
		OPCODE_invokestatic = 0xb8,
		OPCODE_pop = 0x57,
	}
}
