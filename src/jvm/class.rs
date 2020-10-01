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
use jvm::attribute::Attributes;
use jvm::constant::Constant;
use jvm::constantpool::ConstantPool;
use jvm::field::Fields;
use jvm::method::Method;
use jvm::method::Methods;
use jvm::methodarea::MethodArea;
use std::fmt;
use std::fs;
use std::io::Read;
use std::iter;
use std::rc::Rc;

#[repr(u16)]
pub enum ClassAccessFlags {
	Public = 0x0001,
	Final = 0x0010,
	Super = 0x0020,
	Interface = 0x0200,
	Abstract = 0x0400,
	Synthetic = 0x1000,
	Annotation = 0x2000,
	Enum = 0x4000,
}

pub enum ClassInitializationStatus {
	VerifiedPreparedNotInitialized,
	BeingInitialized,
	Initialized,
	Error,
}

#[derive(Clone, Default)]
pub struct Class {
	bytes: Vec<u8>,
	magic: u32,
	minor_version: u16,
	major_version: u16,
	constant_pool_count: u16,
	constant_pool: ConstantPool,
	pub access_flags: u16,
	this_class: u16,
	super_class: u16,
	interfaces_count: u16,
	interfaces: Vec<u16>,
	fields_count: u16,
	fields: Fields,
	methods_count: u16,
	methods: Methods,
	attributes_count: u16,
	attributes: Attributes,
}

impl Class {
	pub fn get_constant_pool_ref(&self) -> &ConstantPool {
		&self.constant_pool
	}

	pub fn resolve_superclass(&self) -> Option<String> {
		let mut superclass_name: Option<String> = None;
		let cp = &self.constant_pool;

		if let Constant::Class(_, superclassname_index) =
			cp.get_constant_ref(self.super_class as usize)
		{
			if let Constant::Utf8(_, _, _, superclassname_value) =
				cp.get_constant_ref(*superclassname_index as usize)
			{
				superclass_name = Some(superclassname_value.to_string());
			}
		}
		superclass_name
	}

	pub fn superclass_name(&self) -> Option<String> {
		self.resolve_superclass()
	}

	/**
	 * is_type_of
	 *
	 * Recursively, check whether `type` matches this class', or one of its
	 * superclasses. Because the parameter is a mutable reference to the MethodArea,
	 * it must be locked before calling. Assume that is the case. TODO: This locking
	 * should be more precise.
	 */
	pub fn is_type_of(&self, r#type: &String, methodarea: &mut MethodArea) -> bool {
		if self.get_class_name().unwrap() == *r#type {
			true
		} else if let Some(parent_name) = self.superclass_name() {
			methodarea.maybe_load_class(&parent_name);
			if let Some(super_class) = methodarea.get_loaded_class(&parent_name) {
				(*super_class).class.is_type_of(r#type, methodarea)
			} else {
				false
			}
		} else {
			false
		}
	}

