use std::fs;

/// Returns a value in the range from a-z = 1-26 and A-Z = 27-52
fn get_character_code(character: &char) -> u32 {
  if character.is_ascii_lowercase() {
    return (*character as u32) - 96;
  } else if character.is_ascii_uppercase() {
    return (*character as u32) - 38;
  }

  return 0;
}

fn main() {
  let file_contents = fs::read_to_string("resources/day3_input.txt").expect("Could not read file.");

  let mut sum = 0;
  let mut lines = file_contents.lines();

  loop {
    let rucksack_one = lines.next();
    let rucksack_two = lines.next();
    let rucksack_three = lines.next();
    
    if rucksack_one.is_none() || rucksack_two.is_none() || rucksack_three.is_none() {
      break;
    }
    
    let characters_one: Vec<char> = rucksack_one.unwrap().chars().collect();
    let characters_two: Vec<char> = rucksack_two.unwrap().chars().collect();
    let characters_three: Vec<char> = rucksack_three.unwrap().chars().collect();

    for character in characters_one.iter() {
      if characters_two.contains(character) && characters_three.contains(character) {
        let points = get_character_code(character);
        println!("{} is contained in all three rucksacks and is worth {} points", character, points);
        sum += points;
        break;
      }
    }
  }

  println!("Sum: {}", sum);
}
