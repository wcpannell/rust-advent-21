#[derive(Debug)]
struct Point {
    value: u32,
    row_index: usize,
    col_index: usize,
}

impl PartialEq for Point {
    // See if it comes from the same cell in the map
    fn eq(&self, other: &Self) -> bool {
        self.row_index == other.row_index && self.col_index == other.col_index
    }
}

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let map: Vec<Vec<u32>> = inputdata
        .iter()
        .map(|row| {
            row.chars()
                .map(|value| value.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut low_points: Vec<Point> = Vec::new();
    let max_i = map.len() - 1;
    let max_j = map.iter().map(|x| x.len()).max().unwrap() - 1;
    for i in 0..max_i + 1 {
        for j in 0..max_j + 1 {
            // check left
            if j > 0 {
                if map[i][j] >= map[i][j - 1] {
                    // not the lowest
                    continue;
                }
            }

            // check up
            if i > 0 {
                if map[i][j] >= map[i - 1][j] {
                    continue;
                }
            }

            // check right
            if j < max_j {
                if map[i][j] >= map[i][j + 1] {
                    continue;
                }
            }

            // check down
            if i < max_i {
                if map[i][j] >= map[i + 1][j] {
                    continue;
                }
            }

            // survived checks, so must be a low point
            low_points.push(Point {
                value: map[i][j],
                row_index: i,
                col_index: j,
            });
        }
    }

    //println!("{low_points:?}");
    let sum_risks: u32 = low_points.iter().fold(0, |acc, x| acc + x.value + 1);
    println!("Total risk {sum_risks}");

    // Now need to traverse each higher point that's less than 9 from each
    // low_point. Using DFS, we'll take each low point as an origin point and
    // treat all neighbor points that are greater than the current point and
    // less than 9 as having a traversible link from the point.

    let mut basins: Vec<Vec<Point>> = Vec::new();

    for low_point in low_points {
        let mut basin: Vec<Point> = Vec::new();
        let mut dfs_stack: Vec<Point> = Vec::new();
        dfs_stack.push(low_point);
        while !dfs_stack.is_empty() {
            let point = dfs_stack.pop().unwrap();

            // Already discovered? skip.
            if basin.contains(&point) {
                continue;
            }

            // Check up
            if point.row_index > 0 {
                let up = Point {
                    value: map[point.row_index - 1][point.col_index],
                    row_index: point.row_index - 1,
                    col_index: point.col_index,
                };
                // Note: graph could be cyclical, so skip if point is already in stack
                if up.value > point.value && up.value < 9 && (!dfs_stack.contains(&up)) {
                    dfs_stack.push(up);
                }
            }

            // Check down
            if point.row_index < max_i {
                let down = Point {
                    value: map[point.row_index + 1][point.col_index],
                    row_index: point.row_index + 1,
                    col_index: point.col_index,
                };
                if down.value > point.value && down.value < 9 && (!dfs_stack.contains(&down)) {
                    dfs_stack.push(down);
                }
            }

            // Check left
            if point.col_index > 0 {
                let left = Point {
                    value: map[point.row_index][point.col_index - 1],
                    row_index: point.row_index,
                    col_index: point.col_index - 1,
                };
                if left.value > point.value && left.value < 9 && (!dfs_stack.contains(&left)) {
                    dfs_stack.push(left);
                }
            }

            // Check Right
            if point.col_index < max_j {
                let right = Point {
                    value: map[point.row_index][point.col_index + 1],
                    row_index: point.row_index,
                    col_index: point.col_index + 1,
                };
                if (right.value > point.value) && (right.value < 9) && (!dfs_stack.contains(&right))
                {
                    dfs_stack.push(right);
                }
            }

            basin.push(point);
        }
        basins.push(basin);
    }

    let mut sizes: Vec<usize> = basins.iter().map(|x| x.len()).collect();
    let slen = sizes.len() - 1;
    sizes.sort();
    let p2answer = sizes[slen] * sizes[slen - 1] * sizes[slen - 2];
    println!("Part 2 answer: {p2answer}");
}
