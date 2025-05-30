use crate::board::Board;
use crate::geometry::variants;
use crate::geometry::Point;
use crate::geometry::Shape;

struct ShapeState<P: Clone> {
    point_index: usize,
    variant_index: usize,
    remove: Option<Vec<Point>>,
    places: usize,
    label: String,
    points: Vec<P>,
}

fn new_shape_state(label: String, ps: Vec<Point>) -> ShapeState<Point> {
    ShapeState {
        point_index: 0,
        variant_index: 0,
        remove: None,
        places: 0,
        label: label.to_string(),
        points: ps,
    }
}

fn step_state(state: &mut ShapeState<Point>, board: &mut Board<Point>, base_variants: &Vec<Vec<Point>>, min_size: usize) -> bool {
    if let Some(remove) = state.remove.take() {
        board.unfill(remove);
        state.remove = None;
    }

    if state.variant_index < base_variants.len() {
        state.remove = board.fill(
            &base_variants[state.variant_index],
            state.points[state.point_index],
            &state.label,
        );

        if state.remove.is_some() {
            state.places += 1;
        }

        state.variant_index += 1;
    } else if state.variant_index == base_variants.len() {
        let p = state.points[state.point_index];

        if board.reachable(&p, min_size) < min_size {
            state.point_index = state.points.len();
        } else {
            state.point_index += 1;
        }
        state.variant_index = 0;
    }

    return state.point_index < state.points.len();
}

fn is_placed<P: Clone>(state: &ShapeState<P>) -> bool {
    state.remove.is_some()
}

fn never_placed<P: Clone>(state: &ShapeState<P>) -> bool {
    state.places == 0
}

pub struct Solver<P: Clone> {
    board: Board<P>,
    labeled_shapes: Vec<(String, Shape<P>)>,
    shape_states: Vec<ShapeState<P>>,
    base_variants: Vec<Vec<Vec<Point>>>,
    min_size: usize,
}

fn next_shape_state(solver: &Solver<Point>) -> ShapeState<Point> {
    let i = solver.shape_states.len();
    new_shape_state(
        solver.labeled_shapes[i].0.clone(),
        solver
            .board
            .remaining()
            .iter()
            .map(|p| (*p).clone())
            .collect(),
    )
}

pub fn create_solver(b: Board<Point>, shapes: Vec<(String, Shape<Point>)>) -> Solver<Point> {
    let count = shapes.len();
    let min_size = shapes.iter().map(|(_, shape)| shape.points.len()).min().unwrap();
    let base_variants = shapes.iter().map(|(_, shape)| {
        variants(shape)
    }).collect::<Vec<_>>();
    let mut solver = Solver {
        board: b,
        labeled_shapes: shapes,
        shape_states: Vec::with_capacity(count),
        base_variants,
        min_size,
    };

    let state = next_shape_state(&solver);
    solver.shape_states.push(state);
    solver
}

pub enum StepEvent {
    FailedToPlace,
    Placed,
    Solved,
}

pub fn step<F>(solver: &mut Solver<Point>, mut callback: F) -> bool
where
    F: FnMut(StepEvent, &Board<Point>),
{
    if solver.shape_states.is_empty() {
        return false; // No shapes to place
    }

    let i = solver.shape_states.len() - 1;
    let state = solver.shape_states.last_mut().unwrap();

    let more = step_state(state, &mut solver.board, &solver.base_variants[i], solver.min_size);
    if !more && is_placed(state) {
        callback(StepEvent::Placed, &solver.board);
    }

    if !more {
        if never_placed(state) {
            callback(StepEvent::FailedToPlace, &solver.board);
        }

        solver.shape_states.pop();
        !solver.shape_states.is_empty()
    } else {
        if is_placed(state) {
            let solved = solver.shape_states.len() == solver.labeled_shapes.len();

            callback(
                if solved {
                    StepEvent::Solved
                } else {
                    StepEvent::Placed
                },
                &solver.board,
            );

            if !solved {
                let state = next_shape_state(&solver);
                solver.shape_states.push(state);
            }
        }
        true
    }
}
