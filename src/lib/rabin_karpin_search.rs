use memchr::memmem::Finder; // uses Rabin-Karp alg

pub fn rabin_karpin_search(text_input: &str, text_search: &str) -> Vec<usize> {
    let finder = Finder::new(text_search.as_bytes());
    return finder.find_iter(text_input.as_bytes()).collect();
}
