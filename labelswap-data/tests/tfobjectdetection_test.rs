use labelswap_data::models::{annotation::ClassRepresentation, Annotation, Image};
use labelswap_data::parser::{TfObjectDetectionParser, FormatParser};
use std::path::PathBuf;

mod common_parser;

#[test]
fn test_io() {
    let mut parser = TfObjectDetectionParser::new();
    let source_file = common_parser::resolve_test_path("tests/tfcsv-labels/tensorflow.csv");

    if let Err(e) = parser.init(&source_file) {
        panic!("An error ocurred trying to init TfObjectDetectionParser: {e}");
    }

    common_parser::test_annotation(
        &mut parser,
        vec![
            Annotation {
                image: Image {
                    path: Some(PathBuf::from("000001.jpg")),
                    width: Some(500),
                    height: Some(375),
                },
                class: ClassRepresentation::ClassName(String::from("helmet")),
                source_file: Some(PathBuf::from(&source_file)),
                difficulty: false,
                ..Annotation::from_min_max(111.0, 134.0, 144.0, 174.0)
            },
            Annotation {
                image: Image {
                    path: Some(PathBuf::from("000002.jpg")),
                    width: Some(250),
                    height: Some(450),
                },
                class: ClassRepresentation::ClassName(String::from("helmet")),
                source_file: Some(PathBuf::from(&source_file)),
                difficulty: false,
                ..Annotation::from_min_max(178.0, 230.0, 84.0, 143.0)
            },
            Annotation {
                image: Image {
                    path: Some(PathBuf::from("000003.jpg")),
                    width: Some(1123),
                    height: Some(543),
                },
                class: ClassRepresentation::ClassName(String::from("helmet")),
                source_file: Some(PathBuf::from(&source_file)),
                difficulty: false,
                ..Annotation::from_min_max(280.0, 337.0, 127.0, 208.0)
            },
            Annotation {
                image: Image {
                    path: Some(PathBuf::from("000006.jpg")),
                    width: Some(500),
                    height: Some(466),
                },
                class: ClassRepresentation::ClassName(String::from("helmet")),
                source_file: Some(PathBuf::from(&source_file)),
                difficulty: false,
                ..Annotation::from_min_max(336.0, 387.0, 148.0, 223.0)
            },
        ],
    );
}
