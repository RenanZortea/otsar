use regex::Regex;

// transforms "$(text, style)" -> "<span class='style'>text</span>"
pub fn parse_custom_syntax(input: &str) -> String {
    // 1. Regex to find $(...) pattern
    // Explanation:
    // \$\(      -> Literal "$("
    // ([^,]+)   -> Capture Group 1: Anything that is NOT a comma (The content)
    // ,         -> Literal comma separator
    // \s* -> Optional whitespace
    // ([^)]+)   -> Capture Group 2: Anything that is NOT a closing parenthesis (The style)
    // \)        -> Literal ")"
    let re = Regex::new(r"\$\(([^,]+),\s*([^)]+)\)").unwrap();

    // 2. Replace with HTML span
    let html_output = re.replace_all(input, |caps: &regex::Captures| {
        let content = &caps[1].trim();
        let styles = &caps[2].trim();
        format!(r#"<span class="{}">{}</span>"#, styles, content)
    });

    html_output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let raw = "Hello $(world, text-red-500 font-bold)";
        let parsed = parse_custom_syntax(raw);
        assert_eq!(
            parsed,
            r#"Hello <span class="text-red-500 font-bold">world</span>"#
        );
    }
}
