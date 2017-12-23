type CoordType = isize;
type ThicknessType = usize;

#[derive(Debug)]
pub struct Point {
    pub x: CoordType,
    pub y: CoordType,
}

#[derive(Debug)]
pub enum GraphicElement {
    Polygon {
        points: Vec<Point>,
        unit: usize,
        convert: usize,
        thickness: usize,
        // TODO: parts, convert, filled, not filled
    },
    Rectangle {
        start: Point,
        end: Point,
        unit: usize,
        convert: usize,
        // TODO: parts, convert, filled
    },
    Circle {
        center: Point,
        radius: ThicknessType,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool
    },
    CircleArc {
        center: Point,
        radius: ThicknessType,
        start_coord: Point,
        end_coord: Point,
        start_angle: isize,
        end_angle: isize,
        unit: usize,
        convert: usize,
        thickness: usize,
        filled: bool
    },
    TextField {
        content: String,
        orientation: TextOrientation,
        position: Point,
        unit: usize,
        convert: usize,
        // TODO: parts, convert, filled
    },
    Pin {
        orientation: PinOrientation,
        name: Option<String>,
        number: usize,
        position: Point,
        length: usize,
        number_size: usize,
        name_size: usize,
        invisible: bool,
    }
}

#[derive(Debug)]
pub enum TextOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum PinDescription {
}

#[derive(Debug, PartialEq)]
pub enum PinOrientation {
    Up,
    Down,
    Right,
    Left,
}