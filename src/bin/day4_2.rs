use std::fs;

struct Range {
  start: i32,
  end: i32,
}

fn parse_range(range_str: &str) -> Range {
  let range: Vec<&str> = range_str.split("-").collect();
  Range {
    start: range.get(0).unwrap().parse().unwrap(),
    end: range.get(1).unwrap().parse().unwrap()
  }
}

fn parse_pair(pair: &str) -> (Range, Range) {
  let ranges: Vec<&str> = pair.split(",").collect();
  let range_a = ranges.get(0).expect("Expected an entry for the first range");
  let range_b = ranges.get(1).expect("Expected an entry for the seconds range");
  (parse_range(range_a), parse_range(range_b))
}

fn is_overlapping(a: &Range, b: &Range) -> bool {
  a.start <= b.end && a.end >= b.start
}

fn main() {
  let file_contents = fs::read_to_string("resources/day4_input.txt").expect("Could not read file.");

  let mut sum = 0;

  for line in file_contents.lines() {
    let ranges = parse_pair(line);
    if is_overlapping(&ranges.0, &ranges.1) {
      sum += 1;
    }
  }

  println!("Sum: {}", sum);
}
