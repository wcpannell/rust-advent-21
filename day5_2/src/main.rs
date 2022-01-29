#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    // Parse the input data into lines
    // Lines in format "start.x,start.y -> end.x,end.y"
    // trim in inner map removes any extraneous spaces

    let lines: Vec<Line> = inputdata
        .iter()
        .map(|line| {
            let mut point = line
                .split(" -> ")
                .flat_map(|point| point.split(",").map(|x| x.trim().parse::<u32>().unwrap()));
            Line {
                start: Point {
                    x: point.next().unwrap(),
                    y: point.next().unwrap(),
                },
                end: Point {
                    x: point.next().unwrap(),
                    y: point.next().unwrap(),
                },
            }
        })
        .collect();
    //println!("Lines: {lines:#?}");

    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;

    // There's probably a better way to do this
    for line in &lines {
        if line.start.x > max_x {
            max_x = line.start.x;
        }
        if line.end.x > max_x {
            max_x = line.end.x;
        }
        if line.start.y > max_y {
            max_y = line.start.y;
        }
        if line.end.y > max_y {
            max_y = line.end.y;
        }
    }

    let mut graph: Vec<Vec<u32>> = vec![vec![0; (max_y + 1) as usize]; (max_x + 1) as usize];

    for line in &lines {
        let delta_x: i32 = line.end.x as i32 - line.start.x as i32;
        let delta_y: i32 = line.end.y as i32 - line.start.y as i32;
        if delta_x == 0 || delta_y == 0 || (delta_x.abs() == delta_y.abs()) {
            let mut x: u32 = line.start.x;
            let mut y: u32 = line.start.y;
            graph[y as usize][x as usize] += 1;
            while (x != line.end.x) || (y != line.end.y) {
                x = (x as i32
                    + match delta_x {
                        _ if delta_x.is_positive() => 1i32,
                        _ if delta_x.is_negative() => -1i32,
                        _ => 0i32,
                    }) as u32;
                y = (y as i32
                    + match delta_y {
                        _ if delta_y.is_positive() => 1i32,
                        _ if delta_y.is_negative() => -1i32,
                        _ => 0i32,
                    }) as u32;
                graph[y as usize][x as usize] += 1;
            }
        }
    }

    let mut junctions: u32 = 0;
    for line in &graph {
        //println!("{line:?}");
        for col in line {
            if *col > 1 {
                junctions += 1;
            }
        }
    }

    println!("Total number of junctions: {junctions}");
}
