// '///' -> 해당 블럭은 rust에서 테스트용 코드로 참조
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}
