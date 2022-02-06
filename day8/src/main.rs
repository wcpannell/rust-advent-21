struct DigitSet {
    patterns: [Vec<char>; 10],
    display: Vec<String>,
}

impl DigitSet {
    // Creates a new DigitSet from a "patterns | display" string.
    pub fn from_pattern(pattern: &String) -> Self {
        let mut temp = pattern.split("|");
        return DigitSet {
            patterns: PatternPossibility::from_str(temp.next().unwrap()).to_resolved_array(),
            display: temp
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect(),
        };
    }

    // Converts the DigitSet display to a u64
    pub fn to_number(&self) -> u64 {
        let mut accumulator: u64 = 0;
        for (index, number) in self.display.iter().enumerate() {
            // number * 10 ^ place
            accumulator += (self.digit_number(number).unwrap() as u64)
                * u64::pow(10, (self.display.len() - 1 - index) as u32);
        }
        return accumulator;
    }

    /// Helper function for to_number. Converts each digit
    fn digit_number(&self, number: &String) -> Option<u8> {
        for (index, pattern) in self.patterns.iter().enumerate() {
            if pattern.len() != number.len() {
                continue;
            }
            //println!("attempting to find {number:?} in {pattern:?}");
            let mut accumulator = 0;
            for num_seg in number.chars() {
                for &pat_seg in pattern {
                    if num_seg == pat_seg {
                        accumulator += 1;
                    }
                }
            }
            if accumulator == number.len() {
                return Some(index as u8);
            }
            // This would work if I bothered to sort the segments
            // if number.chars().zip(pattern).filter(|(x, y)| x == *y).count() == pattern.len() {
            //     return Some(index as u8);
            // }
        }
        None
    }
}

struct PatternPossibility {
    one: Vec<char>,   // Known: only with 2 segments
    four: Vec<char>,  // Known: only with 4 segments
    seven: Vec<char>, // Known: only with 3 segments
    eight: Vec<char>, // Known: only with 7 segments
    others: Vec<Vec<char>>,
}

impl PatternPossibility {
    pub fn new() -> Self {
        PatternPossibility {
            one: Vec::new(),
            four: Vec::new(),
            seven: Vec::new(),
            eight: Vec::new(),
            others: Vec::new(),
        }
    }

    pub fn from_str(input: &str) -> Self {
        let patterns: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|x| x.to_owned())
            .collect();
        let mut wip = PatternPossibility::new();
        for pattern in patterns {
            let mut char_pattern: Vec<char> = pattern.chars().collect();
            char_pattern.sort();
            match pattern.len() {
                2 => wip.one = char_pattern,
                3 => wip.seven = char_pattern,
                4 => wip.four = char_pattern,
                7 => wip.eight = char_pattern,
                _ => wip.others.push(char_pattern),
            }
        }
        return wip;
    }

    pub fn to_resolved_array(mut self) -> [Vec<char>; 10] {
        let nine = self.resolve_nine().unwrap();
        let three = self.resolve_three().unwrap();

        // Order matters for the following
        let zero = self.resolve_zero().unwrap();
        let six = self.resolve_six().unwrap();
        let five = self.resolve_five(&six).unwrap();
        let two = self.others.pop().unwrap();
        [
            zero, self.one, two, three, self.four, five, six, self.seven, self.eight, nine,
        ]
    }

    /// Nine is the only unresolved segment that includes 4
    fn resolve_nine(&mut self) -> Option<Vec<char>> {
        for (index, pattern) in self.others.iter().enumerate() {
            let mut match_counter = 0;
            for segment in pattern {
                for four_seg in &self.four {
                    if segment == four_seg {
                        match_counter += 1;
                    }
                }
            }
            if match_counter == self.four.len() {
                return Some(self.others.remove(index));
            }

            // This stops after the number of digits in four, so can't make a
            // complete match unless we're lucky enough to match the first 4
            // if pattern
            //     .iter()
            //     .zip(&self.four)
            //     .filter(|(x, y)| {
            //         println!("{pattern:?}: {x}, {y} == {}", x == y);
            //         x == y
            //     })
            //     .count()
            //     == self.four.len()
            // {
            //     return Some(self.others.remove(index));
            // }
        }
        return None;
    }

    /// Three is the only unresolved segment that includes 7 and has len of 5
    fn resolve_three(&mut self) -> Option<Vec<char>> {
        for (index, pattern) in self.others.iter().enumerate() {
            if pattern.len() != 5 {
                continue;
            }
            let mut match_counter = 0;
            for segment in pattern {
                for seven_seg in &self.seven {
                    if segment == seven_seg {
                        match_counter += 1;
                    }
                }
            }
            if match_counter == self.seven.len() {
                return Some(self.others.remove(index));
            }
            // if pattern
            //     .iter()
            //     .zip(&self.seven)
            //     .filter(|(x, y)| x == y)
            //     .count()
            //     == self.seven.len()
            // {
            //     return Some(self.others.remove(index));
            // }
        }
        return None;
    }

    /// Zero is the only unresolved segment that includes 7 and has a length of
    /// 6, other than 9
    fn resolve_zero(&mut self) -> Option<Vec<char>> {
        for (index, pattern) in self.others.iter().enumerate() {
            if pattern.len() != 6 {
                continue;
            }

            let mut match_counter = 0;
            for segment in pattern {
                for seven_seg in &self.seven {
                    if segment == seven_seg {
                        match_counter += 1;
                    }
                }
            }
            if match_counter == self.seven.len() {
                return Some(self.others.remove(index));
            }
            // if pattern
            //     .iter()
            //     .zip(&self.seven)
            //     .filter(|(x, y)| x == y)
            //     .count()
            //     == self.seven.len()
            // {
            //     return Some(self.others.remove(index));
            // }
        }
        return None;
    }

    /// after resolving 0 and 9, 6 is the only remaining 6-lit segment.
    fn resolve_six(&mut self) -> Option<Vec<char>> {
        for (index, pattern) in self.others.iter().enumerate() {
            if pattern.len() == 6 {
                return Some(self.others.remove(index));
            }
        }
        return None;
    }

    /// five is only unresolved segment wholly contained within 6
    fn resolve_five(&mut self, six: &Vec<char>) -> Option<Vec<char>> {
        for (index, pattern) in self.others.iter().enumerate() {
            //println!("Find all but 1 seg: {:?} in {:?}", six, pattern);
            let mut match_counter = 0;
            for segment in pattern {
                for six_seg in six {
                    if segment == six_seg {
                        match_counter += 1;
                    }
                }
            }
            // "5" should have 5 segments match within "6"
            if match_counter == 5 {
                return Some(self.others.remove(index));
            }
            // if pattern.iter().zip(six).filter(|(x, y)| x == y).count() == pattern.len() {
            //     return Some(self.others.remove(index));
            // }
        }
        return None;
    }
}

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let sets: Vec<DigitSet> = inputdata
        .iter()
        .map(|line| DigitSet::from_pattern(line))
        .collect();

    // Part 1. count 1,4,7,8 in outputs
    let mut p1_count: u64 = 0;
    for set in sets.iter() {
        for value in &set.display {
            for pattern in [
                &set.patterns[1],
                &set.patterns[4],
                &set.patterns[7],
                &set.patterns[8],
            ] {
                // take advantage that these matches are based solely on length
                if value.len() == pattern.len() {
                    p1_count += 1;
                }
            }
        }
    }
    println!("{}", p1_count);

    // part 2.
    let sum: u64 = sets.iter().fold(0, |acc, set| acc + set.to_number());
    println!("Sum of all sets' displays: {sum}");
}
