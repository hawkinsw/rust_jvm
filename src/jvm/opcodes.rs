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
	pub enum OperandCode {
		Iconst_m1 = 0x2,
		Iconst_0 = 0x3,
		Iconst_1 = 0x4,
		Iconst_2 = 0x5,
		Iconst_3 = 0x6,
		Iconst_4 = 0x7,
		Iconst_5 = 0x8,
		Bipush = 0x10,
		Ldc = 0x12,
		Iload_0 = 0x1a,
		Iload_1 = 0x1b,
		Iload_2 = 0x1c,
		Iload_3 = 0x1d,
		Aload_0 = 0x2a,
		Aload_1 = 0x2b,
		Aload_2 = 0x2c,
		Aload_3 = 0x2d,
		Istore_0 = 0x3b,
		Istore_1 = 0x3c,
		Istore_2 = 0x3d,
		Istore_3 = 0x3e,
		Astore_0 = 0x4b,
		Astore_1 = 0x4c,
		Astore_2 = 0x4d,
		Astore_3 = 0x4e,
		Pop = 0x57,
		Dup = 0x59,
		Iadd = 0x60,
		Imul = 0x68,
		If_icmpeq = 0x9f,
		If_icmpne = 0xa0,
		If_icmplt = 0xa1,
		If_icmpge = 0xa2,
		If_icmpgt = 0xa3,
		If_icmple = 0xa4,
		Goto = 0xa7,
		Ireturn = 0xac,
		r#Return = 0xb1,
		GetStatic = 0xb2,
		Invokevirtual = 0xb6,
		Invokespecial = 0xb7,
		Invokestatic = 0xb8,
		New = 0xbb,
	}
}
