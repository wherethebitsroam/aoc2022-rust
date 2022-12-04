pub fn read_lines_as_ints(s: &str) -> impl Iterator<Item = i32> + '_ {
    s.trim()
        .split('\n')
        .map(|s| s.parse().expect("not a number"))
}

pub fn read_lines(s: &str) -> impl Iterator<Item = &str> {
    s.trim().split('\n')
}
