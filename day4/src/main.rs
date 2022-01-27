#[derive(Clone, Copy, Debug)]
struct Mark {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<u32>>,
    marks: Vec<Mark>,
    won: bool,
}

#[derive(Debug)]
struct WinningBoard {
    score: u64,
    win_marks: Vec<u32>,
}

impl BingoBoard {
    pub fn new() -> Self {
        BingoBoard {
            board: Vec::new(),
            marks: Vec::new(),
            won: false,
        }
    }

    fn mark(&mut self, num: u32) -> bool {
        let mut hit: bool = false;
        for (row_index, row) in self.board.iter().enumerate() {
            for (col_index, value) in row.iter().enumerate() {
                if *value == num {
                    self.marks.push(Mark {
                        row: row_index,
                        col: col_index,
                    });
                    hit = true;
                }
            }
        }
        return hit;
    }

    // probably not used
    fn winning_values(&self) -> Option<Vec<u32>> {
        if self.marks.len() < 5 {
            // Not enough marks to win, bail early
            return None;
        }

        let mut row_hits: Vec<u32> = vec![0; 5];
        let mut col_hits: Vec<u32> = vec![0; 5];

        for mark in self.marks.iter() {
            row_hits[mark.row] += 1;
            col_hits[mark.col] += 1;
        }

        for (row_index, value) in row_hits.iter().enumerate() {
            if *value >= 5 {
                let mut retval: Vec<u32> = Vec::with_capacity(5);
                for markval in self.board[row_index].iter() {
                    retval.push(*markval);
                }
                return Some(retval);
            }
        }

        for (col_index, value) in col_hits.iter().enumerate() {
            if *value >= 5 {
                let mut retval: Vec<u32> = Vec::with_capacity(5);
                for i in 0..5 {
                    retval.push(self.board[i][col_index]);
                }
                return Some(retval);
            }
        }
        return None;
    }

    fn score(&self) -> u64 {
        let mut score: u64 = 0;
        for row in self.board.iter() {
            score += row.iter().copied().sum::<u32>() as u64;
        }
        for mark in self.marks.iter() {
            score -= self.board[mark.row][mark.col] as u64;
        }
        let last_mark = self.marks.last().unwrap();
        score *= self.board[last_mark.row][last_mark.col] as u64;
        return score;
    }
}

fn main() {
    // Get input
    //let inputdata = match common::read_input("../test_input.txt") {
    let inputdata = match common::read_input("../input.txt") {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };

    let mut groups_input: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    for row in &inputdata {
        if row != "" {
            group.push(row.to_string())
        } else {
            groups_input.push(group);
            group = Vec::new();
        }
    }

    // Grab the group if last line was not empty
    if !group.is_empty() {
        groups_input.push(group);
    }
    //println!("groups: {groups_input:#?}");

    // parse callout number list
    let callouts: Vec<u32> = groups_input
        .remove(0)
        .remove(0)
        .trim()
        .split(",")
        .map(|x| {
            let x = match x.parse::<u32>() {
                Ok(val) => val,
                Err(e) => panic!("Error parsing callout numbers! {}", e),
            };
            return x;
        })
        .collect();
    //println!("Numbers called {callouts:?}");

    let mut boards: Vec<BingoBoard> = Vec::new();
    for group in &groups_input {
        let mut board: BingoBoard = BingoBoard::new();
        for row in group.iter() {
            board.board.push(
                row.trim()
                    .split_whitespace()
                    .map(|x| {
                        let x = match x.parse::<u32>() {
                            Ok(val) => val,
                            Err(e) => panic!("Parsing board number string {e}"),
                        };
                        return x;
                    })
                    .collect(),
            );
        }
        boards.push(board);
    }
    //println!("Boards: {boards:#?}");

    // Play the game
    let mut winners: Vec<WinningBoard> = Vec::new();
    for callout in &callouts {
        for board in &mut boards {
            if board.won == false {
                board.mark(*callout);
                match board.winning_values() {
                    Some(vals) => {
                        //println!("Winning values {vals:#?}");
                        //println!("Score: {:#?}", board.score());
                        winners.push(WinningBoard {
                            score: board.score(),
                            win_marks: vals,
                        });
                        board.won = true;
                    }
                    None => (),
                };
            }
        }
    }

    println!("Winner = {:#?}", winners.first());
    println!("Loser = {:#?}", winners.last());
}
