use crate::board::Board;

use crate::geometry::Point;
use crate::geometry::Shape;

use crate::solver::create_solver;
use crate::solver::step;
use crate::solver::Solver;
use crate::solver::StepEvent;

pub fn create_parallel_solver(
    board: Board<Point>,
    pieces: Vec<(String, Shape<Point>)>,
    n: usize,
) -> Vec<Solver<Point>> {
    let mut solvers = Vec::new();

    let (first_n, remainder) = pieces.split_at(n);
    let first_n_vec = first_n.to_vec();
    let remainder_vec = remainder.to_vec();

    let mut solver = create_solver(board.clone(), first_n_vec);

    let mut handle_step_event = |e: StepEvent, b: &Board<Point>| match e {
        StepEvent::FailedToPlace => (),
        StepEvent::Placed => (),
        StepEvent::Solved => {
            let new_solver = create_solver(b.clone(), remainder_vec.clone());
            solvers.push(new_solver);
        }
    };

    while step(&mut solver, &mut handle_step_event) {
        // Continue stepping until no more steps can be taken
    }

    solvers
}
