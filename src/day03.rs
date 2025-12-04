use std::fs::File;
use std::iter::{once};
use std::io::{BufRead, BufReader, Lines};
use std::env;

// I implemented this for part 1, but this function is redundant with
// n_battery_joltage and find_max_and_first_position below
fn two_battery_joltage(line: &str) -> u64 {
  let chars: Vec<char> = line.chars().collect();
  let (left_index, left) = chars[0..chars.len()-1].iter()
    .enumerate()
    .reduce( |(i, biggest), (j, current)| {
      if *current > *biggest { (j, current) } else { (i, biggest) }
    }).unwrap();
  let right = chars[left_index+1..].iter().max().unwrap();
  let joltage = format!("{}{}", left, right);
  println!("{joltage} from {line}");
  joltage.parse().expect("int")
}

fn find_max_and_first_position(values: &[char]) -> (usize, &char) {
  values
    .iter()
    .enumerate()
    .reduce(|(i, biggest), (j, current)| {
      if *current > *biggest { (j, current) } else { (i, biggest) }
    })
    .unwrap()
}

fn n_battery_joltage(n: usize, batteries: &[char]) -> Vec<&char> {
  if n == 1 {
    vec![batteries.iter().max().unwrap()]
  } else {
    let (index, max) =
      find_max_and_first_position(&batteries[0..batteries.len()-(n-1)]);
    once(max)
      .chain(n_battery_joltage(n-1, &batteries[index+1..]))
      .collect()
  }
}

fn solve_part1(lines: Lines<BufReader<File>>) -> u64 {
  lines
    .map(|line| two_battery_joltage(&line.unwrap()) )
    .sum()
}

fn solve_part2(lines: Lines<BufReader<File>>) -> u64 {
  lines
    .map(|line| {
      let chars: Vec<char> = line.unwrap().chars().collect();
      n_battery_joltage(12, &chars)
        .into_iter()
        .collect::<String>()
        .parse::<u64>().expect("int")
    })
    .sum()
}

fn main() {
  let args: Vec<_> = env::args().collect();
  let file = File::open(&args[1]).expect("file opens");
  let lines = BufReader::new(file).lines();
  let answer = if &args[2] == "1" { solve_part1(lines) } else { solve_part2(lines) };
  println!("\nAnswer: {answer}")
}
