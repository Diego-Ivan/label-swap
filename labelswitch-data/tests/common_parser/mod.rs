use labelswitch_data::{models::annotation::Annotation, *};

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
