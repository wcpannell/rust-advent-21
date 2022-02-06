pub fn cost_part2(target: u32, positions: &Vec<u32>) -> u32 {
    let mut fuel: u32 = 0;
    for &value in positions {
        let delta = (value as i32 - target as i32).abs() as u32;
        fuel += ((delta * delta) + delta) / 2;
    }
    fuel
}

pub fn cost_part1(target: u32, positions: &Vec<u32>) -> u32 {
    let mut fuel: u32 = 0;
    for &value in positions {
        fuel += (value as i32 - target as i32).abs() as u32;
    }
    fuel
}

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let mut positions: Vec<u32> = inputdata[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // start at median
    positions.sort();
    let max_pos: u32 = positions[positions.len() - 1];
    let median: u32 = positions[positions.len() / 2];
    let min_pos: u32 = positions[0];

    println!(
        "Part 1: Median: {median}, cost {}",
        cost_part1(median, &positions)
    );

    let mut min_cost = cost_part2(median, &positions);
    let mut min = median;
    let mut target = median + 1;
    let mut target_cost = cost_part2(target, &positions);

    while (target_cost <= min_cost) && (target < max_pos) {
        min = target;
        min_cost = target_cost;
        target += 1;
        target_cost = cost_part2(target, &positions);
    }

    target = median - 1;
    target_cost = cost_part2(target, &positions);
    while (target_cost <= min_cost) && (target > min_pos) {
        min = target;
        min_cost = target_cost;
        target -= 1;
        target_cost = cost_part2(target, &positions);
    }

    println!("Part2: Min cost {min_cost}, position {min}");
}
