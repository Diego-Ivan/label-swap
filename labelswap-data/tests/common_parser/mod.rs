use labelswap_data::{models::annotation::Annotation, *};

use std::path::{Path, PathBuf};

pub fn test_annotation(parser: &mut impl parser::FormatParser, expected: Vec<Annotation>) {
    let mut annotations = Vec::new();
    while parser.has_next() {
        match parser.get_next() {
            Ok(annotation) => annotations.push(annotation),
            Err(e) => panic!("Failed to parse annotation: {e}"),
        }
    }
    assert_eq!(annotations, expected);
}

pub fn resolve_test_path(path: impl AsRef<Path>) -> PathBuf {
    let mut cwd = std::env::current_dir().expect("Could not get current directory");
    cwd.push(path);
    return cwd;
}
