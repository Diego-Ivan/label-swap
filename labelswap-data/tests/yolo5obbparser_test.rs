use labelswap_data::models::{annotation::ClassRepresentation, Annotation, Image};
use labelswap_data::parser::*;
use std::path::{Path, PathBuf};

mod common_parser;

#[test]
fn test_io() {
    let mut parser = Yolo5ObbParser::new();
    parser
        .init(
            Path::new("/var/home/diegoivan/Escritorio/annotation-example/labelswap-data/tests/yolo5obb-labels"),
        )
        .unwrap();
    common_parser::test_annotation(
        &mut parser,
        vec![
            Annotation {
                class: ClassRepresentation::ClassName(String::from("small-vehicle")),
                difficulty: false,
                source_file: Some(PathBuf::from("/var/home/diegoivan/Escritorio/annotation-example/labelswap-data/tests/yolo5obb-labels/001.txt")),
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
                source_file: Some(PathBuf::from("/var/home/diegoivan/Escritorio/annotation-example/labelswap-data/tests/yolo5obb-labels/001.txt")),
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
                source_file: Some(PathBuf::from("/var/home/diegoivan/Escritorio/annotation-example/labelswap-data/tests/yolo5obb-labels/001.txt")),
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
                source_file: Some(PathBuf::from("/var/home/diegoivan/Escritorio/annotation-example/labelswap-data/tests/yolo5obb-labels/002.txt")),
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
