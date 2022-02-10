use std::env;

#[derive(Debug, Clone, Copy)]
struct Dot {
    x: u32,
    y: u32,
}

impl PartialEq for Dot {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

#[derive(Debug)]
enum FoldDirection {
    X,
    Y,
}

impl FoldDirection {
    fn from_str(string: &str) -> Option<Self> {
        match string.chars().last() {
            Some('x') => Some(FoldDirection::X),
            Some('y') => Some(FoldDirection::Y),
            Some(_) => None,
            None => None,
        }
    }
}

#[derive(Debug)]
struct Fold {
    direction: FoldDirection,
    location: u32,
}

impl Dot {
    fn fold(&mut self, fold: &Fold) {
        match fold.direction {
            FoldDirection::X => {
                if self.x >= fold.location {
                    // (fold.location - 1) - (self.x - (fold.location + 1))
                    // simplifies to:
                    self.x = (2 * fold.location) - self.x;
                }
            }
            FoldDirection::Y => {
                if self.y >= fold.location {
                    self.y = (2 * fold.location) - self.y;
                }
            }
        }
    }
}

fn main() {
    // Get input
    let args: Vec<String> = env::args().collect();
    let inputdata = match common::read_input(&args[1]) {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let mut fold_state: bool = false;
    let mut dots: Vec<Dot> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();
    for line in inputdata {
        if fold_state == false {
            if line == "" {
                fold_state = true;
                continue;
            }
            let (x, y) = line.split_once(",").unwrap();
            dots.push(Dot {
                x: x.parse::<u32>().unwrap(),
                y: y.parse::<u32>().unwrap(),
            });
        } else {
            let (direction, location): (&str, &str) = line.split_once("=").unwrap();
            folds.push(Fold {
                direction: FoldDirection::from_str(direction).unwrap(),
                location: location.parse::<u32>().unwrap(),
            });
        }
    }

    //part1

    for fold in folds {
        println!("Executing Fold {:?}", fold);
        for dot in &mut dots {
            dot.fold(&fold);
        }
        // Deduplicate
        let mut new_dots: Vec<Dot> = Vec::new();
        for dot in &dots {
            if !new_dots.contains(dot) {
                new_dots.push(dot.to_owned());
            }
        }
        dots = new_dots;
        println!("count {}", dots.len());
    }

    // print dots
    let mut x_max: u32 = 0;
    let mut y_max: u32 = 0;
    for dot in &dots {
        if dot.x > x_max {
            x_max = dot.x;
        }
        if dot.y > y_max {
            y_max = dot.y;
        }
    }
    let mut display: Vec<Vec<char>> = vec![vec![' '; (x_max + 1) as usize]; (y_max + 1) as usize];
    for dot in dots {
        display[dot.y as usize][dot.x as usize] = '#';
    }

    let string_display: Vec<String> = display
        .iter()
        .map(|line| line.into_iter().collect())
        .collect();
    for line in string_display {
        println!("{line}");
    }
}
