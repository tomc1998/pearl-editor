use java_model::*;
use std::sync::Mutex;
use search::SearchBuffer;
use std::ptr::null_mut;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// Selection of a package. Contains the qualified package name.
    Package(String),
    /// Selection of a decl / interface. Contains the qualified decl name.
    Decl(String),
}

impl Selection {
    /// Checks if this selection is a package, then checks if the package name matches the given
    /// input p.
    pub fn is_package(&self, p: &str) -> bool {
        match *self {
            Selection::Package(ref _p) => p == _p,
            _ => false,
        }
    }

    /// Checks if this selection is a decl, then checks if the package name matches the given
    /// input p.
    pub fn is_decl(&self, p: &str) -> bool {
        match *self {
            Selection::Decl(ref _p) => p == _p,
            _ => false,
        }
    }
}

pub struct Project {
    pub package_list: Mutex<Vec<Package>>,

    /// A searchable list of strings for autocompleting packages
    pub pkg_completion_list: Mutex<SearchBuffer>,

    /// A searchable list of strings for autocompleting decls. This will probably be pretty
    /// fucking heavyweight to search.
    pub decl_completion_list: Mutex<SearchBuffer>,

    /// A reference to the current selcetion. This will be highlighted when rendering, and allows
    /// for faster editing due to context-aware commands (i.e. create decl will already have
    /// package filled in when that package is selected)
    pub curr_sel: Mutex<Option<Selection>>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            package_list: Mutex::new(Vec::new()),
            curr_sel: Mutex::new(None),
            pkg_completion_list: Mutex::new(SearchBuffer::new()),
            decl_completion_list: Mutex::new(SearchBuffer::new()),
        }
    }

    /// Regenerate the decl completion list.
    pub fn regen_decl_completion_list(&self) {
        let decl_completion_list = &mut *self.decl_completion_list.lock().unwrap();
        decl_completion_list.clear();
        let package_list = self.package_list.lock().unwrap();
        for p in package_list.iter() {
            decl_completion_list.add_strings_owned(&p.gen_decl_completion_list()[..]);
        }
    }

    /// Regenerate the package completion list.
    #[allow(dead_code)]
    pub fn regen_pkg_completion_list(&self) {
        let pkg_completion_list = &mut *self.pkg_completion_list.lock().unwrap();
        pkg_completion_list.clear();
        let package_list = self.package_list.lock().unwrap();
        for p in package_list.iter() {
            pkg_completion_list.add_strings_owned(&p.gen_package_completion_list()[..]);
        }
    }

    /// Add a declaration to the given package
    pub fn add_decl(&self, pkg: &str, decl: Declaration) {
        assert!(pkg.len() > 0, "Trying to add decl to default package, not implemented: {}", pkg);
        self.add_package(pkg);
        for p in self.package_list.lock().unwrap().iter_mut() {
            if p.add_decl(pkg, decl.clone()) {
                return;
            }
        }
    }

    /// Add a fully qualified package name. If the start of the package name is already used, trace
    /// down the tree and insert new package in the appropriate replaces.     
    pub fn add_package(&self, name: &str) {
        let mut package_list = self.package_list.lock().unwrap();
        for p in package_list.iter_mut() {
            let deepest = p.add_subpackage(name);
            if deepest.is_some() {
                return;
            }
        }
        package_list.push(Package::new(name).0);
    }

    /// Add a member to a given fully qualified decl name. Panics if decl not found. Eventually
    /// this should probably return a result!;)
    pub fn add_decl_member(&self, name: &str, field_name: &str) {
        let splits: Vec<&str> = name.split(".").collect();
        let first_pkg_name = splits[0];
        let mut package_list = self.package_list.lock().unwrap();
        let mut pkg_ptr: *mut Package = null_mut();
        for p in package_list.iter_mut() {
            if p.name == first_pkg_name {
                pkg_ptr = p;
                break;
            }
        }
        if pkg_ptr == null_mut() {
            panic!("Package not found");
        }

        'outer: for s in &splits[1..splits.len() - 1] {
            let curr_pkg;
            unsafe {
                curr_pkg = &mut *pkg_ptr;
            }
            let package_list = &mut curr_pkg.package_list;
            for p in package_list {
                if p.name == *s {
                    pkg_ptr = p;
                    continue 'outer;
                }
            }
            // If we're here, the package wasn't found
            panic!("Adding decl member to unknown decl");
        }

        // Find decl in pkg_ptr
        let p;
        let mut decl = None;
        unsafe { p = &mut *pkg_ptr };
        for d in &mut p.decl_list {
            if &d.name() == splits.last().unwrap() {
                decl = Some(d);
                break;
            }
        }
        assert!(decl.is_some(), "Adding decl member to unknown decl");

        match *decl.unwrap() {
            Declaration::Class(ref mut c) => {
                c.members.push(ClassMember {
                    modifiers: vec![Modifier::Private],
                    name: field_name.to_owned(),
                    member_type: MemberType::Variable,
                });
            }
        }
    }
}
