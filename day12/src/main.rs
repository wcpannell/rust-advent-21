use std::collections::HashMap;

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test0_input.txt") {
    //let inputdata = match common::read_input("../test1_input.txt") {
    //let inputdata = match common::read_input("../test2_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let mut adjacencies: HashMap<String, Vec<String>> = HashMap::new();

    // read data into adjacencies map
    for line in inputdata {
        let (this_node, that_node) = line.split_once("-").unwrap();
        adjacencies
            .entry(this_node.to_owned()) // grab entry if exists
            .or_default() // else create new empty entry
            .push(that_node.to_owned()); // and that_node to list of connected nodes

        // push everything in the other way because graph is not directional
        adjacencies
            .entry(that_node.to_owned()) // grab entry if exists
            .or_default() // else create new empty entry
            .push(this_node.to_owned()); // and that_node to list of connected nodes
    }

    println!("part1 count: {}", dfs(&adjacencies, false));
    println!("part2 count: {}", dfs(&adjacencies, true));
}

#[derive(Clone, Debug)]
struct Path {
    path: Vec<String>, // The path traversed
    small_twice: bool, // consumable token to allow passing through small caves twice
}

fn dfs(adjacencies: &HashMap<String, Vec<String>>, small_twice: bool) -> usize {
    let mut paths: Vec<Path> = Vec::new(); // holds all completed Paths
    let mut work_stack: Vec<Path> = Vec::new(); // holds all in-progress Paths

    // start at "start" node.
    work_stack.push(Path {
        path: vec!["start".to_owned()],
        small_twice,
    });

    while let Some(this_path) = work_stack.pop() {
        // Record completed paths
        if this_path.path.last().unwrap() == "end" {
            paths.push(this_path);
            continue;
        }

        // for each node connected to this one
        for connected_node in adjacencies
            .get(&this_path.path.last().unwrap().to_owned())
            .unwrap()
        {
            // can't re-enter start node
            if connected_node == "start" {
                continue;
            }

            // Probably start a new path.
            // Here because we may need to consume the token in the next step.
            // This causes some wasted cycles/mem cloning if we double-back
            // across a small cave without a token, but it feels cleaner to me
            // than having another variable.
            //
            // @Performance: Refactor into a token_consumed flag and move this
            // to just before adding new connected_node to new_path.
            let mut new_path = this_path.clone();

            // bail out of we cross a small cave too often in this path
            if connected_node.chars().all(|x| x.is_lowercase())
                && this_path.path.contains(&connected_node)
            {
                if this_path.small_twice {
                    // consume freepass to double back once on small caves
                    // println!("consumed small token {this_path:?} + {connected_node}");
                    new_path.small_twice = false;
                } else {
                    // can't push on, bail.
                    // println!("rejected small cave {this_path:?} + {connected_node}");
                    continue;
                }
            }

            // keep exploring
            new_path.path.push(connected_node.to_owned());
            work_stack.push(new_path);
        }
    }
    // println!("Paths {paths:?}");
    return paths.len();
}
