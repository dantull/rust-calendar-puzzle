mod geometry;
use std::collections::HashMap;

use geometry::Point;

mod board;
use board::make_point_board;

mod stringify;
use stringify::convert_to_points;
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

    let res = board.fill(vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }], "X");

    if (res.is_some()) {
        println!("Board after placing 2 square X piece:");
        print_board(&points, &board);

        board.unfill(res.unwrap());

        println!("Board after un-placing 2 square X piece:");
    }
    print_board(&points, &board);
}
