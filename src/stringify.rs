use crate::geometry::{LabeledPoint, LabeledPoints, Point, Shape, ShapeAttrs, VisualShape};

pub fn convert_to_points(shape: &[&str], blank: &str) -> Vec<Point> {
    let mut points = Vec::new();

    for (y, line) in shape.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.to_string() != blank {
                points.push(Point { x: x as i16, y: y as i16 });
            }
        }
    }

    points
}

pub fn convert_to_labeled_points(shape: &[&str], width: usize) -> LabeledPoints<Point> {
    let mut res = Vec::new();

    for (y, line) in shape.iter().enumerate() {
        let row = line.len() / width;

        for x in 0..row {
            let start = x * width;
            let end = (x + 1) * width;
            let label = line[start..end].trim().to_string();

            if !label.is_empty() {
                let point = Point { x: x as i16, y: y as i16 };
                res.push(LabeledPoint { label, point });
            }
        }
    }

    res
}

pub fn bounds(ps: &[Point]) -> (Point, Point) {
    let (min_x, max_x, min_y, max_y) = ps.iter().fold(
        (i16::MAX, i16::MIN, i16::MAX, i16::MIN),
        |(min_x, max_x, min_y, max_y), p| {
            (min_x.min(p.x), max_x.max(p.x), min_y.min(p.y), max_y.max(p.y))
        },
    );

    (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
}

pub fn convert_to_strings(ps: &[Point], to_char: impl Fn(&Point) -> char) -> Vec<String> {
    let (min, max) = bounds(ps);
    let width = (max.x - min.x + 1) as usize;
    let height = (max.y - min.y + 1) as usize;

    let mut grid = vec![vec![' '; width]; height];

    for p in ps {
        grid[(p.y - min.y) as usize][(p.x - min.x) as usize] = to_char(p);
    }

    grid.iter().map(|cs| cs.iter().collect()).collect()
}

/*
// used to zero out shape coordinates so the first point is always (0, 0)
// which is important for the solver's iteration
fn subtract(p1:Point, p2:Point) -> Point {
    return Point {x: p1.x - p2.x, y: p1.y - p2.y}
}
pub fn convert_to_shape(vs: &VisualShape) -> Shape<Point> {
    let points = convert_to_points(&vs.points, "");
    let first = points[0];

    Shape {
        attrs: vs.attrs,
        points: points.iter().map(|p| subtract(*p, first)).collect(),
    }
}
*/