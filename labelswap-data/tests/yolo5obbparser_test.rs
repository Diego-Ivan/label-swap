use labelswap_data::models::{annotation::ClassRepresentation, Annotation, Image};
use labelswap_data::parser::*;
use labelswap_data::resolve_relative_path;
use std::path::{Path, PathBuf};

mod common_parser;

#[test]
fn test_io() {
    let mut parser = Yolo5ObbParser::new();
    let source_directory = common_parser::resolve_test_path("tests/yolo5obb-labels");
    parser.init(&source_directory).unwrap();
    common_parser::test_annotation(
        &mut parser,
        vec![
            Annotation {
                class: ClassRepresentation::ClassName(String::from("small-vehicle")),
                difficulty: false,
                source_file: resolve_relative_path(&source_directory, "001.txt").ok(),
                image: Image::new(),
                x1: 287.0,
                y1: 268.0,
                x2: 282.0,
                y2: 268.0,
                x3: 282.0,
                y3: 279.0,
                x4: 287.0,
                y4: 279.0,
            },
            Annotation {
                class: ClassRepresentation::ClassName(String::from("large-vehicle")),
                difficulty: false,
                source_file: resolve_relative_path(&source_directory, "001.txt").ok(),
                image: Image::new(),
                x1: 212.00000000000006,
                y1: 285.0,
                x2: 195.00000000000006,
                y2: 285.0,
                x3: 195.00000000000006,
                y3: 293.0,
                x4: 212.00000000000006,
                y4: 293.0,
            },
            Annotation {
                class: ClassRepresentation::ClassName(String::from("large-vehicle")),
                difficulty: false,
                source_file: resolve_relative_path(&source_directory, "001.txt").ok(),
                image: Image::new(),
                x1: 167.0,
                y1: 151.0,
                x2: 149.0,
                y2: 151.0,
                x3: 149.0,
                y3: 158.0,
                x4: 167.0,
                y4: 158.0,
            },
            Annotation {
                class: ClassRepresentation::ClassName(String::from("large-vehicle")),
                difficulty: false,
                source_file: resolve_relative_path(&source_directory, "002.txt").ok(),
                image: Image::new(),
                x1: 167.0,
                y1: 151.0,
                x2: 149.0,
                y2: 151.0,
                x3: 149.0,
                y3: 158.0,
                x4: 167.0,
                y4: 158.0,
            },
        ],
    );
}
