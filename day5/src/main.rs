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
        if line.start.x == line.end.x {
            match line.start.y < line.end.y {
                true => {
                    for y in line.start.y..(line.end.y + 1) {
                        graph[y as usize][line.start.x as usize] += 1;
                    }
                }
                false => {
                    for y in line.end.y..(line.start.y + 1) {
                        graph[y as usize][line.start.x as usize] += 1;
                    }
                }
            }
        }
        if line.start.y == line.end.y {
            match line.start.x < line.end.x {
                true => {
                    for x in line.start.x..(line.end.x + 1) {
                        graph[line.start.y as usize][x as usize] += 1;
                    }
                }
                false => {
                    for x in line.end.x..(line.start.x + 1) {
                        graph[line.start.y as usize][x as usize] += 1;
                    }
                }
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
