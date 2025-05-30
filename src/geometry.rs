#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
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
    pub attrs: ShapeAttrs,   // Storing ShapeAttrs within VisualShape
}

#[derive(Debug, Clone)]
pub struct Shape<P: Clone> {
    pub points: Vec<P>,    // Using Vec<P> to represent an array of points
    pub attrs: ShapeAttrs, // Storing ShapeAttrs within Shape
}

type Mapper = fn(Point) -> Point;

fn flip_point(p: Point) -> Point {
    Point { x: -p.x, y: p.y }
}

fn identity(p: Point) -> Point {
    p
}

const ROTATES: [Mapper; 4] = [
    identity,                              // 0 degrees rotation
    |p: Point| Point { x: -p.y, y: p.x },  // 90 degrees rotation
    |p: Point| Point { x: -p.x, y: -p.y }, // 180 degrees rotation
    |p: Point| Point { x: p.y, y: -p.x },  // 270 degrees rotation
];

pub fn variants(shape: &Shape<Point>) -> Vec<Vec<Point>> {
    let flips: Vec<Mapper> = if shape.attrs.chiral {
        vec![identity, flip_point]
    } else {
        vec![identity]
    };

    let mut vs = Vec::with_capacity(shape.attrs.rotations as usize * flips.len());

    for &flip in &flips {
        for i in 0..=shape.attrs.rotations {
            let rotate = ROTATES[(i as usize) % 4];
            let v: Vec<Point> = shape
                .points
                .iter()
                .map(|&p| {
                    let pt: Point = p.into();
                    rotate(flip(pt))
                })
                .collect();
            vs.push(v);
        }
    }

    vs
}
