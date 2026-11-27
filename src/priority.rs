use crate::flag::LineNumberf;

// I will eventually replace this with an enum
pub fn line_number(n: bool, b: bool) -> LineNumberf {
    if n && !b {
        return LineNumberf::Number;
    }
    if !n && b || n && b {
        return LineNumberf::Blank;
    }
    return LineNumberf::No;
}
