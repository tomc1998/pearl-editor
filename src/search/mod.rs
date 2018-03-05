//! Module containing algorithms / data structures for searching lists of stuff.

/// A buffer containing a list of searchable strings. Instead of being a simple Vec<String>, the
/// whole buffer is instead stored contiguously separated with null terminators, to allow for more
/// cache efficient subsequence searching.
///
/// NOTE: This will be a case INSENSITIVE search.
pub struct SearchBuffer {
    /// The buffer containing all the chars
    buf: String,
}

impl SearchBuffer {
    pub fn new() -> SearchBuffer {
        SearchBuffer { buf: String::new() }
    }

    /// Add a list of strings
    #[allow(dead_code)]
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
            self.buf.push_str(&s);
            self.buf.push_str("\x00");
        }
    }

    /// Clear this buffer.
    pub fn clear(&mut self) {
        self.buf.clear()
    }

    /// Given a string, return a list of string slices of which the input is a subsequence of.
    pub fn find_all_subsequences(&self, search: &str) -> Vec<&str> {
        use std::ascii::AsciiExt;
        let search = &search.to_lowercase();
        let mut results = Vec::with_capacity(1000);
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
                c.to_ascii_lowercase() ==
                    curr_search_iter.as_str().chars().next().unwrap()
            {
                curr_search_iter.next().unwrap();
            }
        }
        return results;
    }

    #[allow(dead_code)]
    pub fn find_all_containing_char(&self, search: char) -> Vec<&str> {
        use std::ascii::AsciiExt;
        let search = search.to_ascii_lowercase();
        let mut results = Vec::with_capacity(1000);
        let mut curr_base = 0; // Base ix of the curr string in buf
        let mut skipping = false;
        for (ii, c) in self.buf.char_indices() {
            if c == '\x00' {
                skipping = false;
                curr_base = ii + 1;
            }
            if skipping { continue; }
            // Not a null char, so check if we can advance curr_search_ix
            if search == c.to_ascii_lowercase() {
                unsafe {
                    results.push(self.buf.slice_unchecked(curr_base, ii));
                }
                skipping = true;
            }
        }
        return results;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    /** Test searching through a given amount of 30 char items */
    fn bench_search(b: &mut test::Bencher, num_classes: usize) {
        const SEARCH_TERM : &'static str = "Al£adfAff";
        let mut search_buffer = SearchBuffer::new();
        search_buffer.add_strings(&vec!["ALKJasdlf.£asdwhio.fASdhsdfo."; num_classes][..]);
        b.iter(|| search_buffer.find_all_subsequences(SEARCH_TERM));
    }

    #[bench]
    /** Test searching through enough data to overflow cache exactly (3mb) */
    fn bench_search_0_125x_cache(b: &mut test::Bencher) {
        bench_search(b, 12500);
    }

    #[bench]
    /** Test searching through enough data to overflow cache exactly (3mb) */
    fn bench_search_0_25x_cache(b: &mut test::Bencher) {
        bench_search(b, 25000);
    }

    #[bench]
    /** Test searching through enough data to overflow cache exactly (3mb) */
    fn bench_search_0_5x_cache(b: &mut test::Bencher) {
        bench_search(b, 50000);
    }


    #[bench]
    /** Test searching through enough data to overflow cache exactly (3mb) */
    fn bench_search_1x_cache(b: &mut test::Bencher) {
        bench_search(b, 100000);
    }

    #[bench]
    /** Test searching through enough data to overflow cache by 2x (6mb) */
    fn bench_search_2x_cache(b: &mut test::Bencher) {
        bench_search(b, 200000);
    }

    #[bench]
    /** Test searching through enough data to overflow cache by 3x (9mb) */
    fn bench_search_3x_cache(b: &mut test::Bencher) {
        bench_search(b, 300000);
    }

    #[bench]
    /** Test searching for a char through loads of 18 char items, then placing the results in a search
     * buffer */
    fn bench_char_search_1000_and_alloc(b: &mut test::Bencher) {
        let mut search_buffer = SearchBuffer::new();
        search_buffer.add_strings(&vec!["ABCDEFJALSKQOEMWND"; 50000][..]);
        b.iter(|| {
            let res = search_buffer.find_all_containing_char('n');
            let mut new_search_buffer = SearchBuffer::new();
            new_search_buffer.add_strings(&res[..]);
            test::black_box(new_search_buffer);
        });
    }
}
