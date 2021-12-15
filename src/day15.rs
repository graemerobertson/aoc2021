use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/*
 * I couldn't be bothered to re-invent Dijkstra's algorithm, so I've literally
 * just copied it from https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
 * and moved on with my life.
 */

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn build_graph(grid: &[Vec<u32>], grid_dimension: usize, graph: &mut Vec<Vec<Edge>>) {
    for row in 0..grid_dimension {
        for column in 0..grid_dimension {
            let mut edges: Vec<Edge> = Vec::new();
            if row > 0 {
                edges.push(Edge {
                    node: grid_dimension * (row - 1) + column,
                    cost: grid[row - 1][column] as usize,
                });
            }
            if column > 0 {
                edges.push(Edge {
                    node: grid_dimension * (row) + column - 1,
                    cost: grid[row][column - 1] as usize,
                });
            }
            if row < grid_dimension - 1 {
                edges.push(Edge {
                    node: grid_dimension * (row + 1) + column,
                    cost: grid[row + 1][column] as usize,
                });
            }
            if column < grid_dimension - 1 {
                edges.push(Edge {
                    node: grid_dimension * (row) + column + 1,
                    cost: grid[row][column + 1] as usize,
                });
            }
            graph.push(edges);
        }
    }
}

pub(crate) fn day15() {
    let f: File = File::open("data/day15.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let part1_grid_dimension: usize = diagnostics.get(0).unwrap().len();
    let mut part1_grid: Vec<Vec<u32>> = vec![vec![0; part1_grid_dimension]; part1_grid_dimension];

    for (i, line) in diagnostics.iter().enumerate() {
        for (j, point) in line.chars().enumerate() {
            part1_grid[i][j] = point as u32 - 48;
        }
    }
    let mut part1_graph: Vec<Vec<Edge>> = Vec::new();
    build_graph(&part1_grid, part1_grid_dimension, &mut part1_graph);
    println!(
        "Shortest path in part 1: {}",
        shortest_path(&part1_graph, 0, part1_graph.len() - 1).unwrap()
    );

    let part2_grid_dimension: usize = part1_grid_dimension * 5;
    let mut part2_grid: Vec<Vec<u32>> = vec![vec![0; part2_grid_dimension]; part2_grid_dimension];
    for row in 0..part2_grid_dimension {
        for column in 0..part2_grid_dimension {
            let mut value = part1_grid[row % part1_grid_dimension][column % part1_grid_dimension]
                + ((row / part1_grid_dimension) as u32)
                + ((column / part1_grid_dimension) as u32);
            if value > 9 {
                value -= 9;
            }
            part2_grid[row][column] = value;
        }
    }
    let mut part2_graph: Vec<Vec<Edge>> = Vec::new();
    build_graph(&part2_grid, part2_grid_dimension, &mut part2_graph);
    println!(
        "Shortest path in part 2: {}",
        shortest_path(&part2_graph, 0, part2_graph.len() - 1).unwrap()
    );
}