	pub fn resolve_field_ref(&self, field_ref_index: usize) -> Option<(String, String, String)> {
		let mut result: Option<(String, String, String)> = None;
		let cp = &self.constant_pool;

		if let Constant::Fieldref(_, class_idx, name_type_idx) =
			cp.get_constant_clone(field_ref_index as usize)
		{
			if let Constant::NameAndType(_, field_name_idx, field_type_idx) =
				cp.get_constant_clone(name_type_idx as usize)
			{
				if let Constant::Utf8(_, _, _, name) =
					cp.get_constant_clone(field_name_idx as usize)
				{
					if let Constant::Utf8(_, _, _, r#type) =
						cp.get_constant_clone(field_type_idx as usize)
					{
						if let Constant::Class(_, field_class_idx) =
							cp.get_constant_clone(class_idx as usize)
						{
							if let Constant::Utf8(_, _, _, class) =
								cp.get_constant_clone(field_class_idx as usize)
							{
								result = Some((class, name, r#type));
							}
						}
					}
				}
			}
		}
		/*else {
					FatalError::new(FatalErrorType::InvalidConstantReference(
						class.get_class_name().unwrap(),
						"Utf8".to_string(),
						field_name_idx as u16,
					))
					.call();
				}
			} else {
				FatalError::new(FatalErrorType::InvalidConstantReference(
					class.get_class_name().unwrap(),
					"NameAndType".to_string(),
					field_name_type_idx as u16,
				))
				.call();
			}
		} else {
			FatalError::new(FatalErrorType::InvalidConstantReference(
				class.get_class_name().unwrap(),
				"Fieldref".to_string(),
				field_index as u16,
			))
			.call();
		}
		*/
		result
	}

	/// Resolve a method reference into the name of method, the type of
	/// the method and the class of the method.
	///
	/// # Arguments
	///
	/// `method_ref_index` - The index into this class' constant pool
	/// that points to a method reference.
	pub fn resolve_method_ref(&self, method_ref_index: usize) -> Option<(String, String, String)> {
		let mut result: Option<(String, String, String)> = None;
		let cp = &self.constant_pool;

		if let Constant::Methodref(_, class_index, method_index) =
			cp.get_constant_ref(method_ref_index)
		{
			if let Constant::Class(_, class_name_index) = cp.get_constant_ref(*class_index as usize)
			{
				if let Constant::NameAndType(_, method_name_index, method_type_index) =
					cp.get_constant_ref(*method_index as usize)
				{
					if let Constant::Utf8(_, _, _, class_name) =
						cp.get_constant_ref(*class_name_index as usize)
					{
						if let Constant::Utf8(_, _, _, method_name) =
							cp.get_constant_ref(*method_name_index as usize)
						{
							if let Constant::Utf8(_, _, _, method_type) =
								cp.get_constant_ref(*method_type_index as usize)
							{
								result = Some((
									method_name.to_string(),
									method_type.to_string(),
									class_name.to_string(),
								));
							}
						}
					}
				}
			}
		}
		result
	}

	pub fn get_method_rc_by_name_and_type(
		&self,
		method_name: &String,
		method_type: &String,
	) -> Option<Rc<Method>> {
		self.methods
			.get_by_name_and_type(&method_name, &method_type, &self.constant_pool)
	}

	pub fn get_methods_ref(&self) -> &Methods {
		&self.methods
	}

	pub fn get_mut_fields_ref(&mut self) -> &mut Fields {
		&mut self.fields
	}

	pub fn get_fields_ref(&self) -> &Fields {
		&self.fields
	}

	pub fn get_class_name(&self) -> Option<String> {
		match self
			.constant_pool
			.get_constant_ref(self.this_class as usize)
		{
			Constant::Class(_, name_idx) => {
				match self.constant_pool.get_constant_ref(*name_idx as usize) {
					Constant::Utf8(_, _, _, name) => {
						return Some(name.clone());
					}
					_ => return None,
				}
			}
			_ => return None,
		}
	}

	fn load_constant_pool(c: &mut Class, offset: usize) -> usize {
		c.constant_pool = ConstantPool::from(&c.bytes[offset..].to_vec());
		c.constant_pool_count = c.constant_pool.constant_pool_count();
		offset + c.constant_pool.byte_len()
	}

	fn load_attributes(c: &mut Class, offset: usize) -> usize {
		c.attributes = Attributes::from(&c.bytes[offset..].to_vec());
		c.attributes_count = c.attributes.attributes_count();
		offset + c.attributes.byte_len()
	}

	fn load_fields(c: &mut Class, offset: usize) -> usize {
		c.fields = Fields::from(&c.bytes[offset..].to_vec());
		c.fields_count = c.fields.fields_count();
		offset + c.fields.byte_len()
	}

	fn load_methods(c: &mut Class, offset: usize) -> usize {
		c.methods = Methods::from((&c.bytes[offset..].to_vec(), &c.constant_pool));
		c.methods_count = c.methods.methods_count();
		offset + c.methods.byte_len()
	}

	pub fn load_from_bytes(bytes: Vec<u8>) -> Option<Class> {
		let mut c = Class::default();
		let mut offset: usize = 0;

		c.bytes = bytes;

		c.magic = (c.bytes[offset + 0] as u32) << 24
			| (c.bytes[offset + 1] as u32) << 16
			| (c.bytes[offset + 2] as u32) << 8
			| (c.bytes[offset + 3] as u32) << 0;
		offset += 4;

		c.minor_version = (c.bytes[offset + 0] as u16) << 8 | (c.bytes[offset + 1] as u16) << 0;
		offset += 2;

		c.major_version = (c.bytes[offset + 0] as u16) << 8 | (c.bytes[offset + 1] as u16) << 0;
		offset += 2;

		/*
		 * Load the constants pool.
		 */
		offset = Class::load_constant_pool(&mut c, offset);

		c.access_flags = (c.bytes[offset + 0] as u16) << 8 | (c.bytes[offset + 1] as u16);
		offset += 2;

		c.this_class = (c.bytes[offset + 0] as u16) << 8 | (c.bytes[offset + 1] as u16);
		offset += 2;

		c.super_class = (c.bytes[offset + 0] as u16) << 8 | (c.bytes[offset + 1] as u16);
		offset += 2;

		c.interfaces_count = (c.bytes[offset + 0] as u16) << 8 | (c.bytes[offset + 1] as u16);
		offset += 2;

		/*
		 * Handle the interfaces.
		 */
		c.interfaces = iter::repeat(0 as u16)
			.take(c.interfaces_count as usize)
			.collect();
		for i in 0..c.interfaces_count as usize {
			c.interfaces[i] = (c.bytes[offset + 0] as u16) << 8 | (c.bytes[offset + 1] as u16);
			offset += 2;
		}

		/*
		 * Now parse the fields.
		 */

		offset = Class::load_fields(&mut c, offset);

		/*
		 * Now parse the methods.
		 */
		offset = Class::load_methods(&mut c, offset);

		Class::load_attributes(&mut c, offset);
		Some(c)
	}

	/*
	 * TODO: This is going to be have to be much more robust!
	 */
	pub fn load_from_file(class_with_path: &str) -> Option<Class> {
		let mut bytes: Vec<u8> = Vec::new();
		let mut c = Class::default();
		let mut offset: usize = 0;

		match fs::File::open(class_with_path) {
			Ok(mut fd) => {
				if let Err(err) = fd.read_to_end(&mut bytes) {
					print!(
						"oops: could not read the class file '{}': {}\n",
						class_with_path, err
					);
					return None;
				}
			}
			Err(err) => {
				print!(
					"oops: could not read the class file '{}': {}\n",
					class_with_path, err
				);
				return None;
			}
		}
		Class::load_from_bytes(bytes)
	}
}

impl fmt::Display for Class {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "size: {}\n", self.bytes.len());
		write!(f, "magic: {}\n", self.magic);
		write!(f, "minor_version: {}\n", self.minor_version);
		write!(f, "major_version: {}\n", self.major_version);
		write!(f, "constant_pool_count: {}\n", self.constant_pool_count);
		for i in 1..self.constant_pool_count {
			write!(
				f,
				"#{}: {}\n",
				i,
				self.constant_pool.get_constant_ref(i as usize)
			);
		}
		write!(f, "access_flags: {}\n", self.access_flags);
		write!(f, "this_class: {}\n", self.this_class);
		write!(f, "super_class: {}\n", self.super_class);
		write!(f, "interfaces_count: {}\n", self.interfaces_count);
		for i in 1..self.interfaces_count {
			write!(f, "#{}: {}\n", i, self.interfaces[i as usize - 1]);
		}
		write!(f, "fields_count: {}\n", self.fields_count);
		write!(f, "fields: {}\n", self.fields);
		write!(f, "methods_count: {}\n", self.methods_count);
		write!(f, "methods: {}\n", self.methods);
		write!(f, "attributes_count: {}\n", self.attributes_count);
		write!(f, "attributes: {}\n", self.attributes)
	}
}
