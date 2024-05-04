mod geometry;
use std::collections::HashMap;

use geometry::Point;

mod board;
use board::make_point_board;

mod stringify;
use stringify::convert_to_points;
use stringify::convert_to_strings;

fn main() {
    let shape = vec![
        "****",
        "*  *",
        "*  *",
        "****",
    ];

    let points = convert_to_points(&shape, " ");

    // Print the points
    for point in &points {
        println!("({}, {})", point.x, point.y);
    }

    let strs = convert_to_strings(&points, |_| 'x');

    let result = strs.join("\n");
    println!("{}", result);

    let mut board = make_point_board((points));
}
