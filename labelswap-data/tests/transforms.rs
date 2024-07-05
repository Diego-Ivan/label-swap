use std::{collections::HashSet, hash::Hash};

use labelswap_data::{models::format::*, transforms::RequiredTransformations};

#[test]
pub fn normalize_compatibility () {
    let source = Format {
        name: String::from("Dummy format 1"),
        id: String::from("dummy1"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::NoMapping,
        class_format: ClassFormat::Name,
        source_type: SourceType::SingleFile,
    };

    let target = Format {
        name: String::from("Dummy format 2"),
        id: String::from("dummy2"),
        file_extension: None,
        is_normalized: true,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::NoMapping,
        class_format: ClassFormat::Name,
        source_type: SourceType::SingleFile,
    };

    let source_compatible = Format {
        name: String::from("Dummy format 3"),
        id: String::from("dummy3"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::NoMapping,
        class_format: ClassFormat::Name,
        source_type: SourceType::SingleFile,
    };

    let compatibility = source.check_compatibility(&target);
    let mut expected = HashSet::new();
    expected.insert(RequiredTransformations::Normalize);

    assert_eq!(compatibility, expected);

    let compatibility = target.check_compatibility(&source);
    let mut expected = HashSet::new();
    expected.insert(RequiredTransformations::Denormalize);

    assert_eq!(compatibility, expected);

    let compatibility = source.check_compatibility(&source_compatible);
    assert_eq!(compatibility, HashSet::new());
}

#[test]
pub fn mapping_name_to_id() {
    // Test mapping name -> id without available mapping in source format
    let source = Format {
        name: String::from("Dummy format 1"),
        id: String::from("dummy1"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::NoMapping,
        class_format: ClassFormat::Name,
        source_type: SourceType::SingleFile,
    };

    let target_id = Format {
        name: String::from("Dummy format 2"),
        id: String::from("dummy2"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::ContainsMapping,
        class_format: ClassFormat::Id,
        source_type: SourceType::SingleFile,
    };

    let compatibility = source.check_compatibility(&target_id);
    let mut expected = HashSet::new();
    expected.insert(RequiredTransformations::MapToId);
    assert_eq!(compatibility, expected);

    // Test mapping name -> both when source format does not contain mapping
    let target = Format {
        name: String::from("Dummy format 2"),
        id: String::from("dummy2"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::ContainsMapping,
        class_format: ClassFormat::Both,
        source_type: SourceType::SingleFile,
    };

    let compatibility = source.check_compatibility(&target);
    let mut expected = HashSet::new();
    expected.insert(RequiredTransformations::MapToId);
    assert_eq!(compatibility, expected);


    // Test transformation from name -> id with mapping in source format
    let source = Format {
        name: String::from("Dummy format 1"),
        id: String::from("dummy1"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::ContainsMapping,
        class_format: ClassFormat::Name,
        source_type: SourceType::SingleFile,
    };

    let compatibility = source.check_compatibility(&target_id);
    assert_eq!(compatibility, HashSet::new());

    // Test name -> name with mapping
}

#[test]
pub fn mapping_id_to_name() {
    // Test mapping id -> name without available mapping in source format
    let source = Format {
        name: String::from("Dummy format 1"),
        id: String::from("dummy1"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::NoMapping,
        class_format: ClassFormat::Id,
        source_type: SourceType::SingleFile,
    };

    let target_name = Format {
        name: String::from("Dummy format 2"),
        id: String::from("dummy2"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::ContainsMapping,
        class_format: ClassFormat::Name,
        source_type: SourceType::SingleFile,
    };

    let compatibility = source.check_compatibility(&target_name);
    let mut expected = HashSet::new();
    expected.insert(RequiredTransformations::MapToName);
    assert_eq!(compatibility, expected);

    let target_both = Format {
        name: String::from("Dummy format 2"),
        id: String::from("dummy2"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::ContainsMapping,
        class_format: ClassFormat::Both,
        source_type: SourceType::SingleFile,
    };

    let compatibility = source.check_compatibility(&target_both);
    let mut expected = HashSet::new();
    expected.insert(RequiredTransformations::MapToName);
    assert_eq!(compatibility, expected);

    let source_mapping = Format {
        name: String::from("Dummy format 1"),
        id: String::from("dummy1"),
        file_extension: None,
        is_normalized: false,
        image_path: ImagePath::NoPath,
        class_mapping: ClassMapping::ContainsMapping,
        class_format: ClassFormat::Id,
        source_type: SourceType::SingleFile,
    };

    let compatibility = source_mapping.check_compatibility(&target_name);
    assert_eq!(compatibility, HashSet::new());

    let compatibility = source_mapping.check_compatibility(&target_both);
    assert_eq!(compatibility, HashSet::new());
}