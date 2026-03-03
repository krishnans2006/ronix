pub fn escape_nix_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '$' => {
                // Escape ${ to prevent Nix interpolation
                out.push_str("\\$");
            }
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

pub(crate) fn indent_str(level: usize) -> String {
    "  ".repeat(level)
}

pub(crate) fn format_nix_float(v: f64) -> String {
    let s = format!("{}", v);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}
