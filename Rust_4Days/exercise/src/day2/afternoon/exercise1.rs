// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefixes = prefix.split('/');
    let request_paths = request_path
        .split('/')
        .map(|p| Some(p))
        .chain(std::iter::once(None));

    for (prefix, request_path) in prefixes.zip(request_paths) {
        match request_path {
            // match 값이 None이 아닐 경우
            Some(request_path) => {
                if (prefix != "*") && (prefix != request_path) {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
