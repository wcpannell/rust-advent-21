/// General Assumptions:
/// target is always below the starting position (0,0)
/// the target always has a positive x value
use std::ops::Range;
use std::vec;

/// Calculate missile trajectory
/// Drag value of 1 applied to X in either direction while moving
/// Gravity applies -1 to y velocity each step
/// just use bruteforce instead of motion equations
fn trajectory(
    initial_velocity: (i32, i32),
    initial_position: (i32, i32),
    target: &(Range<i32>, Range<i32>),
) -> Result<Vec<(i32, i32)>, &'static str> {
    let mut trajectory: Vec<(i32, i32)> = vec![initial_position];
    let mut velocity = initial_velocity;
    let mut position = initial_position;

    #[cfg(test)]
    println!("Targeting {target:?} with {initial_velocity:?}");

    // assume target is always below the starting position
    while position.1 > target.1.start {
        // update position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // add position to trajectory
        trajectory.push(position);

        // update velocity
        if velocity.0 > 0 {
            velocity.0 -= 1;
        } else if velocity.0 < 0 {
            velocity.0 += 1;
        } // no change if 0

        velocity.1 -= 1;

        // do we need to go further?
        if target.0.contains(&position.0) && target.1.contains(&position.1) {
            return Ok(trajectory);
        }
    }
    Err("Miss")
}

/// Convert the text values to a range
///
/// Rust ranges are up-to-noninclusive where the problem is up-to-inclusive, so
/// the end value is incremented by one.
fn text_range_to_range(input: &str) -> Range<i32> {
    let tokens: Vec<&str> = input.split("=").collect();
    #[cfg(test)]
    println!("{tokens:#?}");
    let range: Vec<i32> = tokens[1]
        .trim_end_matches(",")
        .split("..")
        .into_iter()
        .map(|val| val.parse::<i32>().unwrap())
        .collect();
    range[0]..range[1] + 1
}

fn parse_input(input: &String) -> (Range<i32>, Range<i32>) {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    #[cfg(test)]
    println!("{tokens:#?}");
    (
        text_range_to_range(tokens[2]),
        text_range_to_range(tokens[3]),
    )
}

fn max_height_in_trajectory(trajectory: &Vec<(i32, i32)>) -> i32 {
    #[cfg(test)]
    println!("{trajectory:?}");
    let mut max = i32::MIN;
    for position in trajectory {
        if position.1 > max {
            max = position.1;
        } else {
            break;
        }
    }
    #[cfg(test)]
    println!("{max}");
    max
}

fn max_height_trajectory(target: &(Range<i32>, Range<i32>)) -> i32 {
    // find minimal starting point for x. for max trajectory assume x always
    // stalls out.
    // use the triangle equation to find a velocity that will hit after
    // stalling.
    let mut x: i32 = 0;
    while (x * (x + 1) / 2) < target.0.start {
        x += 1
    }

    let mut y: i32 = 0;
    let mut max_y: i32 = 0;
    let mut traj = trajectory((x, y), (0, 0), &target);
    const MAX_STEPS: u32 = 800;
    let mut steps = 0u32;

    while steps < MAX_STEPS {
        if traj.is_ok() {
            let this_max_y = max_height_in_trajectory(&traj.unwrap());
            if this_max_y > max_y {
                max_y = this_max_y;
                println!("Max height: {max_y}");
            }
        }
        y += 1;
        traj = trajectory((x, y), (0, 0), &target);
        steps += 1;
    }
    println!("end x: {x} y: {y}");
    return max_y;
}

/// Just stupid brute force, I'm sleepy.
fn find_all_hits(target: &(Range<i32>, Range<i32>)) -> usize {
    let mut hits = Vec::<(i32, i32)>::new();

    for x in 0..300 {
        for y in -150..150 {
            if trajectory((x, y), (0, 0), &target).is_ok() {
                hits.push((x, y));
            }
        }
    }

    hits.dedup();

    #[cfg(test)]
    println!("pairs: {hits:#?}");

    return hits.len();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let inputdata = match common::read_input(&args[1]) {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let target = parse_input(&inputdata[0]);
    println!("target it {target:#?}");

    println!("highest is {}", max_height_trajectory(&target));

    println!("number of valid velocity pairs {}", find_all_hits(&target));
}

#[cfg(test)]
/// Tests come from the problem statement examples
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input: Vec<String> = vec!["target area: x=240..292, y=-90..-57".to_string()];
        assert_eq!((240..292 + 1, -90..-57 + 1), parse_input(&input[0]));

        assert_eq!(
            (20..30 + 1, -10..-5 + 1),
            parse_input(&String::from("target area: x=20..30, y=-10..-5"))
        );

        // sanity check on my understanding of ranges.
        let test_range = -10..-5 + 1;
        println!("{test_range:#?}");
        assert!(test_range.contains(&-10));
        assert!(test_range.contains(&-5));
        assert!(!test_range.contains(&-11));
        assert!(!test_range.contains(&-4));
    }

    #[test]
    fn test_trajectory() {
        let target = (20..30 + 1, -10..-5 + 1);
        let _hit = trajectory((7, 2), (0, 0), &target).unwrap();
        let _hit = trajectory((6, 3), (0, 0), &target).unwrap();
        let _hit = trajectory((9, 0), (0, 0), &target).unwrap();
        let _hit = trajectory((6, 9), (0, 0), &target).unwrap();
        assert!(match trajectory((17, -4), (0, 0), &target) {
            Ok(_) => false,
            Err(_) => true,
        });
    }

    #[test]
    fn test_max_in_traj() {
        let traj = vec![(0, 0), (0, 2), (-1, 3), (32, 4), (2, 3), (2, -12)];
        assert_eq!(4, max_height_in_trajectory(&traj));
    }

    #[test]
    fn test_max_height_trajectory() {
        let target = (20..30 + 1, -10..-5 + 1);
        assert_eq!(45, max_height_trajectory(&target));
    }

    #[test]
    fn test_all_hits() {
        let target = (20..30 + 1, -10..-5 + 1);
        assert_eq!(112, find_all_hits(&target));
    }
}
