use std::collections::{HashMap, HashSet};

// Importing the Point struct from the geometry module
use crate::Point;

// Binary operation type alias
type BinaryOperation<P> = fn(&P, &P) -> P;

// Structure representing the Board
pub struct Board<P> {
    filled: HashMap<P, String>,
    unfilled: HashSet<P>,
    pub all: Vec<P>,
    adder: BinaryOperation<P>,
    dirs: Vec<P>,
}

impl<P> Board<P>
where
    P: Copy + Eq + std::hash::Hash,
{
    // Constructor
    pub fn new(ps: Vec<P>, adder: BinaryOperation<P>, dirs: Vec<P>) -> Self {
        let mut unfilled = HashSet::new();
        for p in &ps {
            unfilled.insert(*p);
        }

        Board {
            filled: HashMap::new(),
            unfilled,
            all: ps,
            adder,
            dirs,
        }
    }

    // Private method to recursively spread and collect reachable points
    fn spread(&self, p: &P, limit: usize, accum: &mut HashSet<P>) {
        if accum.len() < limit && self.unfilled.contains(p) && !accum.contains(p) {
            accum.insert(*p);

            for d in &self.dirs {
                self.spread(&(self.adder)(p, d), limit, accum);

                if accum.len() == limit {
                    break;
                }
            }
        }
    }

    // Public method to find the number of reachable points within a limit
    pub fn reachable(&self, p: &P, limit: usize) -> usize {
        let mut reached = HashSet::new();
        self.spread(p, limit, &mut reached);
        reached.len()
    }

    // Method to fill the board with markers at specified points
    pub fn fill(&mut self, ps: &Vec<P>, offset: P, marker: &str) -> Option<Vec<P>> {
        let mut eps = vec![];

        for p in ps {
            let op = (self.adder)(p, &offset);
            if !self.unfilled.contains(&op) {
                return None;
            } else {
                eps.push(op);
            }
        }
        for ep in &eps {
            self.unfilled.remove(ep);
            self.filled.insert(*ep, marker.to_string());
        }

        Some(eps)
    }

    pub fn unfill(&mut self, eps: Vec<P>) {
        for ep in eps {
            self.unfilled.insert(ep);
            self.filled.remove(&ep);
        }
    }

    // Method to get the marker at a specified point
    pub fn at(&self, p: &P) -> Option<&str> {
        if self.unfilled.contains(p) {
            return None; // fillable square
        }

        self.filled.get(p).map(|s| s.as_str())
    }

    // Method to get the remaining unfilled points
    pub fn remaining(&self) -> Vec<&P> {
        self.unfilled.iter().collect()
    }
}

// Function to add two Points
pub fn add(a: &Point, b: &Point) -> Point {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

// Function to create a Point Board
pub fn make_point_board(points: Vec<Point>) -> Board<Point> {
    Board::new(
        points,
        add,
        vec![
            Point { x: 0, y: -1 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
        ],
    )
}
