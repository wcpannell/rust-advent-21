use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Ord, Eq)]
struct Point {
    row_index: usize,
    col_index: usize,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    cost: u32,
    point: Point,
    parent: Point,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Point,
}

// impl PartialEq for Point {
//     // See if it comes from the same cell in the map
//     fn eq(&self, other: &Self) -> bool {
//         self.row_index == other.row_index && self.col_index == other.col_index
//     }
//
//     // Just do the opposite
//     fn ne(&self, other: &Self) -> bool {
//         self.eq(other) == false
//     }
// }
//
// impl Eq for Point {}

impl Point {
    fn edges(&self, map: &Vec<Vec<u32>>) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();
        let max_row: usize = map.len() - 1;
        let max_col: usize = map[max_row].len() - 1;

        // up
        if self.row_index > 0 {
            let newpoint = Point {
                row_index: self.row_index - 1,
                col_index: self.col_index,
            };
            edges.push(Edge {
                cost: map[newpoint.row_index][newpoint.col_index],
                point: newpoint,
                parent: self.clone(),
            });
        }

        // left
        if self.col_index > 0 {
            let newpoint = Point {
                row_index: self.row_index,
                col_index: self.col_index - 1,
            };
            edges.push(Edge {
                cost: map[newpoint.row_index][newpoint.col_index],
                point: newpoint,
                parent: self.clone(),
            });
        }

        // down
        if self.row_index < max_row {
            let newpoint = Point {
                row_index: self.row_index + 1,
                col_index: self.col_index,
            };
            edges.push(Edge {
                cost: map[newpoint.row_index][newpoint.col_index],
                point: newpoint,
                parent: self.clone(),
            });
        }

        // right
        if self.col_index < max_col {
            let newpoint = Point {
                row_index: self.row_index,
                col_index: self.col_index + 1,
            };
            edges.push(Edge {
                cost: map[newpoint.row_index][newpoint.col_index],
                point: newpoint,
                parent: self.clone(),
            });
        }

        edges
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph_adj: &HashMap<Point, Vec<Edge>>, start: Point, end: Point) -> Option<u32> {
    let mut distances: HashMap<Point, u32> = graph_adj
        .keys()
        .map(|key| (key.clone(), u32::MAX))
        .collect();

    // Start cost is never entered, per the spec.
    *distances.entry(start).or_insert(u32::MAX) = 0;

    let mut to_visit = BinaryHeap::new();
    to_visit.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = to_visit.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > distances[&position] {
            continue;
        }

        for edge in &graph_adj[&position] {
            let next_edge = State {
                cost: cost + edge.cost,
                position: edge.point,
            };

            if next_edge.cost < distances[&next_edge.position] {
                to_visit.push(next_edge);
                *distances.entry(next_edge.position).or_default() = next_edge.cost;
            }
        }
    }
    return None;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut inputdata: Vec<String> = common::read_input(&args[1]).unwrap();

    let costmap: Vec<Vec<u32>> = inputdata
        .iter_mut()
        .map(|line| line.chars().map(|val| val.to_digit(10).unwrap()).collect())
        .collect();

    let cm_rows = costmap.len();
    let cm_cols = costmap[cm_rows - 1].len();
    // println!("rows {cm_rows}, cols {cm_cols}");

    let mut graph: HashMap<Point, Vec<Edge>> = HashMap::new();
    for i in 0..cm_rows {
        for j in 0..cm_cols {
            let point = Point {
                row_index: i,
                col_index: j,
            };
            graph.insert(point, point.edges(&costmap));
        }
    }

    // println!("{graph:#?}");
    let part1 = dijkstra(
        &graph,
        Point {
            col_index: 0,
            row_index: 0,
        },
        Point {
            row_index: cm_rows - 1,
            col_index: cm_cols - 1,
        },
    )
    .unwrap();
    println!("part1 weight: {part1}");

    let mut costmap2: Vec<Vec<u32>> = vec![vec![u32::MAX; 5 * cm_cols]; 5 * cm_rows];
    for i in 0..5 as u32 {
        for j in 0..5 as u32 {
            for cm_row in 0..cm_rows {
                for cm_col in 0..cm_cols {
                    let new_row = cm_row + (cm_rows * i as usize);
                    let new_col = cm_col + (cm_cols * j as usize);
                    costmap2[new_row][new_col] = costmap[cm_row][cm_col] + i + j;

                    // Check if overflowed max of 9, can not overflow twice
                    // since only tiling 5 times
                    if costmap2[new_row][new_col] > 9 {
                        costmap2[new_row][new_col] -= 9;
                    }
                }
            }
        }
    }
    // println!("{costmap2:?}");

    let cm2_rows = costmap2.len();
    let cm2_cols = costmap2[cm2_rows - 1].len();
    // println!("rows {cm2_rows}, cols {cm2_cols}");

    let mut graph2: HashMap<Point, Vec<Edge>> = HashMap::new();
    for i in 0..cm2_rows {
        for j in 0..cm2_cols {
            let point = Point {
                row_index: i,
                col_index: j,
            };
            graph2.insert(point, point.edges(&costmap2));
        }
    }

    // println!("{graph2:#?}");
    let part2 = dijkstra(
        &graph2,
        Point {
            col_index: 0,
            row_index: 0,
        },
        Point {
            row_index: cm2_rows - 1,
            col_index: cm2_cols - 1,
        },
    )
    .unwrap();
    println!("part2 weight: {part2}");
}
