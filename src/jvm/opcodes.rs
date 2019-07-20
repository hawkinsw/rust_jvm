/*
 * FILE: XXXXX
 * DESCRIPTION: 
 *
 * Copyright (c) 2019, Will Hawkins
 *
 * This file is part of Rust-JVM.
 *
 * Rust-JVM is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rust-JVM is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Rust-JVM.  If not, see <https://www.gnu.org/licenses/>.
 */
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
		OPCODE_iload_0 = 0x1a,
		OPCODE_iload_1 = 0x1b,
		OPCODE_iload_2 = 0x1c,
		OPCODE_iload_3 = 0x1d,
		OPCODE_ireturn = 0xac,
		OPCODE_invokestatic = 0xb8,
		OPCODE_pop = 0x57,
		OPCODE_iadd = 0x60,
	}
}
