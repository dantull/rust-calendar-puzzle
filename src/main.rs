mod geometry;

use geometry::Point;

mod board;
use board::make_point_board;

mod stringify;
use geometry::VisualShape;
use stringify::convert_to_labeled_points;
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

    let board_pts = convert_to_labeled_points(&[
        "Jan Feb Mar Apr May Jun",
        "Jul Aug Sep Oct Nov Dec",
        "  1   2   3   4   5   6   7 ",
        "  8   9  10  11  12  13  14 ",
        " 15  16  17  18  19  20  21 ",
        " 22  23  24  25  26  27  28 ",
        " 29  30  31 Sun Mon Tue Wed ",
        "                Thu Fri Sat ",
    ], 4);

    println!("Labeled Points:");
    for labeled_point in &board_pts {
        println!("Label: {}, Point: ({}, {})", labeled_point.label, labeled_point.point.x, labeled_point.point.y);
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

    if res.is_some() {
        println!("Board after placing piece:");
        print_board(&points, &board);

        let count = board.reachable(&Point{x: 2, y: 0}, 20);
        println!("Reachable points from (2, 0): {}", count);

        board.unfill(res.unwrap());

        println!("Board after removing piece:");

    }
    print_board(&points, &board);

    for p in board.remaining() {
        println!("Remaining point: ({}, {})", p.x, p.y);
    }
}
