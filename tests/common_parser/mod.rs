use annotation_example::*;

pub fn test_annotation (parser: &impl parser::FormatParser, expected: Vec<Annotation>) {
    let mut annotations = Vec::new();

    while parser.has_next() {
        let annotation = parser.get_next();
        if let Err(e) = annotation {
            panic!("Failed to parse annotation: {e}");
        }
        annotations.push(annotation);
    }

    assert_eq!(annotation, expected);
}