#[derive(Debug)]
struct Node {
    value: u32,
    flashed: bool,
}
fn inc_push(map: &mut Vec<Vec<Node>>, stack: &mut Vec<[usize; 2]>, row: usize, col: usize) {
    map[row][col].value += 1;
    if map[row][col].value > 9 {
        stack.push([row, col]);
    }
}

fn flash(map: &mut Vec<Vec<Node>>, stack: &mut Vec<[usize; 2]>, row: usize, col: usize) -> () {
    let max_i = map.len() - 1;
    let max_j = map[0].len() - 1; // assume uniform size

    if map[row][col].flashed == true {
        return;
    }

    // mark flashed
    map[row][col].flashed = true;

    // check N
    if row > 0 {
        inc_push(map, stack, row - 1, col);
    }

    // NE
    if row > 0 && col < max_j {
        inc_push(map, stack, row - 1, col + 1);
    }

    // E
    if col < max_j {
        inc_push(map, stack, row, col + 1);
    }

    // SE
    if row < max_i && col < max_j {
        inc_push(map, stack, row + 1, col + 1);
    }

    // S
    if row < max_i {
        inc_push(map, stack, row + 1, col);
    }

    // SW
    if row < max_i && col > 0 {
        inc_push(map, stack, row + 1, col - 1);
    }

    // W
    if col > 0 {
        inc_push(map, stack, row, col - 1);
    }

    // NW
    if row > 0 && col > 0 {
        inc_push(map, stack, row - 1, col - 1);
    }
}

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let mut map: Vec<Vec<Node>> = inputdata
        .iter()
        .map(|row| {
            row.chars()
                .map(|value| Node {
                    value: value.to_digit(10).unwrap(),
                    flashed: false,
                })
                .collect()
        })
        .collect();

    // in each step energy += 1, energy == 10 flashes && adjacent (inc diagonal)
    // energy += 1 && adjacent == 9 flashes && only flash once per step, each
    // energy >= 9 set to 0. how many flashes after 100 steps?
    // ==>
    // states:
    // 1) increment all
    // 2a) collect any nodes that will flash
    // 2b) if node > 9: increment neighbor nodes; goto 2a; else: goto 3)
    // 3) set any node > 9 to 0

    let mut flashes: u32 = 0;
    let mut old_flashes: u32; // delay assignment to avoid error
    let mut work_stack: Vec<[usize; 2]> = Vec::new();
    let mut step: u32 = 0;

    //for step in 0..100 {
    loop {
        step += 1;
        //update old_flashes
        old_flashes = flashes;
        // increment all
        for row_index in 0..map.len() {
            for col_index in 0..map[row_index].len() {
                map[row_index][col_index].value += 1;
                // collect any that will flash, assume none have flashed yet
                if map[row_index][col_index].value >= 10 {
                    work_stack.push([row_index, col_index]);
                }
            }
        }

        // flash and increment and reflash as-needed
        while let Some(flash_pair) = work_stack.pop() {
            flash(&mut map, &mut work_stack, flash_pair[0], flash_pair[1]);
        }

        for row_index in 0..map.len() {
            for col_index in 0..map[row_index].len() {
                if map[row_index][col_index].value > 9 {
                    map[row_index][col_index].value = 0;
                    flashes += 1;
                    map[row_index][col_index].flashed = false; // clear old flash
                }
            }
        }

        if flashes - old_flashes == 100 {
            println!("All flashed on Step {step}");
            break;
        }
        if step == 100 {
            println!("Flashed {flashes} times after 100 steps!");
        }
    }
}
