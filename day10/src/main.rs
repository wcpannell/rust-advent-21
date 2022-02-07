fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };
    let opening: [char; 4] = ['(', '[', '{', '<'];
    let closing: [char; 4] = [')', ']', '}', '>'];

    let mut first_corrupted: Vec<char> = Vec::new();
    let mut completions: Vec<Vec<char>> = Vec::new();

    for line in inputdata {
        let mut stack: Vec<char> = Vec::new();
        for x in line.chars() {
            match opening.contains(&x) {
                true => stack.push(x),
                false => {
                    let val = stack.pop().unwrap();
                    match opening.iter().position(|&q| q == val).unwrap()
                        == closing.iter().position(|&q| q == x).unwrap()
                    {
                        true => continue,
                        false => {
                            //println!("found closing char {x}");
                            first_corrupted.push(x);
                            stack.clear(); // corrupted not considered for completions
                            break;
                        }
                    }
                }
            }
        }
        // if there's something still on the stack, it's incomplete. offer
        // completion
        if stack.len() != 0 {
            let mut this_completion: Vec<char> = Vec::new();
            while let Some(opening_char) = stack.pop() {
                this_completion
                    .push(closing[opening.iter().position(|&x| x == opening_char).unwrap()]);
            }
            completions.push(this_completion);
        }
    }
    let p1points: u32 = first_corrupted.iter().fold(0, |acc, x| {
        acc + match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    });

    println!("total score {p1points}");

    //println!("Completions: {completions:?}");

    let mut scores: Vec<u64> = completions
        .iter()
        .map(|completion| {
            completion.iter().fold(0, |acc, x| {
                5 * acc
                    + match x {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
            })
        })
        .collect();
    scores.sort();

    //println!("scores: {scores:?}");

    println!("Total score p2 {}", scores[scores.len() / 2]);
}
