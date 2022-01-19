#[derive(Debug)]
enum MyError {
    InvalidDirection,
    InvalidIntParse,
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    magnitude: i64,
}

impl Command {
    fn from_string(text: String) -> Result<Command, MyError> {
        let text = text.to_lowercase();
        let tokens: Vec<&str> = text.split_whitespace().collect();
        let dir: Direction = match tokens[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => {
                eprintln!("invalid direction found in line {:?}!", text);
                return Err(MyError::InvalidDirection);
            }
        };
        let mag = match tokens[1].trim().parse() {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Got Error parsing command string magnitude {:?}", e);
                return Err(MyError::InvalidIntParse);
            }
        };

        Ok(Command {
            direction: dir,
            magnitude: mag,
        })
    }
}

fn main() {
    // Read input file
    //let string_data = match common::read_input("../test_input.txt") {
    let string_data = match common::read_input("input.txt") {
        Ok(val) => val,
        Err(_) => panic!("File not Found! PANIC!"),
    };
    //println!("{:?}", string_data);

    // Tokenize each line
    let mut commands: Vec<Command> = Vec::new();
    for line in string_data {
        commands.push(Command::from_string(line).unwrap());
    }

    //println!("Got these commands {:?}", commands);

    let mut forward: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => {
                forward += command.magnitude;
                depth += aim * command.magnitude;
            }
            Direction::Down => aim += command.magnitude,
            Direction::Up => aim -= command.magnitude,
        }
    }

    println!(
        "Total magnitude of movement (fore * depth) = {:?}",
        forward * depth,
    );
}
