#[derive(Debug, Default, Clone, Copy)]
struct ProblemState {
    north: BankState,
    south: BankState,
}
#[derive(Debug, Clone, Copy)]
enum BankEnum {
    North(BankState),
    South(BankState),
}
impl ProblemState {
    fn with_bankstate(self, new: BankEnum) -> ProblemState {
        match new {
            BankEnum::North(b) => ProblemState {
                north: b,
                south: self.south,
            },
            BankEnum::South(b) => ProblemState {
                north: self.north,
                south: b,
            },
        }
    }
}

fn get_count_from_string(line: &str, delim: char) -> usize {
    let sw = line.split_whitespace();
    for s in sw {
        if s.ends_with("c") {
            return s[..s.len()].parse::<usize>().expect("invalid parse");
        }
    }
    0
}
fn generate_bankstate_from_line(line: &str) -> Result<BankState, ()> {
    let cannibals = get_count_from_string(line, 'c');
    let missionaries = get_count_from_string(line, 'm');

    Ok(BankState::default())
}

#[derive(Debug, Default, Clone, Copy)]
struct BankState {
    missionaries: usize,
    cannibals: usize,
    raft: bool,
}
impl BankState {
    fn with_missionaries(self, count: usize) -> BankState {
        BankState {
            missionaries: count,
            cannibals: self.cannibals,
            raft: self.raft,
        }
    }
    fn with_cannibals(self, count: usize) -> BankState {
        BankState {
            missionaries: self.missionaries,
            cannibals: count,
            raft: self.raft,
        }
    }
    fn with_raft(self) -> BankState {
        BankState {
            missionaries: self.missionaries,
            cannibals: self.cannibals,
            raft: true,
        }
    }
}
fn main() -> Result<(), Box<std::error::Error>> {
    let input = "N: 0m 0c\nS: 3m 3c R";
    //let mut problem_state = ProblemState::default();

    let problem_state = input.lines().fold(ProblemState::default(), |a, l| {
        let first_char = l.chars().nth(0).expect("Empty string?");
        let bankstate = generate_bankstate_from_line(l).expect("Invalid line");
        match first_char {
            'N' => a.with_bankstate(BankEnum::North(bankstate)),
            'S' => a.with_bankstate(BankEnum::South(bankstate)),
            _ => unreachable!(),
        }
    });

    Ok(())
}
