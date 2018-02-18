//! Module containing algorithms / data structures for searching lists of stuff.

/// A buffer containing a list of searchable strings. Instead of being a simple Vec<String>, the
/// whole buffer is instead stored contiguously separated with null terminators, to allow for more
/// cache efficient subsequence searching.
pub struct SearchBuffer {
    /// The buffer containing all the chars
    buf: String,
}

impl SearchBuffer {
    pub fn new() -> SearchBuffer {
        SearchBuffer { buf: String::new() }
    }

    /// Add a list of strings
    pub fn add_strings(&mut self, strings: &[&str]) {
        let mut len = 0;
        for s in strings {
            len += s.len();
        }

        self.buf.reserve(len);
        for s in strings {
            self.buf.push_str(s);
            self.buf.push_str("\x00");
        }
    }

    /// Same as add_strings but with a slice of owned strings
    pub fn add_strings_owned(&mut self, strings: &[String]) {
        let mut len = 0;
        for s in strings {
            len += s.len();
        }

        self.buf.reserve(len);
        for s in strings {
            self.buf.push_str(s);
            self.buf.push_str("\x00");
        }
    }

    /// Clear this buffer.
    pub fn clear(&mut self) {
        self.buf.clear()
    }

    /// Given a string, return a list of string slices of which the input is a subsequence of.
    pub fn find_all_subsequences(&self, search: &str) -> Vec<&str> {
        let mut results = Vec::new();
        // Hold a reference into the search string, advance each time a char is a match
        let mut curr_search_iter = search.chars();
        let mut curr_base = 0; // Base ix of the curr string in buf
        for (ii, c) in self.buf.char_indices() {
            if c == '\x00' {
                // Null char is str boundary
                if curr_search_iter.as_str().len() == 0 {
                    unsafe {
                        results.push(self.buf.slice_unchecked(curr_base, ii));
                    }
                }
                curr_base = ii + 1;
                curr_search_iter = search.chars();
                continue;
            }
            // Not a null char, so check if we can advance curr_search_ix
            if curr_search_iter.as_str().len() > 0 &&
                c == curr_search_iter.as_str().chars().next().unwrap()
            {
                curr_search_iter.next().unwrap();
            }
        }
        return results;
    }
}
