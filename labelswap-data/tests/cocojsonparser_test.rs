use labelswap_data::models::{annotation::ClassRepresentation, Annotation, Image};
use labelswap_data::parser::{CocoJsonParser, FormatParser};
use std::path::Path;

mod common_parser;
#[test]
fn test_io() {
    let mut parser = CocoJsonParser::new();
    if let Err(e) = parser.init(Path::new("/var/home/diegoivan/Escritorio/annotation-example/labelswap-data/tests/cocojson-labels/coco.json")) {
        panic!("An error ocurred trying to init Coco JSON: {e}");
    }

    common_parser::test_annotation(
        &mut parser,
        vec![
            Annotation {
                image: Image::new_with_path(Path::new("0001.jpg")),
                class: ClassRepresentation::Both {
                    id: String::from("2"),
                    name: String::from("helmet"),
                },
                source_file: None,
                difficulty: false,
                x1: 45.0,
                x2: 45.0 + 85.0,
                x3: 45.0 + 85.0,
                x4: 45.0,
                y1: 2.0,
                y2: 2.0,
                y3: 2.0 + 85.0,
                y4: 2.0 + 85.0,
            },
            Annotation {
                image: Image::new_with_path(Path::new("0001.jpg")),
                class: ClassRepresentation::Both {
                    id: String::from("2"),
                    name: String::from("helmet"),
                },
                source_file: None,
                difficulty: false,
                x1: 324.0,
                x2: 324.0 + 72.0,
                x3: 324.0 + 72.0,
                x4: 324.0,
                y1: 29.0,
                y2: 29.0,
                y3: 29.0 + 81.0,
                y4: 29.0 + 81.0,
            },
        ],
    );
}
