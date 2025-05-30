mod geometry;

use geometry::Point;

mod board;
use board::make_point_board;

mod stringify;
use geometry::VisualShape;
mod solver;
use solver::create_solver;
use stringify::convert_to_labeled_points;
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
        println!("{} => ({}, {})", labeled_point.label, labeled_point.point.x, labeled_point.point.y);
    }

    let points: Vec<Point> = board_pts.iter().map(|lp| lp.point).collect();
    let board = make_point_board(points.clone());

    let l_piece = convert_to_shape(&VisualShape {
        points: vec![
            "***".to_string(),
            "*".to_string()
        ],
        attrs: geometry::ShapeAttrs {
            chiral: true,
            rotations: 3,
        },
    });

    let mut s = create_solver(board, vec![("L".to_string(), l_piece)]);

    fn handle_step_event(e: solver::StepEvent, b: &board::Board<Point>) {
        match e {
            solver::StepEvent::FailedToPlace => println!("Failed to place shape"),
            solver::StepEvent::Placed => println!("Shape placed successfully"),
            solver::StepEvent::Solved => {
                println!("Solved!");
                print_board(&b.all, b);
            }
        }
    }

    while solver::step(&mut s, handle_step_event) {
        // Continue stepping until no more steps can be taken
    }
    }
