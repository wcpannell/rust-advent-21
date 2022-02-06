struct Fish {
    days: u32,
}

impl Fish {
    pub fn new() -> Self {
        Fish { days: 8 }
    }

    pub fn spawn(&mut self) -> bool {
        if self.days == 0 {
            self.days = 6;
            return true;
        }
        self.days -= 1;
        return false;
    }
}

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    // only one row in input

    let mut fishes: Vec<Fish> = inputdata[0]
        .split(",")
        .map(|x| Fish {
            days: x.parse::<u32>().unwrap(),
        })
        .collect();

    let mut new_fishes: Vec<Fish> = Vec::new();
    for day in 1..(80 + 1) {
        //for day in 1..(256 + 1) {
        for fish in &mut fishes {
            if fish.spawn() == true {
                new_fishes.push(Fish::new());
            }
        }
        fishes.append(&mut new_fishes);
        println!("day: {day}, fish: {}", fishes.len());
    }
}
