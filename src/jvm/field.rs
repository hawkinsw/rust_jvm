use jvm::attribute::Attributes;
use std::fmt;
use std::iter::repeat;

#[derive(Default, Clone)]
pub struct Field {
    pub byte_len: usize,
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Attributes,
}

impl Field {
    pub fn byte_len(&self) -> usize {
        self.byte_len
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "access_flags: {}, name_index: {}, descriptor_index: {}, attributes_count: {}, attributes: {}",
			self.access_flags,
			self.name_index,
			self.descriptor_index,
			self.attributes_count,
			self.attributes)
    }
}

impl<'l> From<&'l Vec<u8>> for Field {
    fn from(bytes: &'l Vec<u8>) -> Self {
        let mut offset = 0;
        let access_flags: u16;
        let name_index: u16;
        let descriptor_index: u16;
        let attributes: Attributes;

        access_flags = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
        offset += 2;
        name_index = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
        offset += 2;
        descriptor_index = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
        offset += 2;

        attributes = Attributes::from(&bytes[offset..].to_vec());
        offset += attributes.byte_len();

        Field {
            byte_len: offset,
            access_flags,
            name_index,
            descriptor_index,
            attributes_count: attributes.attributes_count(),
            attributes,
        }
    }
}

#[derive(Clone, Default)]
pub struct Fields {
    byte_len: usize,
    fields: Vec<Field>,
}

impl Fields {
    pub fn set(&mut self, index: usize, field: Field) {
        self.fields[index] = field;
    }

    pub fn get(&self, index: usize) -> Field {
        self.fields[index].clone()
    }

    pub fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub fn fields_count(&self) -> u16 {
        self.fields.len() as u16
    }
}

impl<'l> From<&'l Vec<u8>> for Fields {
    fn from(bytes: &'l Vec<u8>) -> Self {
        let mut offset = 0;
        let mut fields: Vec<Field>;
        let fields_count: u16 = (bytes[offset + 0] as u16) << 8 | (bytes[offset + 1] as u16);
        offset += 2;

        fields = repeat(Field {
            ..Default::default()
        })
        .take(fields_count as usize)
        .collect();
        for field_index in 0..fields_count as usize {
            fields[field_index] = Field::from(&bytes[offset..].to_vec());
            offset += fields[field_index].byte_len();
        }
        Fields {
            byte_len: offset,
            fields: fields,
        }
    }
}

impl fmt::Display for Fields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: fmt::Result = Ok(());
        for i in 0..self.fields.len() {
            result = write!(f, "{}\n", self.get(i))
        }
        result
    }
}
