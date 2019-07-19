use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum JvmPrimitiveType {
    Boolean,
    Integer,
}

#[derive(Clone)]
pub enum JvmReferenceType {
    Array(Rc<JvmTypeValue>, u64),
    Class(String),
    Interface(String),
}

#[derive(Clone)]
pub struct JvmPrimitiveTypeValue {
    tipe: JvmPrimitiveType,
    pub value: i64,
}

impl JvmPrimitiveTypeValue {
    pub fn new(tipe: JvmPrimitiveType, value: i64) -> Self {
        Self {
            tipe: tipe,
            value: value,
        }
    }
}

#[derive(Clone)]
pub struct JvmReferenceTypeValue {
    tipe: JvmReferenceType,
    reference: u64,
}

impl JvmReferenceTypeValue {
    pub fn new_array(dimension: u64, component_type: JvmTypeValue, reference: u64) -> Self {
        JvmReferenceTypeValue {
            tipe: JvmReferenceType::Array(Rc::new(component_type), dimension),
            reference: reference,
        }
    }

    pub fn new_class(name: String, reference: u64) -> Self {
        JvmReferenceTypeValue {
            tipe: JvmReferenceType::Class(name),
            reference: reference,
        }
    }

    pub fn new_interface(name: String, reference: u64) -> Self {
        JvmReferenceTypeValue {
            tipe: JvmReferenceType::Interface(name),
            reference: reference,
        }
    }
}

impl fmt::Display for JvmPrimitiveTypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = match self.tipe {
            JvmPrimitiveType::Boolean => "Boolean",
            JvmPrimitiveType::Integer => "Integer",
        };
        write!(f, "{}: {}", type_name, self.value)
    }
}

impl fmt::Display for JvmTypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JvmTypeValue::Primitive(p) => return write!(f, "{}", p),
            _ => (),
        };
        return write!(f, "Can't print references yet.");
    }
}

#[derive(Clone)]
pub enum JvmTypeValue {
    Primitive(JvmPrimitiveTypeValue),
    Reference(JvmReferenceTypeValue),
}
