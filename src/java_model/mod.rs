#![allow(dead_code)]

mod class;
mod modifier;

pub use self::class::{MemberType, ClassMember, Class};
pub use self::modifier::Modifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Class(Class),
}




