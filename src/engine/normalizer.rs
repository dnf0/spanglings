pub fn normalize(s: &str) -> String {
    let s = s.trim();

    // Define characters to strip from start and end
    let start_chars = |c: char| {
        c.is_whitespace()
            || c == '¿'
            || c == '¡'
            || c == '"'
            || c == '\''
            || c == '«'
            || c == '»'
            || c == '“'
            || c == '”'
            || c == '‘'
            || c == '’'
    };

    let end_chars = |c: char| {
        c.is_whitespace()
            || c == '?'
            || c == '!'
            || c == '.'
            || c == ','
            || c == ';'
            || c == ':'
            || c == '"'
            || c == '\''
            || c == '«'
            || c == '»'
            || c == '“'
            || c == '”'
            || c == '‘'
            || c == '’'
    };

    let s = s
        .trim_start_matches(start_chars)
        .trim_end_matches(end_chars);

    // Collapse multiple whitespace spaces into a single space
    let words: Vec<&str> = s.split_whitespace().collect();
    words.join(" ")
}
