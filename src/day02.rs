use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_lines_reduce<F, T>(filename: String, initial: T, mut reducer: F) -> T
where
    F: FnMut(T, &str) -> T,
{
    let file = File::open(filename).expect("file opens");
    let reader = BufReader::new(file);
    let mut accumulator = initial;

    // Note that I shamelessly edited the files to have one range per line
    // since it made processing simpler
    for line in reader.lines() {
        let line = line.expect("we get a string");
        accumulator = reducer(accumulator, &line);
    }

   return accumulator
}

fn next_even_value(value: &str) -> (u64, u32) {
  let zeroes = "0".repeat(value.len() / 2);
  let half = format!("1{zeroes}");
  return (half.repeat(2).parse().expect("is int"), half.parse().expect("is int"))
}

fn is_invalid_id(value: &str) -> (bool, u64, u32) {
  let (left, right) = value.split_at(value.len() / 2);
  let left_int = left.parse::<u32>().unwrap();
  let right_int = right.parse::<u32>().unwrap();
  let start_is_invalid = left_int == right_int;
  let next_invalid: u64;
  let half: u32;

  if value.len() % 2 == 1 {
    (next_invalid, half) = next_even_value(value)
  } else if left_int <= right_int {
    (next_invalid, half) = next_invalid_id(left_int)
  } else {
    half = left_int;
    next_invalid = half.to_string().repeat(2).parse().expect("is integer");
  }

  return (start_is_invalid, next_invalid, half)
}

fn next_invalid_id(mut half: u32) -> (u64, u32) {
  half += 1;
  let next = half.to_string().repeat(2).parse().expect("is integer");
  return (next, half)
}

fn part1_reducer(mut sum: u64, line: &str) -> u64 {
  let pieces: Vec<&str> = line.split('-').collect();
  if pieces.len() != 2 { panic!("Expected two numbers; got {}", pieces.len()) }

  let mut start = pieces[0];
  let end = pieces[1];

  if start.len() < 2 {
    if end.len() < 2 { return sum }
    start = "10"
  }
  // Two odd numbers of same length means whole range is safe
  if (start.len() % 2 == 1) && (end.len() % 2 == 1) && start.len() == end.len() { return sum }

  // Algorithm: starting at start, increment halves of the number to get each with sequence repeats, stopping at end
  let end: u64 = end.parse().expect("is integer");
  //let mut current: u64 = start.parse().expect("is integer");

  print!("{start}: ");
  let (invalid_start, mut current, mut half) = is_invalid_id(start);
  if invalid_start {
    let start_int: u64 = start.parse::<u64>().expect("is int");
    print!("{start_int},");
    sum += start_int;
  }

  while current <= end {
    print!("{current},");
    sum += current;
    (current, half) = next_invalid_id(half);
  }

  print!("\n");
  return sum;
}

fn can_be_partitioned_into_sequences(value: &String, sequence_length: usize) -> bool {
  let chars: Vec<char> = value.chars().collect();
  // let chars_clone = chars.clone();
  // println!("Chunks: {:?}", chars_clone.chunks_exact(sequence_length).collect::<Vec<_>>());

  let mut chunks = chars.chunks_exact(sequence_length);
  let first_chunk = chunks.next().expect("has at least one chunk");
  let all_match = chunks.all(|chunk| chunk == first_chunk);
  if chunks.remainder().len() != 0 { panic!("bad chunking for {value} at length {sequence_length}") }
  return all_match
}

fn part2_reducer(mut sum: u64, line: &str) -> u64 {
  let pieces: Vec<&str> = line.split('-').collect();
  if pieces.len() != 2 { panic!("Expected two numbers; got {}", pieces.len()) }

  let start = pieces[0];
  let end = pieces[1];
  let start_value: u64 = start.parse().unwrap();
  let end_value: u64 = end.parse().unwrap();

  print!("{start}-{end}: ");

  for i in start_value..(end_value + 1) {
    let value: String = i.to_string();
    let mut lengths = (1..((value.len() / 2) + 1))
      .filter(|l| value.len() % l == 0);
    //println!("Lengths for #{i}: {:?}", lengths);

    let splittable = lengths.any(|j|
      can_be_partitioned_into_sequences(&value, j)
    );
    if splittable {
      sum += i;
      print!("{i},")
    }
  }

  print!("\n");
  return sum;
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let input_file_path: String = args.get(1).expect("has a file path")
    .parse().expect("is integer");
  let part: i32 = (if args.len() > 2 { &args[2] } else { "1" }).parse().expect("integer");

  let answer = match part {
    1 => process_lines_reduce(input_file_path, 0, part1_reducer),
    2 => process_lines_reduce(input_file_path, 0, part2_reducer),
    _ => panic!("There are only two parts. (I think.)")
  };

  println!("\nAnswer: {answer}")
}
