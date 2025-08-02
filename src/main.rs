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
    let board_pts = convert_to_labeled_points(
        &[
            "Jan Feb Mar Apr May Jun ",
            "Jul Aug Sep Oct Nov Dec ",
            "  1   2   3   4   5   6   7 ",
            "  8   9  10  11  12  13  14 ",
            " 15  16  17  18  19  20  21 ",
            " 22  23  24  25  26  27  28 ",
            " 29  30  31 Sun Mon Tue Wed ",
            "                Thu Fri Sat ",
        ],
        4,
    );

    let points: Vec<Point> = board_pts.iter().map(|lp| lp.point).collect();
    let mut board = make_point_board(points);

    let l_piece = convert_to_shape(&VisualShape {
        points: vec![
            "***".to_string(), //
            "*".to_string(),   //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: true,
            rotations: 3,
        },
    });

    let j_piece = convert_to_shape(&VisualShape {
        points: vec![
            "****".to_string(), //
            "*".to_string(),    //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: true,
            rotations: 3,
        },
    });

    let i_piece = convert_to_shape(&VisualShape {
        points: vec!["****".to_string()],
        attrs: geometry::ShapeAttrs {
            chiral: false,
            rotations: 1,
        },
    });

    let p_piece = convert_to_shape(&VisualShape {
        points: vec![
            "***".to_string(), //
            "**".to_string(),  //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: true,
            rotations: 3,
        },
    });

    let n_piece = convert_to_shape(&VisualShape {
        points: vec![
            "**".to_string(),   //
            " ***".to_string(), //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: true,
            rotations: 3,
        },
    });

    let u_piece = convert_to_shape(&VisualShape {
        points: vec![
            "* *".to_string(), //
            "***".to_string(), //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: false,
            rotations: 3,
        },
    });

    let t_piece = convert_to_shape(&VisualShape {
        points: vec![
            "***".to_string(), //
            " * ".to_string(), //
            " * ".to_string(), //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: false,
            rotations: 3,
        },
    });

    let v_piece = convert_to_shape(&VisualShape {
        points: vec![
            "***".to_string(), //
            "*".to_string(),   //
            "*".to_string(),   //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: false,
            rotations: 3,
        },
    });

    let z_piece = convert_to_shape(&VisualShape {
        points: vec![
            "**".to_string(),  //
            " *".to_string(),  //
            " **".to_string(), //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: true,
            rotations: 1,
        },
    });

    let s_piece = convert_to_shape(&VisualShape {
        points: vec![
            "**".to_string(),  //
            " **".to_string(), //
        ],
        attrs: geometry::ShapeAttrs {
            chiral: true,
            rotations: 1,
        },
    });

    let origin = Point { x: 0, y: 0 };
    let mut count = 0;
    let mut verbose = false;
    let args: Vec<String> = std::env::args().collect();
    let mut goal = 1;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "-m" && i + 1 < args.len() {
            if let Ok(val) = args[i + 1].parse::<usize>() {
                goal = val;
            } else {
                eprintln!("Invalid value for -m: {}", args[i + 1]);
                std::process::exit(1);
            }
            i += 1;
        } else if args[i] == "-v" {
            verbose = true;
        } else {
            // Try to match argument to a labeled point and fill it in the board
            let label = &args[i];
            if let Some(lp) = board_pts.iter().find(|lp| lp.label == *label) {
                let ps = lp.point;
                board.fill(&vec![ps], origin, "*");

                if verbose {
                    println!("Filled label '{}' at point {:?}", label, ps);
                    print_board(&board.all, &board);
                    println!();
                }
            } else {
                eprintln!("Label '{}' not found.", label);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let mut s = create_solver(
        board,
        vec![
            ("Z".to_string(), z_piece),
            ("V".to_string(), v_piece),
            ("U".to_string(), u_piece),
            ("T".to_string(), t_piece),
            ("P".to_string(), p_piece),
            ("N".to_string(), n_piece),
            ("L".to_string(), l_piece),
            ("J".to_string(), j_piece),
            ("I".to_string(), i_piece),
            ("S".to_string(), s_piece),
        ],
    );

    let mut handle_step_event = |e: solver::StepEvent, b: &board::Board<Point>| match e {
        solver::StepEvent::FailedToPlace => (),
        solver::StepEvent::Placed => {
            if verbose {
                println!("Placed:");
                print_board(&b.all, b);
                println!();
            }
        }
        solver::StepEvent::Solved => {
            println!("Solved!");
            print_board(&b.all, b);

            count += 1;
            if count >= goal {
                println!("Reached goal of {} solutions.", goal);
                std::process::exit(0);
            }
        }
    };

    while solver::step(&mut s, &mut handle_step_event) {
        // Continue stepping until no more steps can be taken
    }
}
