use java_model::*;
use std::sync::Mutex;
use search::SearchBuffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// Selection of a package. Contains the qualified package name.
    Package(String),
    /// Selection of a class / interface. Contains the qualified class name.
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

    /// A searchable list of strings for autocompleting classes. This will probably be pretty
    /// fucking heavyweight to search.
    pub decl_completion_list: Mutex<SearchBuffer>,

    /// A reference to the current selcetion. This will be highlighted when rendering, and allows
    /// for faster editing due to context-aware commands (i.e. create class will already have
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

    /// Regenerate the package completion list.
    pub fn regen_pkg_completion_list(&self) {
        let decl_completion_list = &mut *self.decl_completion_list.lock().unwrap();
        decl_completion_list.clear();
        let package_list = self.package_list.lock().unwrap();
        for p in package_list.iter() {
            decl_completion_list.add_strings_owned(&p.gen_decl_completion_list()[..]);
        }
    }

    /// Regenerate the decl completion list.
    pub fn regen_decl_completion_list(&self) {
        let pkg_completion_list = &mut *self.pkg_completion_list.lock().unwrap();
        pkg_completion_list.clear();
        let package_list = self.package_list.lock().unwrap();
        for p in package_list.iter() {
            pkg_completion_list.add_strings_owned(&p.gen_package_completion_list()[..]);
        }
    }


    /// Given a full qualified package name, returns true if that package exists.
    /// If name len is 0, this returns true (default package always exists
    #[allow(dead_code)]
    pub fn package_exists(&self, name: &str) -> bool {
        use std::ptr::null;

        if name.len() == 0 {
            return true;
        }

        let mut splits = name.split(".");
        let first_pkg_name = splits.next().unwrap();
        let package_list = self.package_list.lock().unwrap();
        let mut pkg_ptr: *const Package = null();
        for p in package_list.iter() {
            if p.name == first_pkg_name {
                pkg_ptr = p;
                break;
            }
        }
        if pkg_ptr == null() {
            return false;
        }

        'outer: for s in splits {
            let curr_pkg;
            unsafe {
                curr_pkg = &*pkg_ptr;
            }
            let package_list = &curr_pkg.package_list;
            for p in package_list {
                if p.name == s {
                    pkg_ptr = p;
                    continue 'outer;
                }
            }
            // If we're here, the package wasn't found
            return false;
        }
        return true;
    }

    /// Add a fully qualified package name. If the start of the package name is already used, trace
    /// down the tree and insert new package in the appropriate replaces. Return a mutable
    /// pointer to the last created package.
    ///
    /// This will lock the package list mutex, and the mutex will stay locked whilst you hold the
    /// package reference.
    ///
    /// # Caution
    /// See package::Package::new() for details - the mut pointer returned isn't guaranteed to be
    /// valid forever, and is only a convenience measure to quickly add a class to the deepest
    /// package.
    pub fn add_package(&self, name: &str) -> *mut Package {
        let mut deepest: Option<*mut Package> = None;
        {
            let first_pkg_name = &name[0..name.find(".").unwrap_or(name.len())];
            let mut package_list = self.package_list.lock().unwrap();
            let mut found_subpkg = false;
            for p in &mut package_list.iter_mut() {
                if p.name == first_pkg_name {
                    deepest = Some(p.add_subpackage(name));
                    found_subpkg = true;
                    break;
                }
            }
            if found_subpkg {
                drop(package_list); // Drop otherwise we deadlock with regen_pkg_completion_list
                self.regen_pkg_completion_list();
                return deepest.unwrap();
            }
            let (pkg, mut deepest_opt) = Package::new(name);
            package_list.push(pkg);
            if deepest_opt.is_none() {
                deepest_opt = Some(package_list.last_mut().unwrap() as *mut Package);
            }
            deepest = deepest_opt;
        }
        self.regen_pkg_completion_list();
        return deepest.unwrap();
    }
}
