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
		Aconst_null = 0x1,
		Iconst_m1 = 0x2,
		Iconst_0 = 0x3,
		Iconst_1 = 0x4,
		Iconst_2 = 0x5,
		Iconst_3 = 0x6,
		Iconst_4 = 0x7,
		Iconst_5 = 0x8,
		Fconst_0 = 0xB,
		Fconst_1 = 0xC,
		Bipush = 0x10,
		Ldc = 0x12,
		Iload_0 = 0x1a,
		Iload_1 = 0x1b,
		Iload_2 = 0x1c,
		Iload_3 = 0x1d,
		Fload_0 = 0x22,
		Fload_1 = 0x23,
		Fload_2 = 0x24,
		Fload_3 = 0x25,
		Aload_0 = 0x2a,
		Aload_1 = 0x2b,
		Aload_2 = 0x2c,
		Aload_3 = 0x2d,
		AaLoad = 0x32,
		CaLoad = 0x34,
		Fstore = 0x38,
		Istore_0 = 0x3b,
		Istore_1 = 0x3c,
		Istore_2 = 0x3d,
		Istore_3 = 0x3e,
		Fstore_0 = 0x43,
		Fstore_1 = 0x44,
		Fstore_2 = 0x45,
		Fstore_3 = 0x46,
		Astore_0 = 0x4b,
		Astore_1 = 0x4c,
		Astore_2 = 0x4d,
		Astore_3 = 0x4e,
		AaStore = 0x53,
		CaStore = 0x55,
		Pop = 0x57,
		Dup = 0x59,
		Iadd = 0x60,
		Fadd = 0x62,
		Fsub = 0x66,
		Imul = 0x68,
		Fmul = 0x6a,
		Fdiv = 0x6e,
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
		PutStatic = 0xb3,
		GetField = 0xb4,
		PutField = 0xb5,
		Invokevirtual = 0xb6,
		Invokespecial = 0xb7,
		Invokestatic = 0xb8,
		New = 0xbb,
		NewArray = 0xbc,
		ANewArray = 0xbd,
		ArrayLength = 0xbe,
		Fcmplt = 0x95,
		Fcmpgt = 0x96,
		Ifeq = 0x99,
		Ifne = 0x9a,
		Iflt = 0x9b,
		Ifge = 0x9c,
		Ifgt = 0x9d,
		Ifle = 0x9e,
	}
}
