use std::collections::{HashMap, HashSet};

// Importing the Point struct from the geometry module
use crate::Point;

// Type alias for 'pe'
type Pe = i16;

// Encoder function type alias
type Encoder<P> = fn(&P) -> Pe;

// Binary operation type alias
type BinaryOperation<P> = fn(&P, &P) -> P;

// Structure representing the Board
pub struct Board<P> {
    filled: HashMap<Pe, String>,
    unfilled: HashSet<Pe>,
    pub all: Vec<P>,
    encoder: Encoder<P>,
    adder: BinaryOperation<P>,
    dirs: Vec<P>,
}

impl<P: Copy> Board<P> {
    // Constructor
    pub fn new(ps: Vec<P>, encoder: Encoder<P>, adder: BinaryOperation<P>, dirs: Vec<P>) -> Self {
        let mut unfilled = HashSet::new();
        for p in &ps {
            unfilled.insert(encoder(p));
        }

        Board {
            filled: HashMap::new(),
            unfilled,
            all: ps,
            encoder,
            adder,
            dirs,
        }
    }

    // Private method to recursively spread and collect reachable points
    fn spread(&self, p: &P, limit: usize, accum: &mut HashSet<Pe>) {
        let ep = (self.encoder)(p);

        if accum.len() < limit && self.unfilled.contains(&ep) && !accum.contains(&ep) {
            accum.insert(ep);

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
    pub fn fill(&mut self, ps: Vec<P>, offset: P, marker: &str) -> Option<Vec<Pe>> {
        let mut eps = vec![];

        for p in ps {
            let op = (self.adder)(&p, &offset);
            let ep = (self.encoder)(&op);
            if !self.unfilled.contains(&ep) {
                return None;
            } else {
                eps.push(ep);
            }
        }
        for ep in &eps {
            self.unfilled.remove(&ep);
            self.filled.insert(*ep, marker.to_string());
        }

        Some(eps)
    }

    pub fn unfill(&mut self, eps: Vec<Pe>) {
        for ep in eps {
            self.unfilled.insert(ep);
            self.filled.remove(&ep);
        }
    }

    // Method to get the marker at a specified point
    pub fn at(&self, p: &P) -> Option<&str> {
        let ep = (self.encoder)(p);
        if self.unfilled.contains(&ep) {
            return None; // fillable square
        }

        self.filled.get(&ep).map(|s| s.as_str())
    }

    // Method to get the remaining unfilled points
    pub fn remaining(&self) -> Vec<&P> {
        self.all
            .iter()
            .filter(|p| self.unfilled.contains(&(self.encoder)(p)))
            .collect()
    }
}

// Function to encode a Point
pub fn encode(p: &Point) -> Pe {
    (p.x * 16 + p.y) as Pe
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
        encode,
        add,
        vec![
            Point { x: 0, y: -1 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
        ],
    )
}
