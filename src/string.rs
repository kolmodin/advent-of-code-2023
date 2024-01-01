
pub fn words(s: &str) -> impl Iterator<Item = &str> {
    s.split(' ').filter(|s| !s.is_empty())
}