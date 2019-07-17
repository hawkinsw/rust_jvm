#![allow(non_camel_case_types)]

enum_from_primitive! {
	pub enum OperandCodes {
		OPCODE_iconst_m1 = 0x2,
		OPCODE_iconst_0 = 0x3,
		OPCODE_iconst_1 = 0x4,
		OPCODE_iconst_2 = 0x5,
		OPCODE_iconst_3 = 0x6,
		OPCODE_iconst_4 = 0x7,
		OPCODE_iconst_5 = 0x8,
		OPCODE_ireturn = 0xac,
		OPCODE_invokestatic = 0xb8,
		OPCODE_pop = 0x57,
	}
}
