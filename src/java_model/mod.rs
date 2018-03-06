#![allow(dead_code)]

mod class;
mod modifier;
mod package;
mod field;

pub use self::class::{MemberType, ClassMember, Class};
pub use self::modifier::Modifier;
pub use self::package::Package;
pub use self::field::Field;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Class(Class),
}

impl Declaration {
    pub fn name(&self) -> &str {
        match self {
            &Declaration::Class(ref c) => c.name.as_ref(),
        }
    }
}
