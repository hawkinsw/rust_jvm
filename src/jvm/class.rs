use enum_primitive::FromPrimitive;
use jvm::constant::Constant;
use jvm::constantpool::ConstantPool;
use std::fs;
use std::io::Read;
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

#[derive(Default)]
pub struct Class{
	bytes: Vec<u8>,
	magic: u32,
	minor_version: u16,
	major_version: u16,
	constant_pool_count: u16,
	constant_pool: ConstantPool,
	access_flags: u16,
	this_class: u16,
	super_class: u16,
	interfaces_count: u16,
}

impl Class {
	pub fn load(class_with_path: &str) -> Class {
		let mut bytes: Vec<u8> = Vec::new();
		let mut c = Class::default();
		let mut offset : usize = 0;

		if let Ok(mut fd) = fs::File::open(class_with_path) {
			if let Err(err) = fd.read_to_end(&mut bytes) {
				print!("oops: could not create a loader: {}\n", err);
				return c;
			}
		} else {
			return c;
		}

		c.bytes = bytes;

		c.magic = ((c.bytes[offset + 0] as u32) << 24) |
			((c.bytes[offset + 1] as u32) << 16) |
			((c.bytes[offset + 2] as u32) << 8) |
			((c.bytes[offset + 3] as u32) << 0);
		offset += 4;

		c.minor_version = ((c.bytes[offset + 0] as u16) << 8) |
			((c.bytes[offset + 1] as u16) << 0);
		offset += 2;

		c.major_version = ((c.bytes[offset + 0] as u16) << 8) |
			((c.bytes[offset + 1] as u16) << 0);
		offset += 2;

		c.constant_pool_count = ((c.bytes[offset + 0] as u16) << 8) |
			((c.bytes[offset + 1] as u16) << 0);
		offset += 2;

		/*
		 * Build the constant pool.
		 */
		c.constant_pool = ConstantPool::new(c.constant_pool_count as usize);
		for i in 0..c.constant_pool_count as usize -1 {
			match ConstantTags::from_u8(c.bytes[offset]) {
				Some(ConstantTags::CONSTANT_Class) => {
					let tag:u8 = c.bytes[offset];
					let name_index:u16 = (c.bytes[offset+1] as u16) << 8 |
					                     (c.bytes[offset + 2] as u16);
					offset+=3;
					c.constant_pool.set(i, Constant::Class(tag, name_index));
				},
				Some(ConstantTags::CONSTANT_Fieldref) => {
					let tag:u8 = c.bytes[offset];
					let index:u16 = (c.bytes[offset+1] as u16) << 8 |
					                (c.bytes[offset + 2] as u16);
					let name_and_type_index: u16 = (c.bytes[offset+3] as u16) << 8 |
					                               (c.bytes[offset+4] as u16);
					offset+=5;
					c.constant_pool.set(i, Constant::Fieldref(tag, index, name_and_type_index));
				},
				Some(ConstantTags::CONSTANT_Methodref) => {
					let tag:u8 = c.bytes[offset];
					let index:u16 = (c.bytes[offset+1] as u16) << 8 |
						c.bytes[offset + 2] as u16;
					let name_and_type_index: u16 = (c.bytes[offset+3] as u16) << 8 |
					                               (c.bytes[offset+4] as u16);
					offset+=5;
					c.constant_pool.set(i, Constant::Methodref(tag, index, name_and_type_index));
				},
				Some(ConstantTags::CONSTANT_InterfaceMethodref) => {
					print!("InterfaceMethodref\n");
				},
				Some(ConstantTags::CONSTANT_String) => { 
					let tag:u8 = c.bytes[offset];
					let string_index:u16 = (c.bytes[offset+1] as u16) << 8 |
					                       (c.bytes[offset + 2] as u16);
					offset+=3;
					c.constant_pool.set(i, Constant::String(tag, string_index));

									},
				Some(ConstantTags::CONSTANT_Integer) => { 
					print!("Integer\n");
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
					let tag:u8 = c.bytes[offset];
					let name_index:u16 = (c.bytes[offset+1] as u16) << 8 |
					                     (c.bytes[offset + 2] as u16);
					let descriptor_index: u16 = (c.bytes[offset+3] as u16) << 8 |
					                            (c.bytes[offset+4] as u16);
					offset+=5;
					c.constant_pool.set(i, Constant::NameAndType(tag, name_index, descriptor_index));
				},
				Some(ConstantTags::CONSTANT_Utf8) => {
					let tag:u8 = c.bytes[offset];
					let length:u16 = (c.bytes[offset+1] as u16) << 8 |
					                 (c.bytes[offset+2] as u16);
					let value = str::from_utf8(&c.bytes[offset+3 .. offset+3+length as usize]).unwrap();
					offset+=1+2+length as usize;
					c.constant_pool.set(i, Constant::Utf8(tag,length,value.to_string()));
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

		c.access_flags = (c.bytes[offset+0] as u16) << 8 |
		               (c.bytes[offset+1] as u16);
		offset+=2;

		c.this_class = (c.bytes[offset+0] as u16) << 8 |
		               (c.bytes[offset+1] as u16);
		offset+=2;

		c.super_class = (c.bytes[offset+0] as u16) << 8 |
		              (c.bytes[offset+1] as u16);
		offset+=2;

		c.interfaces_count = (c.bytes[offset+0] as u16) << 8 |
		                   (c.bytes[offset+1] as u16);
		offset+=2;

		c	
	}

	pub fn print(&self) {
		print!("size: {}\n", self.bytes.len());
		print!("magic: {}\n", self.magic);
		print!("minor_version: {}\n", self.minor_version);
		print!("major_version: {}\n", self.major_version);
		print!("constant_pool_count: {}\n", self.constant_pool_count);
		for i in 0 .. self.constant_pool_count-1 {
			print!("#{}: {}\n", i+1, self.constant_pool.get(i as usize));
		}
		print!("access_flags: {}\n", self.access_flags);
		print!("this_class: {}\n", self.this_class);
		print!("super_class: {}\n", self.super_class);
		print!("interfaces_count: {}\n", self.interfaces_count);
	}
}
