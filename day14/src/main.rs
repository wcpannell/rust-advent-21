use std::collections::HashMap;
fn main() {
    // Get input
    let args: Vec<String> = std::env::args().collect();
    let mut inputdata = match common::read_input(&args[1]) {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let template: String = inputdata.remove(0);
    let _ = inputdata.remove(0); // empty line

    let patterns: HashMap<String, char> = inputdata
        .iter_mut()
        .map(|line| {
            let pair: Vec<&str> = line.split(" -> ").collect();
            (pair[0].to_owned(), pair[1].chars().last().unwrap())
        })
        .collect();

    // println!("{patterns:?}");
    // println!("{template}");
    // println!("len {}", template.len());

    part1(&template, &patterns);
    part2(&template, &patterns);
}

/// Find the difference between min and max values in map
///
/// @TODO, make generic for any key and any value that impls Ord (for cmp
/// function)
fn diff(histogram: &HashMap<char, u64>) {
    let min: u64 = histogram
        .iter()
        .min_by(|x, y| x.1.cmp(&y.1))
        .map(|(key, val)| {
            println!("min key: {key}, value: {val}");
            val
        })
        .unwrap()
        .to_owned();
    let max: u64 = histogram
        .iter()
        .max_by(|x, y| x.1.cmp(&y.1))
        .map(|(key, val)| {
            println!("max key: {key}, value: {val}");
            val
        })
        .unwrap()
        .to_owned();

    let diff: u64 = max - min;

    println!("Most - Least = {diff}");
}

// Naive implementation. Exponential runtime, runtime blows up after 20 steps.
fn part1(template: &String, patterns: &HashMap<String, char>) {
    let mut polymer: String = template.to_owned();
    for _ in 0..10 {
        let mut i: usize = 1;
        loop {
            match patterns.get(&polymer[(i - 1)..(i + 1)]) {
                Some(val) => {
                    polymer.insert(i, val.clone());
                    i += 1; // account for new char inserted
                }
                None => (),
            }
            i += 1; // iterate
            if i >= polymer.len() {
                break;
            }
        }
    }
    println!("Part 1, 10 steps:");
    let histogram: HashMap<char, u64> = polymer.chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    diff(&histogram);
}

/// format string into pairs and count instances of each.
/// this is faster because it reduces iterations by the number of instances of
/// each pair. At 40 steps, there are billions of each pair.
fn part2(template: &String, patterns: &HashMap<String, char>) {
    let mut pairs: HashMap<String, u64> = HashMap::new();

    // process template into pairs
    for i in 1..template.len() {
        let pair: String = template[(i - 1)..(i + 1)].to_owned();
        *pairs.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut new_pairs: HashMap<String, u64> = HashMap::new();
        pairs
            .iter()
            .for_each(|(key, value)| match patterns.get(key) {
                Some(newchar) => {
                    let left: String = key[0..1].to_owned() + &(newchar.to_string());
                    let right: String = newchar.to_string() + &key[1..2];
                    *new_pairs.entry(left).or_insert(0) += value;
                    *new_pairs.entry(right).or_insert(0) += value;
                }
                None => {
                    *new_pairs.entry(key.to_owned()).or_insert(0) += value;
                    println!("got none"); // this probably could happen if patterns were poorly chosen
                }
            });
        pairs = new_pairs;
    }

    let mut histogram: HashMap<char, u64> =
        pairs.iter().fold(HashMap::new(), |mut acc, (key, value)| {
            *acc.entry(key.chars().next().unwrap()).or_insert(0) += value;
            acc
        });

    // add count for last char in template. It will always be last because we
    // only insert between pairs and will be undercounted by 1 because we're
    // only counting the first char in each pair.
    *histogram
        .entry(template.chars().last().unwrap())
        .or_insert(0) += 1;

    println!("Part 2, 40 steps:");
    diff(&histogram);
}
