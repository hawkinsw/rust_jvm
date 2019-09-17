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
use std::fmt;

#[derive(Clone)]
pub enum Utf8Reserved {
	Code,
	ConstantValue,
	StackMapTable,
	BoostrapMethods,
	NestHost,
	NestMembers,
	NotReserved,
}

impl fmt::Display for Utf8Reserved {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Utf8Reserved::NotReserved => write!(f, "NotReserved"),
			Utf8Reserved::Code => write!(f, "Code"),
			Utf8Reserved::StackMapTable => write!(f, "StackMapTable"),
			Utf8Reserved::ConstantValue => write!(f, "ConstantValue"),
			_ => write!(f, "Unknown"),
		}
	}
}

#[derive(Clone)]
pub enum Constant {
	Class(u8, u16),
	Fieldref(u8, u16, u16),
	Methodref(u8, u16, u16),
	InterfaceMethodref(u8, u16, u16),
	String(u8, u16),
	Integer(u8, u32),
	Float(),
	Long(),
	Double(u8, u64),
	NameAndType(u8, u16, u16),
	Utf8(u8, Utf8Reserved, u16, String),
	MethodHandle(),
	MethodType(),
	InvokeDynamic(),
	Module(),
	Package(),
	Default(),
}

impl fmt::Display for Constant {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Constant::Methodref(tag, index, name_and_type_index) => write!(
				f,
				"Methodref: tag: {}, index {}, name_and_type_index: {}",
				tag, index, name_and_type_index
			),
			Constant::InterfaceMethodref(tag, class_index, name_and_type_index) => write!(
				f,
				"InterfaceMethodref: tag: {}, class_index {}, name_and_type_index: {}",
				tag, class_index, name_and_type_index
			),
			Constant::Fieldref(tag, index, name_and_type_index) => write!(
				f,
				"Fieldref: tag:{}, index:{}, name_and_type_index:{}",
				tag, index, name_and_type_index
			),
			Constant::Class(tag, name_index) => {
				write!(f, "Class: tag: {}, name_index {}", tag, name_index)
			}
			Constant::String(tag, string_index) => {
				write!(f, "String: tag: {}, string_index {}", tag, string_index)
			}
			Constant::NameAndType(tag, name_index, descriptor_index) => write!(
				f,
				"NameAndType: tag: {}, name_index: {}, descriptor_index: {}",
				tag, name_index, descriptor_index
			),
			Constant::Utf8(tag, reserved, length, value) => write!(
				f,
				"Utf8: tag: {}, reserved: {}, length: {}, value: {}",
				tag, reserved, length, value
			),
			Constant::Integer(tag, value) => write!(f, "Integer: tag: {}, value: {}", tag, value),
			Constant::Double(tag, value) => {
				write!(f, "Double: tag: {}, value: 0x{:x} (ieee754)", tag, value)
			}
			_ => write!(f, "Unknown"),
		}
	}
}
