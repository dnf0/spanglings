pub fn normalize(s: &str) -> String {
    let mut s = s.trim();
    if ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')))
        && s.len() >= 2
    {
        s = &s[1..s.len() - 1];
    }
    let mut s_str = s.trim().to_string();
    if s_str.starts_with('¿') {
        s_str.remove(0);
    }
    if s_str.starts_with('¡') {
        s_str.remove(0);
    }
    // Collapse multiple whitespace spaces into a single space
    let words: Vec<&str> = s_str.split_whitespace().collect();
    words.join(" ")
}
