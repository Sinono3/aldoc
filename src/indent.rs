pub fn remove_indent(text: &str, amount: u32) -> String {
    let mut string = String::new();

    for line in text.lines() {
        let mut to_remove = amount;
        
        for c in line.chars() {
            if c == '\t' && to_remove > 0 {
                to_remove -= 1;
                continue;
            }
            string.push(c);
        }
        string.push('\n');
    }
    return string;
}
pub fn base_indent(text: &str) -> u32 {
    let mut base_indent = 100; // TODO: wtf, why 100

    for line in text.lines() {
        let mut line_indent = 0;

        for _ in line.chars().filter(|c| *c == '\t') {
            line_indent += 1;
        }

        if base_indent > line_indent {
            base_indent = line_indent;
        }
    }
    return base_indent;
}
// makes the base indent of the string 0.
pub fn squash_indent(s: &str) -> String {
    remove_indent(s, base_indent(s))
}
