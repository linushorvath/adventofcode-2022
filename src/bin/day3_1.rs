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
  for line in file_contents.lines() {
    let characters: Vec<char> = line.chars().collect();
    let num_chars = characters.len();
    println!("{} contains {} characters", line, num_chars);

    let chunk: Vec<&[char]> = characters.chunks(num_chars / 2).collect();
    let rucksack_one = chunk.get(0).expect("Failed to split characters");
    let rucksack_two = chunk.get(1).expect("Failed to split characters");

    for character in rucksack_one.iter() {
      if rucksack_two.contains(character) {
        let points = get_character_code(character);
        println!("{} is contained in both rucksacks and is worth {} points", character, points);
        sum += points;
        break;
      }
    }
  }

  println!("Sum: {}", sum);
}
