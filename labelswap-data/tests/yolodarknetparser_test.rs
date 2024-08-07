use labelswap_data::models::{annotation::ClassRepresentation, Annotation, Image};
use labelswap_data::parser::{YoloDarknetParser, FormatParser};
use std::path::Path;
use labelswap_data::resolve_relative_path;

mod common_parser;
#[test]
fn test_io() {
    let mut parser = YoloDarknetParser::new();
    let test_path = common_parser::resolve_test_path("tests/yolodarknet-labels");
    if let Err(e) = parser.init(&test_path) {
        panic!("An error ocurred trying to init the YOLO Darknet Parser: {e}");
    }

    common_parser::test_annotation(
        &mut parser,
        vec![
            Annotation {
                image: Image::empty(),
                class: ClassRepresentation::Both {
                    id: String::from("0"),
                    name: String::from("head"),
                },
                source_file: resolve_relative_path(&test_path, "001.txt").ok(),
                difficulty: false,
                ..Annotation::from_centers(0.408, 0.30266666666666664, 0.104,0.15733333333333333)
            },
            Annotation {
                image: Image::empty(),
                class: ClassRepresentation::Both {
                    id: String::from("1"),
                    name: String::from("helmet"),
                },
                source_file: resolve_relative_path(&test_path, "001.txt").ok(),
                difficulty: false,
                ..Annotation::from_centers(0.245, 0.424, 0.046, 0.08)
            },
            Annotation {
                image: Image::empty(),
                class: ClassRepresentation::Both {
                    id: String::from("1"),
                    name: String::from("helmet"),
                },
                source_file: resolve_relative_path(&test_path, "002.txt").ok(),
                difficulty: false,
                ..Annotation::from_centers(0.245, 0.424, 0.046, 0.1)
            },
        ],
    );
}
