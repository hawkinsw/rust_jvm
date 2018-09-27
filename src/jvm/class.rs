use enum_primitive::FromPrimitive;
use jvm::constant::Constant;
use jvm::constantpool::ConstantPool;
use jvm::field::Fields;
use jvm::field::Field;
use jvm::attribute::Attribute;
use jvm::attribute::Attributes;
use jvm::method::Methods;
use jvm::method::Method;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::str;
use std::iter;
use std::fmt;

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
	interfaces: Vec<u16>,
	fields_count: u16,
	fields: Fields,
	methods_count: u16,
	methods: Methods,
	attributes_count: u16,
	attributes: Attributes,
}

impl Class {

	pub fn get_method(&self, method_name: String) -> Option<&Method> {
		self.methods.get_by_name(&method_name, &self.constant_pool)
	}

	fn load_constant_pool(c: &mut Class,
	                      constants_count: u16,
	                      offset: usize) -> usize {
		let mut offset = offset;
		c.constant_pool = ConstantPool::new(constants_count as usize);
		for i in 1 .. constants_count as usize {
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
					c.constant_pool.set(i, Constant::Fieldref(tag, 
					                                            index,
					                                            name_and_type_index));
				},
				Some(ConstantTags::CONSTANT_Methodref) => {
					let tag:u8 = c.bytes[offset];
					let index:u16 = (c.bytes[offset+1] as u16) << 8 |
					                (c.bytes[offset + 2] as u16);
					let name_and_type_index: u16 = (c.bytes[offset+3] as u16) << 8 |
					                               (c.bytes[offset+4] as u16);
					offset+=5;
					c.constant_pool.set(i, Constant::Methodref(tag,
					                                             index,
					                                             name_and_type_index));
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
					let tag:u8 = c.bytes[offset];
					let bytes:u32 = (c.bytes[offset+1] as u32) << 24  |
					                (c.bytes[offset + 2] as u32) << 16|
					                (c.bytes[offset + 3] as u32) << 8 |
					                (c.bytes[offset + 4] as u32) << 0;
					offset+=5;
					c.constant_pool.set(i, Constant::Integer(tag, bytes));
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
					c.constant_pool.set(i, Constant::NameAndType(tag,
					                                               name_index,
					                                               descriptor_index));
				},
				Some(ConstantTags::CONSTANT_Utf8) => {
					let tag:u8 = c.bytes[offset];
					let length:u16 = (c.bytes[offset+1] as u16) << 8 |
					                 (c.bytes[offset+2] as u16);
					let value_range = offset+3 .. offset+3+(length as usize);
					let value = str::from_utf8(&c.bytes[value_range]).unwrap();
					offset += 1+2+(length as usize);
					c.constant_pool.set(i, Constant::Utf8(tag,
					                                        length,
					                                        value.to_string()));
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
		offset
	}

	fn load_attributes(c: &mut Class, attributes_count: u16, offset: usize)->usize{
		let mut offset = offset;

		c.attributes = Attributes::new(c.attributes_count as usize);
		for attribute_index in 0 .. c.attributes_count as usize {
			let attribute_name_index: u16;
			let attribute_length: u32;
			let mut attribute: Attribute;

			attribute_name_index = (c.bytes[offset+0] as u16) << 8 |
			                       (c.bytes[offset+1] as u16);
			offset+=2;
			attribute_length = (c.bytes[offset+0] as u32) << 24 |
			                   (c.bytes[offset+1] as u32) << 16 |
			                   (c.bytes[offset+2] as u32) << 8  |
			                   (c.bytes[offset+3] as u32);
			offset+=4;
			attribute = Attribute::new(attribute_length as usize);
			attribute.attribute_name_index = attribute_name_index;
			attribute.attribute_length = attribute_length;
			/*
			 * Parse the attributes
			 */
			for info in 0 .. attribute_length {
				attribute.info[info as usize] = c.bytes[offset];
				offset+=1;
			}
			/*
			 * Assign the completed method attribute
			 */
			c.attributes.set(attribute_index as usize, attribute);
		}
		offset
	}

	fn load_fields(c: &mut Class, fields_count: u16, offset: usize)->usize {
		let mut offset = offset;

		c.fields = Fields::new(fields_count as usize);
		for field_index in 0 .. fields_count as usize {
			let access_flags: u16;
			let name_index: u16;
			let descriptor_index: u16;
			let attributes_count: u16;
			let mut f: Field;

			access_flags = (c.bytes[offset+0] as u16) << 8 |
			               (c.bytes[offset+1] as u16);
			offset+=2;
			name_index = (c.bytes[offset+0] as u16) << 8 |
			             (c.bytes[offset+1] as u16);
			offset+=2;
			descriptor_index = (c.bytes[offset+0] as u16) << 8 |
			                   (c.bytes[offset+1] as u16);
			offset+=2;
			attributes_count = (c.bytes[offset+0] as u16) << 8 |
			                   (c.bytes[offset+1] as u16);
			offset+=2;

			f = Field::new(attributes_count as usize);
			f.access_flags = access_flags;
			f.descriptor_index = descriptor_index;
			f.attributes_count = attributes_count;
			/*
			 * Now, parse the attributes.
			 */
			for attribute_index in 0 .. attributes_count {
				let attribute_name_index: u16;
				let attribute_length: u32;
				let mut attribute: Attribute;

				attribute_name_index = (c.bytes[offset+0] as u16) << 8 |
				                       (c.bytes[offset+1] as u16);
				offset+=2;
				attribute_length = (c.bytes[offset+0] as u32) << 24 |
				                   (c.bytes[offset+1] as u32) << 16 |
				                   (c.bytes[offset+2] as u32) << 8  |
				                   (c.bytes[offset+3] as u32);
				offset+=4;
				attribute = Attribute::new(attribute_length as usize);
				attribute.attribute_name_index = attribute_name_index;
				attribute.attribute_length = attribute_length;
				/*
				 * Parse the attributes
				 */
				for info in 0 .. attribute_length {
					attribute.info[info as usize] = c.bytes[offset];
					offset+=1;
				}
				/*
				 * Assign the completed field attribute
				 */
				f.attributes.set(attribute_index as usize, attribute);
			}
			c.fields.set(field_index, f);
		}
		offset
	}

	fn load_methods(c: &mut Class, methods_count: u16, offset: usize)->usize{
		let mut offset = offset;
		/*
		 * Handle the methods.
		 */
		c.methods = Methods::new(methods_count as usize);
		for method_index in 0 .. methods_count as usize {
			let access_flags: u16;
			let name_index: u16;
			let descriptor_index: u16;
			let attributes_count: u16;
			let attributes: Attributes;
			let mut m: Method;

			access_flags = (c.bytes[offset] as u16) << 8 |
			               (c.bytes[offset+1] as u16) << 0;
			offset+=2;
			name_index = (c.bytes[offset] as u16) << 8 |
			             (c.bytes[offset+1] as u16) << 0;
			offset+=2;
			descriptor_index = (c.bytes[offset] as u16) << 8 |
			                   (c.bytes[offset+1] as u16) << 0;
			offset+=2;
			attributes_count = (c.bytes[offset] as u16) << 8 |
			                   (c.bytes[offset+1] as u16) << 0;
			offset+=2;

			m = Method::new(attributes_count as usize);
			m.access_flags = access_flags;
			m.name_index = name_index;
			m.descriptor_index = descriptor_index;
			m.attributes_count = attributes_count;

			for attribute_index in 0 .. attributes_count {
				let attribute_name_index: u16;
				let attribute_length: u32;
				let mut attribute: Attribute;

				attribute_name_index = (c.bytes[offset+0] as u16) << 8 |
				                       (c.bytes[offset+1] as u16);
				offset+=2;
				attribute_length = (c.bytes[offset+0] as u32) << 24 |
				                   (c.bytes[offset+1] as u32) << 16 |
				                   (c.bytes[offset+2] as u32) << 8  |
 				                   (c.bytes[offset+3] as u32);
				offset+=4;
				attribute = Attribute::new(attribute_length as usize);
				attribute.attribute_name_index = attribute_name_index;
				attribute.attribute_length = attribute_length;
				/*
				 * Parse the attributes
				 */
				for info in 0 .. attribute_length {
					attribute.info[info as usize] = c.bytes[offset];
					offset+=1;
				}
				/*
				 * Assign the completed method attribute
				 */
				m.attributes.set(attribute_index as usize, attribute);
			}
			c.methods.set(method_index, m);
		}
		offset
	}

	pub fn load(class_with_path: &str) -> Option<Class> {
		let mut bytes: Vec<u8> = Vec::new();
		let mut c = Class::default();
		let mut offset : usize = 0;
		let mut fd: fs::File;

		match fs::File::open(class_with_path) {
			Ok(mut fd) => {
				if let Err(err) = fd.read_to_end(&mut bytes) {
					print!("oops: could not read the class file '{}': {}\n",
					       class_with_path,
					       err);
					return None;
				}
			},
			Err(err) => {
				print!("oops: could not read the class file '{}': {}\n",
				       class_with_path,
				       err);
				return None;
			}
		}

		c.bytes = bytes;

		c.magic = (c.bytes[offset + 0] as u32) << 24 |
		          (c.bytes[offset + 1] as u32) << 16 |
		          (c.bytes[offset + 2] as u32) << 8  |
		          (c.bytes[offset + 3] as u32) << 0;
		offset += 4;

		c.minor_version = (c.bytes[offset + 0] as u16) << 8 |
		                  (c.bytes[offset + 1] as u16) << 0;
		offset += 2;

		c.major_version = (c.bytes[offset + 0] as u16) << 8 |
		                  (c.bytes[offset + 1] as u16) << 0;
		offset += 2;

		/*
		 * Load the constants pool.
		 */
		let constants_pool_count: u16 = (c.bytes[offset + 0] as u16) << 8 |
		                                (c.bytes[offset + 1] as u16) << 0;
		c.constant_pool_count = constants_pool_count.clone();
		offset += 2;
		offset = Class::load_constant_pool(&mut c, constants_pool_count, offset);

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

		/*
		 * Handle the interfaces.
		 */
		c.interfaces = iter::repeat(0 as u16)
		                    .take(c.interfaces_count as usize)
		                    .collect();
		for i in 1 .. c.interfaces_count as usize {	
			c.interfaces[i] = (c.bytes[offset+0] as u16) << 8 |
			                  (c.bytes[offset+1] as u16);
			offset+=2;
		}

		/*
		 * Now parse the fields.
		 */

		let fields_count: u16 = (c.bytes[offset+0] as u16) << 8 |
		                        (c.bytes[offset+1] as u16);
		c.fields_count = fields_count.clone();

		offset+=2;
		offset = Class::load_fields(&mut c, fields_count, offset);

		/*
		 * Now parse the methods.
		 */
		let methods_count = (c.bytes[offset] as u16) << 8 |
		                    (c.bytes[offset+1] as u16) << 0;
		c.methods_count = methods_count.clone();
		offset+=2;

		offset = Class::load_methods(&mut c, methods_count, offset);

		/*
		 * Now parse the attributes.
		 */
		let attributes_count: u16 = (c.bytes[offset] as u16) << 8 |
		                            (c.bytes[offset+1] as u16) << 0;
		c.attributes_count = attributes_count.clone();
		offset+=2;

		offset = Class::load_attributes(&mut c, attributes_count, offset);
		Some(c)
	}
}

impl fmt::Display for Class {
	fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
		write!(f,"size: {}\n", self.bytes.len());
		write!(f,"magic: {}\n", self.magic);
		write!(f,"minor_version: {}\n", self.minor_version);
		write!(f,"major_version: {}\n", self.major_version);
		write!(f,"constant_pool_count: {}\n", self.constant_pool_count);
		for i in 1 .. self.constant_pool_count {
			write!(f,"#{}: {}\n", i, self.constant_pool.get(i as usize));
		}
		write!(f,"access_flags: {}\n", self.access_flags);
		write!(f,"this_class: {}\n", self.this_class);
		write!(f,"super_class: {}\n", self.super_class);
		write!(f,"interfaces_count: {}\n", self.interfaces_count);
		for i in 1 .. self.interfaces_count  {
			write!(f,"#{}: {}\n", i, self.interfaces[i as usize - 1]);
		}
		write!(f,"fields_count: {}\n", self.fields_count);
		write!(f,"fields: {}\n", self.fields);
		write!(f,"methods_count: {}\n", self.methods_count);
		write!(f,"methods: {}\n", self.methods);
		write!(f,"attributes_count: {}\n", self.attributes_count);
		write!(f,"attributes: {}\n", self.attributes)
	}
}
