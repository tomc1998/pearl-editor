use super::Declaration;

pub struct Package {
    pub name: String,
    pub decl_list: Vec<Declaration>,
    pub package_list: Vec<Package>,
}

impl Package {
    pub fn new(name: String) -> Package {
        Package {
            name: name,
            decl_list: Vec::new(),
            package_list: Vec::new(),
        }
    }
}
