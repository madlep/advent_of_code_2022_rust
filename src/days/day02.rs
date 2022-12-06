pub fn part1(data: String) -> String {
    data.lines()
        .map(parse_part1)
        .map(|round| round.score())
        .sum::<Score>()
        .to_string()
}

pub fn part2(data: String) -> String {
    data.lines()
        .map(parse_part2)
        .map(|round| round.score())
        .sum::<Score>()
        .to_string()
}

use Move::*;
use Outcome::*;

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

type Score = u32;

#[derive(Debug, PartialEq)]
struct Round {
    them: Move,
    us: Move,
}

impl Round {
    fn new(them: Move, us: Move) -> Self {
        Self { them, us }
    }

    fn new_desired(them: Move, desired_outcome: Outcome) -> Self {
        let us = Round::move_for_desired_outcome(&them, desired_outcome);
        Round::new(them, us)
    }

    fn move_for_desired_outcome(them: &Move, desired_outcome: Outcome) -> Move {
        match (them, desired_outcome) {
            (Rock, Win) => Paper,
            (Rock, Loss) => Scissors,
            (Rock, Draw) => Rock,
            (Paper, Win) => Scissors,
            (Paper, Loss) => Rock,
            (Paper, Draw) => Paper,
            (Scissors, Win) => Rock,
            (Scissors, Loss) => Paper,
            (Scissors, Draw) => Scissors,
        }
    }

    fn outcome(&self) -> Outcome {
        match (&self.them, &self.us) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Win,
            (Rock, Scissors) => Loss,
            (Paper, Rock) => Loss,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Win,
            (Scissors, Rock) => Win,
            (Scissors, Paper) => Loss,
            (Scissors, Scissors) => Draw,
        }
    }

    fn move_score(&self) -> Score {
        match self.us {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn outcome_score(&self) -> Score {
        match self.outcome() {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }

    fn score(&self) -> Score {
        self.outcome_score() + self.move_score()
    }
}

fn parse_them_move(mv: &str) -> Move {
    match mv {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        m => panic!("illegal move: {}", m),
    }
}

fn parse_us_move(mv: &str) -> Move {
    match mv {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        m => panic!("illegal move: {}", m),
    }
}

fn parse_desired_outcome(outcome: &str) -> Outcome {
    match outcome {
        "X" => Loss,
        "Y" => Draw,
        "Z" => Win,
        o => panic!("illegal outcome: {}", o),
    }
}

fn parse_part1(line: &str) -> Round {
    let them = parse_them_move(line.get(0..1).unwrap());
    let us = parse_us_move(line.get(2..3).unwrap());
    Round::new(them, us)
}

fn parse_part2(line: &str) -> Round {
    let them = parse_them_move(line.get(0..1).unwrap());
    let desired_outcome = parse_desired_outcome(line.get(2..3).unwrap());
    Round::new_desired(them, desired_outcome)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calcualtes_score() {
        // wins
        assert_eq!(Round::new(Scissors, Rock).score(), 6 + 1);
        assert_eq!(Round::new(Rock, Paper).score(), 6 + 2);
        assert_eq!(Round::new(Paper, Scissors).score(), 6 + 3);

        // losses
        assert_eq!(Round::new(Paper, Rock).score(), 0 + 1);
        assert_eq!(Round::new(Scissors, Paper).score(), 0 + 2);
        assert_eq!(Round::new(Rock, Scissors).score(), 0 + 3);

        // draws
        assert_eq!(Round::new(Rock, Rock).score(), 3 + 1);
        assert_eq!(Round::new(Paper, Paper).score(), 3 + 2);
        assert_eq!(Round::new(Scissors, Scissors).score(), 3 + 3);
    }

    #[test]
    fn it_parses_their_move() {
        assert_eq!(parse_them_move("A"), Rock);
        assert_eq!(parse_them_move("B"), Paper);
        assert_eq!(parse_them_move("C"), Scissors);
    }

    #[test]
    fn it_parses_our_move() {
        assert_eq!(parse_us_move("X"), Rock);
        assert_eq!(parse_us_move("Y"), Paper);
        assert_eq!(parse_us_move("Z"), Scissors);
    }

    #[test]
    fn it_parses_part1_input_lines() {
        assert_eq!(parse_part1("A Y "), Round::new(Rock, Paper));
        assert_eq!(parse_part1("B X "), Round::new(Paper, Rock));
        assert_eq!(parse_part1("C Z "), Round::new(Scissors, Scissors));
    }

    #[test]
    fn it_parses_part2_input_lines() {
        //draw
        assert_eq!(parse_part2("A Y "), Round::new(Rock, Rock));

        // loss
        assert_eq!(parse_part2("B X "), Round::new(Paper, Rock));

        // win
        assert_eq!(parse_part2("C Z "), Round::new(Scissors, Rock));
    }
}
