use super::Declaration;

pub struct Package {
    /// Name is the name of JUST this package.
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
    /// time use, and is mainly for the convenience of adding a package then adding a decl to the
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
        } else {
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

    /// Generate a list of fully qualified package names - e.g. for the package com.tom.example,
    /// this will produce 3 items - 'com', 'com.tom', 'com.tom.example'.
    pub fn gen_package_completion_list(&self) -> Vec<String> {
        use smallvec::SmallVec;
        // stack of packages to visit for a full traversal
        let mut names = Vec::new();
        // Pairs of package pointers and name prefixes
        let mut pkg_stack: SmallVec<[(String, *const Package); 16]> = SmallVec::new();
        pkg_stack.push(("".to_owned(), self));

        while pkg_stack.len() > 0 {
            unsafe {
                let (prefix, p) = pkg_stack.pop().unwrap();
                let p = &*p;
                names.push(prefix.clone() + &p.name);
                for child in &p.package_list {
                    pkg_stack.push((prefix.clone() + &p.name + ".", child));
                }
            }
        }
        return names;
    }

    /// Generate a list of fully qualified decl names.
    pub fn gen_decl_completion_list(&self) -> Vec<String> {

        // inner function to allow recursion. Given a package and a prefix to that package,
        // generate a list of decl names, call recursively, and append all to the given curr_pkg
        // vec. This is to avoid huge reallocations - appending to the same vec is quite easy, vs
        // appending to 1 vec then a big memcpy.
        //
        // The input prefix is mutable to allow us to push names / pop names, but the final value
        // of prefix should remain the same.
        fn _gen_decl_completion_list(
            curr_name_list: &mut Vec<String>,
            pkg: &Package,
            prefix: &mut String,
        ) {
            let orig_prefix_len = prefix.len();
            prefix.push_str(&pkg.name);
            prefix.push_str(".");
            for d in &pkg.decl_list {
                curr_name_list.push(prefix.clone() + d.name());
            }
            for p in &pkg.package_list {
                _gen_decl_completion_list(curr_name_list, p, prefix);
            }
            prefix.truncate(orig_prefix_len);
        }

        let mut names = Vec::new();
        _gen_decl_completion_list(&mut names, self, &mut "".to_owned());

        return names;
    }

    /// Find a package given a fully qualified name.
    /// If the package could not be found, return the package we 'made it to' whilst searching.
    /// This might be self. If the package could not be found, false is returned as the second item
    /// in the tuple - if the package was found, true is returned.
    pub fn find_pkg(&self, name: &str) -> (&Package, bool) {
        use std::iter::Peekable;
        if name.len() == 0 {
            return (self, false);
        }

        // Inner function to allow recursion.
        fn _find_pkg<'a, 'b, I: Iterator<Item = &'b str>>(
            mut splits: Peekable<I>,
            curr_pkg: &'a Package,
        ) -> (&'a Package, bool) {
            let name = splits.next().unwrap();
            for p in &curr_pkg.package_list {
                if p.name == name {
                    if splits.peek().is_some() {
                        return (p, true);
                    } else {
                        return _find_pkg(splits, p);
                    }
                }
            }
            return (curr_pkg, false);
        }

        let splits = name.split(".").peekable();
        return _find_pkg(splits, self);
    }

    /// Find a package given a fully qualified name.
    /// If the package could not be found, return the package we 'made it to' whilst searching.
    /// This might be self. If the package could not be found, false is returned as the second item
    /// in the tuple - if the package was found, true is returned.
    pub fn find_pkg_mut(&mut self, name: &str) -> (&mut Package, bool) {
        use std::iter::Peekable;
        if name.len() == 0 {
            return (self, false);
        }

        // Inner function to allow recursion.
        fn _find_pkg<'a, 'b, I: Iterator<Item = &'b str>>(
            mut splits: Peekable<I>,
            curr_pkg: &'a mut Package,
        ) -> (&'a mut Package, bool) {
            let name = splits.next().unwrap();
            for p in &mut curr_pkg.package_list {
                if p.name == name {
                    if splits.peek().is_some() {
                        return (p, true);
                    } else {
                        return _find_pkg(splits, p);
                    }
                }
            }
            return (curr_pkg, false);
        }

        let splits = name.split(".").peekable();
        return _find_pkg(splits, self);
    }
}
