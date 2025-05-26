#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug)]
pub struct LabeledPoint<P> {
    pub label: String,
    pub point: P,
}

pub type LabeledPoints<P> = Vec<LabeledPoint<P>>;

#[derive(Clone, Debug)]
pub struct ShapeAttrs {
    pub chiral: bool,
    pub rotations: u8, // Since Rust doesn't have union types, we'll use u8 to represent 0, 1, or 3 rotations
}

#[derive(Debug)]
pub struct VisualShape {
    pub points: Vec<String>, // Using Vec<String> to represent an array of strings
    pub attrs: ShapeAttrs, // Storing ShapeAttrs within VisualShape
}

#[derive(Debug)]
pub struct Shape<P> {
    pub points: Vec<P>, // Using Vec<P> to represent an array of points
    pub attrs: ShapeAttrs, // Storing ShapeAttrs within Shape
}
