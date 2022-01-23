#[derive(Debug)]
enum Criterion {
    Low,
    High,
    Ambiguous,
    AllSame,
}

enum Commonality {
    Most,
    Least,
}

fn get_most_common(index: usize, grid: &Vec<Vec<char>>) -> Criterion {
    let threshold: u64 = grid.len() as u64 >> 1;

    let mut zeroes: u64 = 0;

    for row in grid {
        if row[index] == '0' {
            zeroes += 1;
        }
    }

    if (zeroes == 0) || (zeroes == grid.len() as u64) {
        return Criterion::AllSame;
    } else if zeroes > threshold {
        return Criterion::Low;
    } else if zeroes < threshold {
        return Criterion::High;
    } else {
        return Criterion::Ambiguous;
    }
}

fn find_vec(iv: u64, commonality: Commonality, grid: &mut Vec<Vec<char>>) -> u64 {
    let mut value = iv;
    let row_len = grid[0].len();
    for col_index in 1..row_len {
        let temp = get_most_common(col_index, &grid);
        let criterion = match temp {
            Criterion::Low => match commonality {
                Commonality::Most => '0',
                Commonality::Least => '1',
            },
            Criterion::High => match commonality {
                Commonality::Most => '1',
                Commonality::Least => '0',
            },
            Criterion::Ambiguous => match commonality {
                Commonality::Most => '1',
                Commonality::Least => '0',
            },
            Criterion::AllSame => grid[0][col_index],
        };

        // Update the value, "decoding" the bit string
        value = value << 1;
        if criterion == '1' {
            value += 1;
        }

        // traverse the rows, kicking out filtered rows. There's some optimization
        // opportunity to start from the end of the grid and work forwards
        // since the remove method has to shift the vector to the left anytime
        // something gets kicked out.
        //
        // Note: This can be replaced with the drain_filter method once
        // stabilized
        let mut row_index = 0;
        while row_index < grid.len() {
            if grid[row_index][col_index] != criterion {
                let _ = grid.remove(row_index);
            } else {
                row_index += 1;
            }
        }
    }
    return value;
}

fn main() {
    //let text_data = match common::read_input("../test_input.txt") {
    let text_data = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    // create 2D vector of chars from Strings
    let mut char_grid: Vec<Vec<char>> = vec![];
    let mut o2: Vec<Vec<char>> = vec![];
    let mut co2: Vec<Vec<char>> = vec![];

    for row in text_data {
        char_grid.push(row.chars().collect());
    }

    let o2_first_criterion = match get_most_common(0, &char_grid) {
        Criterion::Low => '0',
        Criterion::High => '1',
        Criterion::Ambiguous => '1',
        Criterion::AllSame => char_grid[0][0],
    };

    // split rows into those that could be used for either o2 or co2 values
    for row in char_grid {
        if row[0] == o2_first_criterion {
            o2.push(row);
        } else {
            co2.push(row);
        }
    }

    // get ratings
    let o2_rating = find_vec(
        match o2_first_criterion {
            '0' => 0,
            '1' => 1,
            _ => panic!("wacky value for criterion, shouldn't happen!"),
        },
        Commonality::Most,
        &mut o2,
    );
    let co2_rating = find_vec(
        match o2_first_criterion {
            '0' => 1,
            '1' => 0,
            _ => panic!("wacky value for criterion, shouldn't happen!"),
        },
        Commonality::Least,
        &mut co2,
    );

    println!("O2 Rating: {}, CO2 Rating: {}", o2_rating, co2_rating);
    println!("Life Support Rating: {}", o2_rating * co2_rating);
}
