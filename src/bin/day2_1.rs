use std::fs;

#[derive(PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissors
}

struct SignPair {
  player: Sign,
  opponent: Sign
}

fn line_to_sign_pair(line: &str) -> SignPair {
  match line {
    "A X" => SignPair { player: Sign::Rock, opponent: Sign::Rock },
    "A Y" => SignPair { player: Sign::Paper, opponent: Sign::Rock },
    "A Z" => SignPair { player: Sign::Scissors, opponent: Sign::Rock },
    "B X" => SignPair { player: Sign::Rock, opponent: Sign::Paper },
    "B Y" => SignPair { player: Sign::Paper, opponent: Sign::Paper },
    "B Z" => SignPair { player: Sign::Scissors, opponent: Sign::Paper },
    "C X" => SignPair { player: Sign::Rock, opponent: Sign::Scissors },
    "C Y" => SignPair { player: Sign::Paper, opponent: Sign::Scissors },
    "C Z" => SignPair { player: Sign::Scissors, opponent: Sign::Scissors },
    _ => SignPair { player: Sign::Rock, opponent: Sign::Rock }
  }
}

fn check_a_wins(a: &Sign, b: &Sign) -> bool {
  match (a, b) {
    (Sign::Rock, Sign::Scissors) => true,
    (Sign::Paper, Sign::Rock) => true,
    (Sign::Scissors, Sign::Paper) => true,
    _ => false
  }
}

fn check_draw(a: &Sign, b: &Sign) -> bool {
  a == b
}

fn calculate_outcome_points(pair: &SignPair) -> i32 {
  if check_draw(&pair.player, &pair.opponent) {
    3
  } else if check_a_wins(&pair.player, &pair.opponent) {
    6
  } else {
    0
  }
}

fn calculate_sign_points(sign: &Sign) -> i32 {
  match sign {
    Sign::Rock => 1,
    Sign::Paper => 2,
    Sign::Scissors => 3
  }
}

fn main() {
  let file_contents = fs::read_to_string("resources/day2_input.txt").expect("Could not read file.");

  let mut score = 0;
  for line in file_contents.lines() {
    let sign_pair = line_to_sign_pair(line);
    score += calculate_outcome_points(&sign_pair) + calculate_sign_points(&sign_pair.player);
  }

  println!("Score: {}", score);
}
