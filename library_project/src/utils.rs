fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

pub fn titles_match(a: &str, b: &str) -> bool {
    normalize(a) == normalize(b)
}
