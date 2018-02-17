use super::Declaration;

pub struct Package {
    pub name: String,
    pub decl_list: Vec<Declaration>,
    pub package_list: Vec<Package>,
}

impl Package {
    /// Creates a package from a name. Will split on the '.' char into subpackages.
    /// This returns the root package, and also a mut pointer to the deepest package created. This
    /// is just for convenience.
    /// 
    /// # Returns
    /// Returns the root package, and optionally a mut pointer to the deepest package created. If
    /// this is none, only 1 package was created. Since we pass the root package back on the stack,
    /// holding a pointer to this package results in a segfault.
    ///
    /// # Caution
    /// As soon as any of the parent packages of the returned package pointer are modified, the
    /// package pointer will be invalidated. The package pointer isn't meant to be stored for long
    /// time use, and is mainly for the convenience of adding a package then adding a class to the
    /// deepest package added, without having to traverse the tree twice.
    pub fn new(name: &str) -> (Package, Option<*mut Package>) {
        assert!(name.len() > 0, "Trying to create package with name len 0");
        let mut splits = name.split(".");
        let mut root = Package {
            name: splits.next().unwrap().to_owned(),
            decl_list: Vec::new(),
            package_list: Vec::new(),
        };

        let mut curr_pkg: *mut Package = &mut root;
        for s in splits {
            unsafe {
                (*curr_pkg).package_list.push(Package {
                    name: s.to_owned(),
                    decl_list: Vec::new(),
                    package_list: Vec::new(),
                });
                curr_pkg = &mut (*curr_pkg).package_list[0];
            }
        }

        if curr_pkg == &mut root {
            return (root, None);
        }
        else {
            return (root, Some(curr_pkg));
        }
    }

    /// Given the (full) name of a package, traces down the tree until there's a divergence, then
    /// adds packages from there.
    ///
    /// Panics if the initial name of the package does not match the name of this package - for
    /// example, trying to call add_subpackage with the name com.tom.example on a package named
    /// 'asd' would panic, as 'com' does not match 'asd'. Returns a mutable reference to the
    /// package that was added.
    pub fn add_subpackage(&mut self, name: &str) -> &mut Package {
        let mut splits = name.split(".");
        assert_eq!(
            &self.name,
            splits.next().unwrap(),
            "Package name must match the package when calling add_subpackage."
        );

        let mut curr_pkg: *mut Package = self;
        'outer: for s in splits {
            unsafe {
                for p in &mut (*curr_pkg).package_list {
                    if p.name == s {
                        curr_pkg = p;
                        continue 'outer;
                    }
                }
                // If we're here, a package wasn't found and we need to add subpackages
                (*curr_pkg).package_list.push(Package {
                    name: s.to_owned(),
                    decl_list: Vec::new(),
                    package_list: Vec::new(),
                });
                curr_pkg = (*curr_pkg).package_list.last_mut().unwrap();
            }
        }
        unsafe {
            return &mut *curr_pkg;
        }
    }
}
