#[derive(Debug)]
pub enum MyError {
    InvalidDirection,
    InvalidIntParse,
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::InvalidDirection => write!(f, "Invalid Direction"),
            MyError::InvalidIntParse => write!(f, "i64 Parsing Error"),
        }
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(_: std::num::ParseIntError) -> Self {
        MyError::InvalidIntParse
    }
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
        Ok(Command {
            direction: parse_direciton(tokens[0])?,
            magnitude: tokens[1].trim().parse()?,
        })
    }
}

fn parse_direciton(text: &str) -> Result<Direction, MyError> {
    match text {
        "forward" => Ok(Direction::Forward),
        "up" => Ok(Direction::Up),
        "down" => Ok(Direction::Down),
        _ => {
            eprintln!("invalid direction found in line {:?}!", text);
            Err(MyError::InvalidDirection)
        }
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

    for command in commands {
        match command.direction {
            Direction::Forward => forward += command.magnitude,
            Direction::Down => depth += command.magnitude,
            Direction::Up => depth -= command.magnitude,
        }
    }

    println!(
        "Total magnitude of movement (fore * depth) = {:?}",
        forward * depth,
    );
}
