use ::Modifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberType {
    Variable,
    Method,
    Constructor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassMember {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub member_type: MemberType,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub name: String,
    pub type_params: Vec<String>,
    pub implements: Vec<String>,
    pub extends: String,
    pub members: Vec<ClassMember>,
    pub inner_classes: Vec<Class>,
    pub modifiers: Vec<Modifier>
}

impl Class {
    pub fn new_empty() -> Class {
        Class {
            name: "".to_owned(),
            type_params: Vec::new(),
            implements: Vec::new(),
            extends: "".to_owned(),
            members: Vec::new(),
            inner_classes: Vec::new(),
            modifiers: Vec::new(),
        }
    }
}

