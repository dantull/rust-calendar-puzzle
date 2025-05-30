use crate::board::Board;
use crate::geometry::Point;
use crate::geometry::Shape;
use crate::geometry::variants;

struct ShapeState<P: Clone> {
	point_index: usize,
	variant_index: usize,
	remove: Option<Vec<i16>>,
	places: usize,
	label: String,
	base_variants: Vec<Vec<P>>,
	points: Vec<P>,
}

fn new_shape_state(shape: Shape<Point>, label: String, ps: Vec<Point>) -> ShapeState<Point> {
    let base_variants = variants(&shape);
    ShapeState {
        point_index: 0,
        variant_index: 0,
        remove: None,
        places: 0,
        label: label.to_string(),
        base_variants,
        points: ps,
    }
}

fn step_state(state: &mut ShapeState<Point>, board: &mut Board<Point>, min_size: usize) -> bool {
    if let Some(remove) = state.remove.take() {
        board.unfill(remove);
        state.remove = None;
    }

    if state.variant_index < state.base_variants.len() {
        state.remove = board.fill(
            state.base_variants[state.variant_index].clone(),
            state.points[state.point_index].clone(),
            &state.label.clone(),
        );
        
        if state.remove.is_some() {
            state.places += 1;
        }

        state.variant_index += 1;
    } else if state.variant_index == state.base_variants.len() {
        let p = state.points[state.point_index].clone();

        if board.reachable(&p, min_size) < min_size {
            state.point_index = state.points.len();
        } else {
            state.point_index += 1;
        }
        state.variant_index = 0;
    }

    return state.point_index < state.points.len()
}

fn is_placed<P: Clone>(state: &ShapeState<P>) -> bool {
    state.remove.is_some()   
}

fn never_placed<P: Clone>(state: &ShapeState<P>) -> bool {
    state.places == 0
}

pub struct Solver<P: Clone> {
    board: Board<P>,
    all_labels: Vec<String>,
    all_shapes: Vec<Shape<P>>,
    shape_states: Vec<ShapeState<P>>,
    min_size: usize,
}

fn next_shape_state(
    solver: &Solver<Point>,
) -> ShapeState<Point> {
    let i = solver.shape_states.len();
    new_shape_state(
        solver.all_shapes[i].clone(),
        solver.all_labels[i].clone(),
        solver.board.remaining().iter().map(|p| (*p).clone()).collect(),
    )
}

pub fn create_solver(b: Board<Point>, shapes: Vec<(String, Shape<Point>)>) -> Solver<Point> {
    let mut solver = Solver {
        board: b,
        all_labels: shapes.iter().map(|(label, _)| label.clone()).collect(),
        all_shapes: shapes.iter().map(|(_, shape)| shape.clone()).collect(),
        shape_states: Vec::with_capacity(shapes.len()),
        min_size: 3, // FIXME: this should be determined by inspecting the shapes
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

pub fn step(solver: &mut Solver<Point>, callback: fn(e: StepEvent, b: &Board<Point>)) -> bool {
    if solver.shape_states.is_empty() {
        return false; // No shapes to place
    }

    let state = solver.shape_states.last_mut().unwrap();

    let more = step_state(state, &mut solver.board, solver.min_size);   

    if !more {
        if never_placed(state) {
            callback(StepEvent::FailedToPlace, &solver.board);
        }

        solver.shape_states.pop();
        !solver.shape_states.is_empty()
    } else {
        if is_placed(state) {
            let solved = solver.shape_states.len() == solver.all_labels.len();

            callback(if solved {
                StepEvent::Solved
            } else {
                StepEvent::Placed
            }, &solver.board);

            if !solved {
                let state =next_shape_state(&solver); 
                solver.shape_states.push(state);
            }
        }
        true
    }
    
}