use java_model::*;
use std::sync::Mutex;
use search::SearchBuffer;

pub struct Project {
    pub package_list: Mutex<Vec<Package>>,

    /// A searchable list of strings for autocompleting packages
    pub pkg_completion_list: Mutex<SearchBuffer>,

    /// A reference to the current package. This will be highlighted when rendering, and allows for
    /// faster editing due to context-aware commands (i.e. create class will already have package
    /// filled in)
    pub curr_pkg: Mutex<Option<String>>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            package_list: Mutex::new(Vec::new()),
            curr_pkg: Mutex::new(None),
            pkg_completion_list: Mutex::new(SearchBuffer::new()),
        }
    }

    /// Regenerate the package completion list.
    pub fn regen_pkg_completion_list(&self) {
        let pkg_completion_list = &mut *self.pkg_completion_list.lock().unwrap();
        pkg_completion_list.clear();
        let package_list = self.package_list.lock().unwrap();
        for p in package_list.iter() {
            pkg_completion_list.add_strings_owned(
                &p.gen_package_completion_list()[..],
            );
        }
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
        let mut deepest : Option<*mut Package> = None;
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
