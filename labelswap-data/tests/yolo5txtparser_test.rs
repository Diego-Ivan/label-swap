mod common_parser;

use labelswap_data::models::{annotation::ClassRepresentation, *};
use labelswap_data::parser::*;
use std::path::{Path, PathBuf};
use labelswap_data::resolve_relative_path;

#[test]
fn test_io() {
    let mut parser = Yolo5TxtParser::new();
    let source_directory = common_parser::resolve_test_path("tests/yolo5txt-labels");
    parser.init(&source_directory).unwrap();
    common_parser::test_annotation(
        &mut parser,
        vec![
            Annotation {
                class: ClassRepresentation::ClassId(String::from("1")),
                difficulty: false,
                source_file: resolve_relative_path(&source_directory, "001.txt").ok(),
                image: Image::empty(),
                ..Annotation::from_centers(0.617, 0.3594420600858369, 0.114, 0.17381974248927037)
            },
            Annotation {
                class: ClassRepresentation::ClassId(String::from("1")),
                difficulty: false,
                source_file: resolve_relative_path(&source_directory, "001.txt").ok(),
                image: Image::empty(),
                ..Annotation::from_centers(0.094, 0.38626609442060084, 0.156, 0.23605150214592274)
            },
            Annotation {
                class: ClassRepresentation::ClassId(String::from("1")),
                difficulty: false,
                source_file: resolve_relative_path(&source_directory, "002.txt").ok(),
                image: Image::empty(),
                ..Annotation::from_centers(0.295, 0.3959227467811159, 0.13, 0.19527896995708155)
            },
            Annotation {
                class: ClassRepresentation::ClassId(String::from("1")),
                difficulty: false,
                source_file: resolve_relative_path(&source_directory, "002.txt").ok(),
                image: Image::empty(),
                ..Annotation::from_centers(0.785, 0.398068669527897, 0.07, 0.14377682403433475)
            },
        ],
    );
}
