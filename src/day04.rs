use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::{once};
use itertools::Itertools;
use tap::prelude::*;

const EMPTY: char = '.';
const ROLL: char = '@';
const ACCESSIBLE: char = 'x';

fn to_chars(s: String) -> Vec<char> {
  s.chars().collect()
}

// This is almost the same method as less_than_four_adjacent but has a different signature
fn less_than_four_adjacent_immutable(previous: &Vec<char>, current: &Vec<char>, next: &Vec<char>) -> u64 {
  (0..current.len()).fold(0, |acc, index| {
    if current[index] == EMPTY {
      print!("{EMPTY}");
      return acc;
    }
    let adjacent_count = [
      previous.len() == 0 || index == 0 || previous[index - 1] == EMPTY, // top left
      previous.len() == 0 || previous[index] == EMPTY, // top
      previous.len() == 0 || index == current.len() - 1 || previous[index + 1] == EMPTY, // top right
      index == 0 || current[index - 1] == EMPTY, // left
      index == current.len() - 1 || current[index + 1] == EMPTY, // right
      next.len() == 0 || index == 0 || next[index - 1] == EMPTY, // bottom left
      next.len() == 0 || next[index] == EMPTY, // bottom
      next.len() == 0 || index == current.len() - 1 || next[index + 1] == EMPTY, // bottom right
    ]
    .into_iter()
    .filter(|empty| !empty)
    .count();
    if adjacent_count < 4 {
      print!("{ACCESSIBLE}");
      //current[index] = EMPTY; This line is the only real difference in the mutable version
      acc + 1
    } else {
      print!("{ROLL}");
      acc
    }
  })
  .tap(|_| println!())
}

// Note that for the file sizes of Advent of Code, it would
// be more efficient to collect and store all the lines at once
// then have tuple_windows pass around copies of references.
// Just for interest, my implementation avoids loading the whole file
// into memory--but at the cost of copying Strings for each window and
// converting multiple times (as below) to Vec<char>.
// The part 2 solution loads all lines into memory.
fn solve_part1(lines: Lines<BufReader<File>>) -> u64 {
  // Pad empty lines at start and end to represent all free spaces
  once(String::new())
    .chain(lines.map(|l| l.unwrap()))
    .chain(once(String::new()))
    .tuple_windows::<(_, _, _)>()
    .map(|(previous, current, next)| {
      let (previous, current, next) = (to_chars(previous), to_chars(current), to_chars(next));
      less_than_four_adjacent_immutable(&previous, &current, &next)
    })
    .sum()
}

fn less_than_four_adjacent(previous: &Vec<char>, current: &mut Vec<char>, next: &Vec<char>) -> u64 {
  (0..current.len()).fold(0, |acc, index| {
    if current[index] == EMPTY {
      print!("{EMPTY}");
      return acc;
    }
    let adjacent_count = [
      previous.len() == 0 || index == 0 || previous[index - 1] == EMPTY, // top left
      previous.len() == 0 || previous[index] == EMPTY, // top
      previous.len() == 0 || index == current.len() - 1 || previous[index + 1] == EMPTY, // top right
      index == 0 || current[index - 1] == EMPTY, // left
      index == current.len() - 1 || current[index + 1] == EMPTY, // right
      next.len() == 0 || index == 0 || next[index - 1] == EMPTY, // bottom left
      next.len() == 0 || next[index] == EMPTY, // bottom
      next.len() == 0 || index == current.len() - 1 || next[index + 1] == EMPTY, // bottom right
    ]
    .into_iter()
    .filter(|empty| !empty)
    .count();
    if adjacent_count < 4 {
      print!("{ACCESSIBLE}");
      current[index] = EMPTY;
      acc + 1
    } else {
      print!("{ROLL}");
      acc
    }
  })
  .tap(|_| println!())
}


fn clear_rolls(lines: &mut Vec<Vec<char>>) -> u64 {
  let empty = vec![];
  let mut sum = 0;
  let last_index = lines.len() - 1;

  let (current, rest) = lines.split_at_mut(1);
  sum += less_than_four_adjacent(&empty, &mut current[0], &rest[0]);

  for i in 1..last_index {
    let (before, rest) = lines.split_at_mut(i);
    let (current, after) = rest.split_at_mut(1);
    sum += less_than_four_adjacent(&before[i-1], &mut current[0], &after[0]);
  }

  let (before, current) = lines.split_at_mut(last_index);
  sum += less_than_four_adjacent(&before[last_index-1], &mut current[0], &empty);
  sum
}

fn solve_part2(lines: Lines<BufReader<File>>) -> u64 {
  let mut lines: Vec<Vec<char>> = lines.map(|l| to_chars(l.unwrap())).collect();
  let mut sum: u64 = 0;
  let mut last_removed_count = 1;

  while last_removed_count > 0 {
    last_removed_count = clear_rolls(&mut lines);
    sum += last_removed_count;
  }

  sum
}

fn main() {
  let args: Vec<_> = env::args().collect();
  let file = File::open(&args[1]).expect("file opens");
  let lines = BufReader::new(file).lines();
  let answer = if &args[2] == "1" { solve_part1(lines) } else { solve_part2(lines) };
  println!("\nAnswer: {answer}")
}
