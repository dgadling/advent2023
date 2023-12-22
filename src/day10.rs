const DAY: u8 = 10;

pub fn main() {
    part1();
    part2();
}

use petgraph::algo::simple_paths::all_simple_paths;
use petgraph::graph::{NodeIndex, UnGraph};
use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    pub fn at(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn relative(&self, (x, y): (isize, isize)) -> Option<Self> {
        if self.x as isize + x < 0 {
            return None;
        }

        if self.y as isize + y < 0 {
            return None;
        }

        Some(Self {
            x: max(self.x as isize + x, 0) as usize,
            y: max(self.y as isize + y, 0) as usize,
        })
    }

    const NORTH: (isize, isize) = (0, -1);
    const SOUTH: (isize, isize) = (0, 1);
    const EAST: (isize, isize) = (1, 0);
    const WEST: (isize, isize) = (-1, 0);

    pub fn connections_via(&self, c: char) -> [Option<Node>; 2] {
        match c {
            '|' => [self.relative(Self::NORTH), self.relative(Self::SOUTH)],
            '-' => [self.relative(Self::EAST), self.relative(Self::WEST)],
            'L' => [self.relative(Self::NORTH), self.relative(Self::EAST)],
            'J' => [self.relative(Self::NORTH), self.relative(Self::WEST)],
            '7' => [self.relative(Self::SOUTH), self.relative(Self::WEST)],
            'F' => [self.relative(Self::SOUTH), self.relative(Self::EAST)],
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

fn build_connection_table() -> (Node, HashMap<Node, Vec<Node>>) {
    let mut start: Node = Node::at(usize::MAX, usize::MAX);
    let mut curr: Node;
    let mut conn_by_node: HashMap<Node, Vec<Node>> = HashMap::new();
    let mut connections: Vec<Node>;

    for (y, line) in crate::utils::lines(DAY).enumerate() {
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '.' => {
                    continue;
                }
                'S' => {
                    start = Node::at(x, y);
                    continue;
                }
                _ => (),
            }

            curr = Node::at(x, y);

            // NOTE: Using flatten() means having to borrow a bunch and I couldn't figure out how to make that work
            connections = curr
                .connections_via(tile)
                .iter()
                .filter(|n| n.is_some())
                .map(|n| n.unwrap())
                .collect::<Vec<Node>>()
                .try_into()
                .unwrap();

            if connections.len() == 2 {
                // This means it's not at the edge pointing out into space
                conn_by_node.insert(curr, connections);
            }
        }
    }

    return (start, conn_by_node);

}

fn build_graph(conn_by_node: HashMap<Node, Vec<Node>>, start: Node) -> (UnGraph<Node, ()>, NodeIndex) {
    let mut g = UnGraph::<Node, ()>::default();
    let mut our_idx: NodeIndex;
    let mut start_idx: NodeIndex = NodeIndex::from(0);
    let mut node_idx_by_node: HashMap<Node, NodeIndex> = HashMap::new();
    let mut connection_idxs: Vec<NodeIndex>;
    let mut new_connection_idxs: Vec<NodeIndex>;

    for (n, conns) in conn_by_node.iter() {
        let left = conns.first().unwrap();
        let right = conns.last().unwrap();

        if *left != start && *right != start {
            if !conn_by_node.contains_key(left) || !conn_by_node.contains_key(right) {
                // our left/right nodes don't actually exist, skip!
                continue;
            }

            let left_neighbors = conn_by_node.get(left).unwrap();
            let right_neighbors = conn_by_node.get(right).unwrap();
            if !left_neighbors.contains(&n) || !right_neighbors.contains(&n) {
                // n thinks it connects to left/right, but left/right doesn't doesn't connect back to n.
                // make a special case for the start node since it doesn't know which way it connects.
                // Either way, this pipe is terminated on one side, so skip it since it can't be part of the loop.
                continue;
            }
        }

        our_idx = *node_idx_by_node.entry(*n).or_insert_with(|| g.add_node(*n));
        if *n == start {
            start_idx = our_idx;
        }
        connection_idxs = conns
            .iter()
            .map(|n| *node_idx_by_node.entry(*n).or_insert_with(|| g.add_node(*n)))
            .collect();

        new_connection_idxs = connection_idxs
            .iter()
            .filter(|n_idx| !g.contains_edge(our_idx, **n_idx))
            .map(|n| *n)
            .collect::<Vec<NodeIndex>>();

        for idx in new_connection_idxs {
            g.add_edge(our_idx, idx, ());
        }
    }
    (g, start_idx)
}

fn part1() {
    /*
     Real simple two step process:
     1. Record all the pipes and what they think they connect to
     2. Add to the graph only the pipes that actually connect to what they think they do
          that is, for any given node, it's contained in *both* of its neighbors set of connections
     3. Find the start element, get all paths from it's first neighbor to it's second neighbor. There should only be one.
     4. "furthest" point is (distance walked + 1) / 2
    */

    // Step 1
    let (start, conn_by_node) = build_connection_table();

    // Step 2
    let (g, start_idx) = build_graph(conn_by_node, start);

    // Step 3
    let neighbors: Vec<NodeIndex> = g.neighbors(start_idx).collect();
    let all_paths = all_simple_paths::<Vec<_>, _>(
        &g,
        *neighbors.first().unwrap(),
        *neighbors.last().unwrap(),
        2,
        None,
    )
    .collect::<Vec<_>>();

    assert!(
        all_paths.len() == 1,
        "There should only be one path from the start!"
    );

    done!(DAY, 1, (all_paths.first().unwrap().len() + 1) / 2);
}

fn part2() {
    done!(DAY, 2, "idk");
}
