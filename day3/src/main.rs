fn main() {
    //let text_data = match common::read_input("../test_input.txt") {
    let text_data = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    // greater than floor of half the values
    let threshold: u64 = text_data.len() as u64 >> 1;

    // count '0's and '1's in each column
    let mut zeroes: Vec<u64> = vec![0; text_data[0].chars().count()];
    for row in text_data {
        for (index, value) in row.chars().enumerate() {
            if value == '0' {
                zeroes[index] += 1;
            }
        }
    }

    // Each gamma bit is determined by whichever bit has a higher count in the
    // input (epsilon is the inverse value of gamma)
    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    for count in zeroes {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if count <= threshold {
            // More 1's than zeroes
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    println!(
        "Gamma: {}, Epsilon: {}, Power Consumption {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
