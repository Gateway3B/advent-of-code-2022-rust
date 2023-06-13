use std::{cmp::Ordering, fs::read_to_string, path::Path};

#[derive(Eq, PartialEq, PartialOrd)]
enum RpcAttack {
    Rock,
    Paper,
    Scissors,
    Forfeit,
}

impl RpcAttack {
    fn from_char(char: Option<char>) -> RpcAttack {
        use RpcAttack::*;

        match char {
            None => Forfeit,
            Some(char) => match char {
                'A' | 'X' => Rock,
                'B' | 'Y' => Paper,
                'C' | 'Z' => Scissors,
                _ => Forfeit,
            },
        }
    }

    fn value(&self) -> i32 {
        use RpcAttack::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
            _ => 0,
        }
    }
}

impl Ord for RpcAttack {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        use RpcAttack::*;

        match (self, other) {
            (Rock, Paper) => Less,
            (Rock, Scissors) => Greater,
            (Rock, Rock) => Equal,
            (Paper, Scissors) => Less,
            (Paper, Rock) => Greater,
            (Paper, Paper) => Equal,
            (Scissors, Rock) => Less,
            (Scissors, Paper) => Greater,
            (Scissors, Scissors) => Equal,
            (_, Forfeit) => Greater,
            (Forfeit, _) => Less,
        }
    }
}

struct RpcRound {
    p1: RpcAttack,
    p2: RpcAttack,
}

impl RpcRound {
    fn get_points(&self) -> (i32, i32) {
        let mut p1_value = self.p1.value();
        let mut p2_value = self.p1.value();

        let tie_value = 3;
        let win_value = 6;

        if self.p1 == self.p2 {
            p1_value += tie_value;
            p2_value += tie_value;
        } else if self.p1 > self.p2 {
            p1_value += win_value;
        } else {
            p2_value += win_value;
        }

        (p1_value, p2_value)
    }
}

struct Rpc {
    rounds: Vec<RpcRound>,
}

impl Rpc {
    fn calculate_scores(&self) -> (i32, i32) {
        let mut p1_score = 0;
        let mut p2_score = 0;
        for round in self.rounds.iter() {
            let points = round.get_points();
            p1_score += points.0;
            p2_score += points.1;
        }

        (p1_score, p2_score)
    }
}

fn read_rpc_stategy_guide(path: &Path) -> Rpc {
    let mut rpc = Rpc { rounds: Vec::new() };

    for line in read_to_string(path).unwrap_or_default().lines() {
        let mut p1: Option<char> = None;
        let mut p2: Option<char> = None;
        for char in line.chars().enumerate() {
            match char {
                char if char.0 == 0 => p1 = Some(char.1),
                char if char.0 == 2 => p2 = Some(char.1),
                char if char.0 == 3 => break,
                _ => (),
            }
        }
        rpc.rounds.push(RpcRound {
            p1: RpcAttack::from_char(p1),
            p2: RpcAttack::from_char(p2),
        });
    }

    rpc
}

pub fn day2() {
    let rpc = read_rpc_stategy_guide(&Path::new("rock-paper-scissors-strategy-guide.txt"));
    let scores = rpc.calculate_scores();
    print!("{:?}", scores);
}
