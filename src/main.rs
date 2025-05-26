mod geometry;

use geometry::Point;

mod board;
use board::make_point_board;

mod stringify;
use geometry::VisualShape;
use stringify::convert_to_points;
use stringify::convert_to_shape;
use stringify::convert_to_strings;

fn print_board(points: &[Point], board: &board::Board<Point>) {
    let board_strs = convert_to_strings(points, |p| {
        if let Some(marker) = board.at(p) {
            marker.chars().next().unwrap_or(' ')
        } else {
            '-'
        }
    });

    println!("{}", board_strs.join("\n"));
}


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

    let mut board = make_point_board(points.clone());

    let shape = convert_to_shape(&VisualShape {
        points: vec![
            "**".to_string(),
            "*".to_string()
        ],
        attrs: geometry::ShapeAttrs {
            chiral: false,
            rotations: 0,
        },
    });

    for p in board.remaining() {
        println!("Remaining point: ({}, {})", p.x, p.y);
    }

    let res = board.fill(shape.points, "X");

    if (res.is_some()) {
        println!("Board after placing piece:");
        print_board(&points, &board);

        board.unfill(res.unwrap());

        println!("Board after removing piece:");
    }
    print_board(&points, &board);

    for p in board.remaining() {
        println!("Remaining point: ({}, {})", p.x, p.y);
    }
}
