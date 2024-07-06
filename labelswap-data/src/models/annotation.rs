use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ClassRepresentation<S>
where
    S: Into<String>,
{
    None,
    ClassName(S),
    ClassId(S),
    // ClassName, ClassRepresentation
    Both { name: S, id: S },
}

impl<S: PartialEq + Into<String>> PartialEq for ClassRepresentation<S> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ClassName(s), Self::ClassName(other)) => s == other,
            (Self::ClassId(s), Self::ClassId(o)) => s == o,
            (
                Self::Both {
                    name: s_name,
                    id: s_id,
                },
                Self::Both {
                    name: o_name,
                    id: o_id,
                },
            ) => s_name == o_name && s_id == o_id,
            _ => false,
        }
    }
}

impl ClassRepresentation<String> {
    pub const fn as_ref(&self) -> ClassRepresentation<&String> {
        match *self {
            Self::None => ClassRepresentation::None,
            Self::ClassName(ref s) => ClassRepresentation::ClassName(s),
            Self::ClassId(ref s) => ClassRepresentation::ClassId(s),
            Self::Both { ref name, ref id } => ClassRepresentation::Both {
                name: &name,
                id: &id,
            },
        }
    }

    pub fn to_both(self, missing_value: String) -> ClassRepresentation<String> {
        match self {
            Self::None => Self::None,
            Self::ClassName(name) => Self::Both { name, id: missing_value },
            Self::ClassId(id) => Self::Both{ id, name: missing_value},
            Self::Both { name, id } => Self::Both { name, id },
        }
    }
}

///
/// The Annotation struct represents an annotation in a image. It uses oriented
/// bounding boxes to represent its contents. The current bounding box
/// format is the following:
///                                        
/// x1, y1                       x2, y2    
/// ┌────────────────────────────────┐   
/// │                                │   
/// │                                │   
/// │                                │   
/// │                                │   
/// │                                │   
/// └────────────────────────────────┘   
/// x4, y4                        x3, y3   
///
/// The Annotation struct is usually returned by implementations of FormatParser
/// and used as inputs for FormatSerializer implementations.
#[derive(Debug, PartialEq)]
pub struct Annotation {
    pub class: ClassRepresentation<String>,
    pub source_file: Option<PathBuf>,
    pub difficulty: bool,
    pub image: Option<PathBuf>,

    pub x1: f64,
    pub x2: f64,
    pub x3: f64,
    pub x4: f64,
    pub y1: f64,
    pub y2: f64,
    pub y3: f64,
    pub y4: f64,
}

impl Annotation {
    pub fn new(
        x1: f64,
        x2: f64,
        x3: f64,
        x4: f64,
        y1: f64,
        y2: f64,
        y3: f64,
        y4: f64,
    ) -> Annotation {
        Annotation {
            x1,
            x2,
            x3,
            x4,
            y1,
            y2,
            y3,
            y4,
            class: ClassRepresentation::None,
            source_file: None,
            difficulty: false,
            image: None,
        }
    }

    // Assumes that the origin is in the top left corner
    pub fn from_top_left_corner(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::from_min_max(x, x + width, y, y + height)
    }

    pub fn from_centers(center_x: f64, center_y: f64, width: f64, height: f64) -> Annotation {
        let distance_x = width / 2.0;
        let distance_y = height / 2.0;
        let x1 = center_x - distance_x;
        let x2 = center_x + distance_x;
        let x3 = center_x - distance_x;
        let x4 = center_x + distance_x;

        let y1 = center_y - distance_y;
        let y2 = center_y - distance_y;
        let y3 = center_y + distance_y;
        let y4 = center_y + distance_y;

        Self::new(x1, x2, x3, x4, y1, y2, y3, y4)
    }

    pub fn from_min_max(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Annotation {
        Self::new(x_min, x_max, x_max, x_min, y_min, y_min, y_max, y_max)
    }
}
