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

use enum_primitive::FromPrimitive;
use jvm::constant::Constant;
use jvm::constant::Utf8Reserved;
use std::iter::repeat;
use std::str;

enum_from_primitive! {
pub enum ConstantTag {
	Class = 7,
	Fieldref = 9,
	Methodref = 10,
	InterfaceMethodref = 11,
	String = 8,
	Integer= 3,
	Float= 4,
	Long= 5,
	Double = 6,
	NameAndType = 12,
	Utf8 = 1,
	MethodHandle= 15,
	MethodType = 16,
	InvokeDynamic = 18,
	Module = 19,
	Package = 20,
}}

#[derive(Clone, Default)]
pub struct ConstantPool {
	byte_len: usize,
	constants: Vec<Constant>,
}

impl ConstantPool {
	pub fn set(&mut self, index: usize, constant: Constant) {
		self.constants[index] = constant;
	}

	pub fn get_constant_clone(&self, index: usize) -> Constant {
		self.constants[index].clone()
	}
	pub fn get_constant_ref(&self, index: usize) -> &Constant {
		&self.constants[index]
	}

	pub fn byte_len(&self) -> usize {
		self.byte_len
	}

	pub fn constant_pool_count(&self) -> u16 {
		self.constants.len() as u16
	}
}

impl<'l> From<&'l Vec<u8>> for ConstantPool {
	fn from(bytes: &Vec<u8>) -> Self {
		let mut offset = 0;
		let mut constants: Vec<Constant>;
		let mut skip = false;
		let constants_pool_count: u16 =
			(bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16) << 0;
		offset += 2;

		constants = repeat(Constant::Default())
			.take(constants_pool_count as usize)
			.collect();

		for i in 1..constants_pool_count as usize {
			if skip {
				skip = false;
				continue;
			}

			match ConstantTag::from_u8(bytes[offset]) {
				Some(ConstantTag::Class) => {
					let tag: u8 = bytes[offset];
					let name_index: u16 =
						(bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);
					offset += 3;
					constants[i] = Constant::Class(tag, name_index);
				}
				Some(ConstantTag::Fieldref) => {
					let tag: u8 = bytes[offset];
					let index: u16 = (bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);
					let name_and_type_index: u16 =
						(bytes[offset + 3] as u16) << 8 | (bytes[offset + 4] as u16);
					offset += 5;
					constants[i] = Constant::Fieldref(tag, index, name_and_type_index);
				}
				Some(ConstantTag::Methodref) => {
					let tag: u8 = bytes[offset];
					let index: u16 = (bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);
					let name_and_type_index: u16 =
						(bytes[offset + 3] as u16) << 8 | (bytes[offset + 4] as u16);
					offset += 5;
					constants[i] = Constant::Methodref(tag, index, name_and_type_index);
				}
				Some(ConstantTag::InterfaceMethodref) => {
					let tag: u8 = bytes[offset];
					let class_index: u16 =
						(bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);
					let name_and_type_index: u16 =
						(bytes[offset + 3] as u16) << 8 | (bytes[offset + 4] as u16);
					offset += 5;
					constants[i] =
						Constant::InterfaceMethodref(tag, class_index, name_and_type_index);
				}
				Some(ConstantTag::String) => {
					let tag: u8 = bytes[offset];
					let string_index: u16 =
						(bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);
					offset += 3;
					constants[i] = Constant::String(tag, string_index);
				}
				Some(ConstantTag::Integer) => {
					let tag: u8 = bytes[offset];
					let bytes: u32 = (bytes[offset + 1] as u32) << 24
						| (bytes[offset + 2] as u32) << 16
						| (bytes[offset + 3] as u32) << 8
						| (bytes[offset + 4] as u32) << 0;
					offset += 5;
					constants[i] = Constant::Integer(tag, bytes);
				}
				Some(ConstantTag::Float) => {
					let tag: u8 = bytes[offset];
					let bytes: u32 = (bytes[offset + 1] as u32) << 24
						| (bytes[offset + 2] as u32) << 16
						| (bytes[offset + 3] as u32) << 8
						| (bytes[offset + 4] as u32) << 0;
					offset += 5;
					// TODO: Parse a constant float from bytes into value
					// https://docs.oracle.com/javase/specs/jvms/se13/html/jvms-4.html#jvms-4.4.4
					constants[i] = Constant::Float(tag, bytes);
				}
				Some(ConstantTag::Long) => {
					let tag: u8 = bytes[offset];
					let bytes: u64 = (bytes[offset + 1] as u64) << 56
						| (bytes[offset + 2] as u64) << 48
						| (bytes[offset + 3] as u64) << 40
						| (bytes[offset + 4] as u64) << 32
						| (bytes[offset + 5] as u64) << 24
						| (bytes[offset + 6] as u64) << 16
						| (bytes[offset + 7] as u64) << 8
						| (bytes[offset + 8] as u64) << 0;
					offset += 9;
					/*
					 * From https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5
					 * "... then the next usable item in the pool is located at index n+2"
					 */
					skip = true;
				}
				Some(ConstantTag::Double) => {
					let tag: u8 = bytes[offset];
					let bytes: u64 = (bytes[offset + 1] as u64) << 56
						| (bytes[offset + 2] as u64) << 48
						| (bytes[offset + 3] as u64) << 40
						| (bytes[offset + 4] as u64) << 32
						| (bytes[offset + 5] as u64) << 24
						| (bytes[offset + 6] as u64) << 16
						| (bytes[offset + 7] as u64) << 8
						| (bytes[offset + 8] as u64) << 0;
					offset += 9;
					// TODO: Parse a constant double from bytes into value
					// https://docs.oracle.com/javase/specs/jvms/se13/html/jvms-4.html#jvms-4.4.5
					constants[i] = Constant::Long(tag, bytes);
					constants[i] = Constant::Double(tag, bytes);
					/*
					 * From https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.5
					 * "... then the next usable item in the pool is located at index n+2"
					 */
					skip = true;
				}
				Some(ConstantTag::NameAndType) => {
					let tag: u8 = bytes[offset];
					let name_index: u16 =
						(bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);
					let descriptor_index: u16 =
						(bytes[offset + 3] as u16) << 8 | (bytes[offset + 4] as u16);
					offset += 5;
					constants[i] = Constant::NameAndType(tag, name_index, descriptor_index);
				}
				Some(ConstantTag::Utf8) => {
					let mut reserved: Utf8Reserved = Utf8Reserved::NotReserved;
					let tag: u8 = bytes[offset];
					let length: u16 = (bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);
					let value_range = offset + 3..offset + 3 + (length as usize);
					let mut value: &str = "";

					match str::from_utf8(&bytes[value_range]) {
						Ok(v) => {
							value = v;
						}
						Err(_) => {
							//assert!(false, "TODO: Parse a modified UTF8 string");
						}
					}

					/*
					 * Handle "Six attributes are critical to correct interpretation
					 * of the class file by the Java Virtual Machine" and give them
					 * a special reserved status so that it is easier to check later.
					 */
					if value == "Code".to_string() {
						reserved = Utf8Reserved::Code;
					} else if value == "StackMapTable".to_string() {
						reserved = Utf8Reserved::StackMapTable;
					} else if value == "ConstantValue".to_string() {
						reserved = Utf8Reserved::ConstantValue;
					}

					offset += 1 + 2 + (length as usize);
					constants[i] = Constant::Utf8(tag, reserved, length, value.to_string());
				}
				Some(ConstantTag::MethodHandle) => {
					let tag: u8 = bytes[offset];
					let reference_kind: u8 = bytes[offset + 1];
					let reference_index: u16 =
						(bytes[offset + 2] as u16) << 8 | (bytes[offset + 3] as u16);
					constants[i] = Constant::MethodHandle(tag, reference_kind, reference_index);
					offset += 4;
				}
				Some(ConstantTag::MethodType) => {
					let tag: u8 = bytes[offset];
					let descriptor_index: u16 =
						(bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);

					constants[i] = Constant::MethodType(tag, descriptor_index);
					offset += 3;
				}
				Some(ConstantTag::InvokeDynamic) => {
					let tag: u8 = bytes[offset];
					let bootstrap_method_attr_index: u16 =
						(bytes[offset + 1] as u16) << 8 | (bytes[offset + 2] as u16);
					let name_and_type_index: u16 =
						(bytes[offset + 3] as u16) << 8 | (bytes[offset + 4] as u16);
					constants[i] = Constant::InvokeDynamic(
						tag,
						bootstrap_method_attr_index,
						name_and_type_index,
					);
					offset += 5;
				}
				Some(ConstantTag::Module) => {
					assert!(false, "TODO: Parse a module");
				}
				Some(ConstantTag::Package) => {
					assert!(false, "TODO: Parse a package");
				}
				_ => {
					print!("oops: unhandled constant pool tag.\n");
				}
			};
		}
		ConstantPool {
			byte_len: offset,
			constants: constants,
		}
	}
}
