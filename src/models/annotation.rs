#[derive(Debug, PartialEq)]
pub enum ClassRepresentation<S>
where
    S: Into<String>,
{
    None,
    ClassName(S),
    ClassId(S),
    Both(S, S),
}

#[derive(Debug, PartialEq)]
pub struct Annotation {
    pub class: ClassRepresentation<String>,
    pub source_file: String,
    pub difficulty: bool,

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
            source_file: String::new(),
            difficulty: false,
        }
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
        Self::new(x_min, y_min, x_max, y_min, x_max, y_max, x_min, y_max)
    }
}
