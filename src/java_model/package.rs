use super::Declaration;
use std;

#[derive(PartialEq, Debug, Clone)]
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
        let pkg : *mut Package = root.add_subpackage(name).unwrap();
        // Only return pkg if it's not a pointer to the root (package is returned on the stack, so
        // returning a pointer to pkg in that case would be a segfault)
        if pkg as *mut Package == &mut root as *mut Package {
            return (root, None);
        } else {
            return (root, Some(pkg as *mut Package));
        }
    }

    /// Given the (full) name of a package, traces down the tree until there's a divergence, then
    /// adds packages from there.
    ///
    /// # Returns
    /// None if the first part of the name didn't match this package's name. For example, if the
    /// name was "com.tom.example" and this package's name was "org.asd", this would return None,
    /// as org does not match com.
    pub fn add_subpackage(&mut self, name: &str) -> Option<&mut Package> {
        let (deepest, remaining) = self.find_pkg_mut(name);
        if deepest.is_none() {
            return None;
        };
        let deepest = deepest.unwrap();
        match remaining {
            None => return Some(deepest),
            Some(remaining) => {
                // add packages.                 
                let mut curr_pkg: *mut Package = deepest;
                for n in remaining.split(".") {
                    unsafe {
                        (*curr_pkg).package_list.push(Package::new(n).0);
                        curr_pkg = (*curr_pkg).package_list.last_mut().unwrap();
                    }
                }
                unsafe {
                    return Some(&mut *curr_pkg);
                }
            }
        }
    }

    /// Add a declaration to the package, creating subpackages where needed
    /// # Return
    /// True if decl was added, false if this package did not match
    pub fn add_decl(&mut self, pkg: &str, decl: Declaration) -> bool {
        let pkg = match self.add_subpackage(pkg) {
            None => return false,
            Some(p) => p,
        };
        pkg.decl_list.push(decl);
        return true;
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
    /// This might be self. If the package could not be found, None is returned as the second item
    /// in the tuple, containing the remaining name - if the package was found, None is returned.
    ///
    /// The string returned will not include the last package, for example, if we try and find the
    /// package com.tom.example, but only com.tom exists, this will return Some(example).
    /// 
    /// Will return None for the first part of the tuple if the first part of the package doesn't
    /// match this package's name.
    pub fn find_pkg<'a>(&self, name: &'a str) -> (Option<&Package>, Option<&'a str>) {
        use std::iter::Peekable;
        if name.len() == 0 {
            return (None, Some(name));
        }

        // Inner function to allow recursion.
        fn _find_pkg<'a, I: Iterator<Item = &'a str>>(
            mut splits: Peekable<I>,
            curr_pkg: *const Package,
        ) -> (*const Package, Option<&'a str>) {
            unsafe {
                let name = splits.next().unwrap();
                for p in &(*curr_pkg).package_list {
                    if p.name == name {
                        if splits.peek().is_some() {
                            return _find_pkg(splits, p);
                        } else {
                            return (p, None);
                        }
                    }
                }

                // Combine all remaining slices into one slice spanning them all
                let len = splits.fold(name.len(), |l, _s| l + _s.len() + 1);
                let ret =
                    std::str::from_utf8_unchecked(std::slice::from_raw_parts(name.as_ptr(), len));
                return (curr_pkg, Some(ret));
            }
        }

        let mut splits = name.split(".").peekable();
        if splits.next().unwrap() != self.name {
            return (None, Some(name));
        }
        if splits.peek().is_none() {
            return (Some(self), None);
        }
        let (pkg, found) = _find_pkg(splits, self);
        unsafe {
            return (Some(&*pkg), found); // Cast back to ref (rather than raw pointer)
        }
    }

    /// Find a package given a fully qualified name.
    /// If the package could not be found, return the package we 'made it to' whilst searching.
    /// This might be self. If the package could not be found, None is returned as the second item
    /// in the tuple, containing the remaining name - if the package was found, None is returned.
    ///
    /// The string returned will not include the last package, for example, if we try and find the
    /// package com.tom.example, but only com.tom exists, this will return Some(example).
    ///
    /// Will return None for the first part of the tuple if the first part of the package doesn't
    /// match this package's name.
    pub fn find_pkg_mut<'a>(&mut self, name: &'a str) -> (Option<&mut Package>, Option<&'a str>) {
        use std::iter::Peekable;
        if name.len() == 0 {
            return (None, Some(name));
        }

        // Inner function to allow recursion.
        fn _find_pkg<'a, I: Iterator<Item = &'a str>>(
            mut splits: Peekable<I>,
            curr_pkg: *mut Package,
        ) -> (*mut Package, Option<&'a str>) {
            unsafe {
                let name = splits.next().unwrap();
                for p in &mut (*curr_pkg).package_list {
                    if p.name == name {
                        if splits.peek().is_some() {
                            return _find_pkg(splits, p);
                        } else {
                            return (p, None);
                        }
                    }
                }

                // Combine all remaining slices into one slice spanning them all
                let len = splits.fold(name.len(), |l, _s| l + _s.len() + 1);
                let ret =
                    std::str::from_utf8_unchecked(std::slice::from_raw_parts(name.as_ptr(), len));
                return (curr_pkg, Some(ret));
            }
        }

        let mut splits = name.split(".").peekable();
        if splits.next().unwrap() != self.name {
            return (None, Some(name));
        }
        if splits.peek().is_none() {
            return (Some(self), None);
        }
        let (pkg, found) = _find_pkg(splits, self);
        unsafe {
            return (Some(&mut *pkg), found); // Cast back to mut ref (rather than raw pointer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use java_model::*;

    #[test]
    fn new_pkg() {
        let (p, deepest) = Package::new("com.tom.example");
        unsafe {
            assert!(deepest.is_some());
            assert_eq!((*deepest.unwrap()).name, "example");
            assert_eq!(p.name, "com");
            assert_eq!(p.package_list[0].name, "tom");
        }
        let (p, deepest) = Package::new("newpackage");
        assert!(deepest.is_none());
        assert_eq!(p.name, "newpackage");
    }

    #[test]
    fn find_pkg_and_find_package_mut() {
        let mut p = Package::new("com.tom").0;
        {
            let (deepest, remaining) = p.find_pkg("com.tom.example");
            assert_eq!(deepest, Some(&p.package_list[0]));
            assert_eq!(remaining, Some("example"));
        }
        {
            let (deepest, remaining) = p.find_pkg("com.tom");
            assert_eq!(deepest, Some(&p.package_list[0]));
            assert_eq!(remaining, None);
        }
        {
            let (deepest, remaining) = p.find_pkg("com");
            assert_eq!(deepest, Some(&p));
            assert_eq!(remaining, None);
        }
        {
            let (deepest, remaining) = p.find_pkg("asd");
            assert_eq!(deepest, None);
            assert_eq!(remaining, Some("asd"));
        }

        {
            let (deepest, remaining) = p.find_pkg_mut("com.tom.example");
            assert_eq!(deepest.unwrap().name, "tom");
            assert_eq!(remaining, Some("example"));
        }
        {
            let (deepest, remaining) = p.find_pkg_mut("com.tom");
            assert_eq!(deepest.unwrap().name, "tom");
            assert_eq!(remaining, None);
        }
        {
            let (deepest, remaining) = p.find_pkg_mut("com");
            assert_eq!(deepest.unwrap().name, "com");
            assert_eq!(remaining, None);
        }
        {
            let (deepest, remaining) = p.find_pkg_mut("asd");
            assert_eq!(deepest, None);
            assert_eq!(remaining, Some("asd"));
        }
    }

    #[test]
    fn add_subpackage() {
        let mut p = Package::new("com.tom").0;
        p.add_subpackage("com.tom.example");
        assert_eq!(p.package_list[0].package_list.len(), 1);
        assert_eq!(p.package_list[0].package_list[0].name, "example");
        p.add_subpackage("com.tom.foo");
        assert_eq!(p.package_list[0].package_list.len(), 2);
        assert_eq!(p.package_list[0].package_list[1].name, "foo");
        p.add_subpackage("com.john.foo");
        assert_eq!(p.package_list.len(), 2);
        assert_eq!(p.package_list[1].package_list.len(), 1);
        assert_eq!(p.package_list[1].package_list[0].name, "foo");
        assert_eq!(p.add_subpackage("asd"), None);
    }

    #[test]
    fn gen_package_completion_list() {
        let p = Package::new("com.tom.example").0;
        let completion_list = p.gen_package_completion_list();
        assert_eq!(completion_list, vec!["com", "com.tom", "com.tom.example"]);
    }

    #[test]
    fn gen_decl_completion_list() {
        let (p, deepest) = Package::new("com.tom.example");
        let deepest = deepest.unwrap();
        unsafe {
            (*deepest).decl_list.push(Declaration::Class(Class::new_with_name("MyClass")));
        }
        let completion_list = p.gen_decl_completion_list();
        assert_eq!(completion_list, vec!["com.tom.example.MyClass"]);
    }

    #[test]
    fn add_decl() {
        let mut p = Package::new("com.tom.example").0;
        p.add_decl("com.joe.test", Declaration::Class(Class::new_with_name("test")));
        assert_eq!(p.package_list.len(), 2);
        assert_eq!(p.package_list[1].package_list[0].name, "test");
    }
}
