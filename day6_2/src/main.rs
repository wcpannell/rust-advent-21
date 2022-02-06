struct Fish {
    days: u32,
}

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    // Create an array of 9 days (0-8).
    let mut fishes: Vec<u64> = vec![0; 9]; // counts of fishes N days old

    // Add the existing fish to their respective "bin" in the array
    for age in inputdata[0].split(",").map(|x| x.parse::<u64>().unwrap()) {
        fishes[age as usize] += 1;
    }

    // For each day, the fish with 0 days until spawning reset their
    // counter to 6 days until next spawn.
    // New fish start with 8 days until next spawn.
    for day in 1..(256 + 1) {
        let spawning = fishes.remove(0); // shift everything a day newer
        fishes[6] += spawning; // restart counter
        fishes.push(spawning); // Add new fish to day 8
        println!("day {day} count: {}", fishes.iter().sum::<u64>());
    }
}
