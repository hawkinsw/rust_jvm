use enum_primitive::FromPrimitive;
use jvm::constant::Constant;
use jvm::constant::Utf8Reserved;
use std::iter::repeat;
use std::str;

enum_from_primitive! {
pub enum ConstantTags {
	CONSTANT_Class = 7,
	CONSTANT_Fieldref = 9,
	CONSTANT_Methodref = 10,
	CONSTANT_InterfaceMethodref = 11,
	CONSTANT_String = 8,
	CONSTANT_Integer= 3,
	CONSTANT_Float= 4,
	CONSTANT_Long= 5,
	CONSTANT_Double = 6,
	CONSTANT_NameAndType = 12,
	CONSTANT_Utf8 = 1,
	CONSTANT_MethodHandle= 15,
	CONSTANT_MethodType = 16,
	CONSTANT_InvokeDynamic = 18,
	CONSTANT_Module = 19,
	CONSTANT_Package = 20,
}}

#[derive(Clone,Default)]
pub struct ConstantPool {
	byte_len: usize,
	constants: Vec<Constant>,
}

impl ConstantPool {
	pub fn set(&mut self, index: usize, constant: Constant) {
		self.constants[index] = constant;
	}

	pub fn get(&self, index: usize) -> Constant {
		self.constants[index].clone()
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
		let constants_pool_count: u16 = (bytes[offset + 0] as u16) << 8 |
		                                (bytes[offset + 1] as u16) << 0;
		offset += 2;

		print!("constants_pool_count: {}\n", constants_pool_count);
		constants = repeat(Constant::Default()).take(constants_pool_count as usize).collect();

		for i in 1 .. constants_pool_count as usize {
			match ConstantTags::from_u8(bytes[offset]) {
				Some(ConstantTags::CONSTANT_Class) => {
					let tag:u8 = bytes[offset];
					let name_index:u16 = (bytes[offset+1] as u16) << 8 |
					                     (bytes[offset+2] as u16);
					offset+=3;
					constants[i] = Constant::Class(tag, name_index);
				},
				Some(ConstantTags::CONSTANT_Fieldref) => {
					let tag:u8 = bytes[offset];
					let index:u16 = (bytes[offset+1] as u16) << 8 |
					                (bytes[offset + 2] as u16);
					let name_and_type_index: u16 = (bytes[offset+3] as u16) << 8 |
					                               (bytes[offset+4] as u16);
					offset+=5;
					constants[i] = Constant::Fieldref(tag, index, name_and_type_index);
				},
				Some(ConstantTags::CONSTANT_Methodref) => {
					let tag:u8 = bytes[offset];
					let index:u16 = (bytes[offset+1] as u16) << 8 |
					                (bytes[offset + 2] as u16);
					let name_and_type_index: u16 = (bytes[offset+3] as u16) << 8 |
					                               (bytes[offset+4] as u16);
					offset+=5;
					constants[i] = Constant::Methodref(tag, index, name_and_type_index);
				},
				Some(ConstantTags::CONSTANT_InterfaceMethodref) => {
					print!("InterfaceMethodref\n");
				},
				Some(ConstantTags::CONSTANT_String) => { 
					let tag:u8 = bytes[offset];
					let string_index:u16 = (bytes[offset+1] as u16) << 8 |
					                       (bytes[offset + 2] as u16);
					offset+=3;
					constants[i] = Constant::String(tag, string_index);
				},
				Some(ConstantTags::CONSTANT_Integer) => { 
					print!("Integer\n");
					let tag:u8 = bytes[offset];
					let bytes:u32 = (bytes[offset+1] as u32) << 24  |
					                (bytes[offset + 2] as u32) << 16|
					                (bytes[offset + 3] as u32) << 8 |
					                (bytes[offset + 4] as u32) << 0;
					offset+=5;
					constants[i] = Constant::Integer(tag, bytes);
				},
				Some(ConstantTags::CONSTANT_Float) => {
					print!("Float\n");
				},
				Some(ConstantTags::CONSTANT_Long) => {
					print!("Long\n");
				},
				Some(ConstantTags::CONSTANT_Double) => {
					print!("Double\n");
				},
				Some(ConstantTags::CONSTANT_NameAndType) => {
					let tag:u8 = bytes[offset];
					let name_index:u16 = (bytes[offset+1] as u16) << 8 |
					                     (bytes[offset + 2] as u16);
					let descriptor_index: u16 = (bytes[offset+3] as u16) << 8 |
					                            (bytes[offset+4] as u16);
					offset+=5;
					constants[i] = Constant::NameAndType(tag, name_index, descriptor_index);
				},
				Some(ConstantTags::CONSTANT_Utf8) => {
					let mut reserved: Utf8Reserved = Utf8Reserved::NotReserved;
					let tag:u8 = bytes[offset];
					let length:u16 = (bytes[offset+1] as u16) << 8 |
					                 (bytes[offset+2] as u16);
					let value_range = offset+3 .. offset+3+(length as usize);
					let value = str::from_utf8(&bytes[value_range]).unwrap();

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

					offset += 1+2+(length as usize);
					constants[i] = Constant::Utf8(tag,reserved,length,value.to_string());
				},
				Some(ConstantTags::CONSTANT_MethodHandle) => {
					print!("MethodHandle\n");
				},
				Some(ConstantTags::CONSTANT_MethodType) => {
					print!("MethodType\n");
				},
				Some(ConstantTags::CONSTANT_InvokeDynamic) => {
					print!("InvokeDynamic\n");
				},
				Some(ConstantTags::CONSTANT_Module) => {
					print!("Module\n");
				},
				Some(ConstantTags::CONSTANT_Package) => {
					print!("Package\n");
				},
				_ => {
					print!("oops: unhandled constant pool tag.\n");
				},
			};
		}
		ConstantPool{byte_len: offset, constants: constants}
	}
}
