pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn contains_pattern(src: &str, pat: &str, ignore_case: bool, word_regexp: bool) -> bool {
    let src = if ignore_case { src.to_lowercase() } else { src.to_owned() };
    let pat = if ignore_case { pat.to_lowercase() } else { pat.to_owned() };

    if word_regexp {
        src.split_whitespace().any(|p| p == pat)
    } else {
        src.contains(&pat)
    }
}
