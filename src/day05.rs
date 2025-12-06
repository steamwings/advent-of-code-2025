#![feature(btree_cursors)]
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Bound;

// I used this to find the right answer for part 1 before fixing my better algorithm.
// fn solve_part1_brute(lines: &mut Lines<BufReader<File>>) -> usize {
//   let fresh_ranges: Vec<(u64, u64)> =
//     lines.take_while(|line| {
//         line.as_ref().unwrap() != ""
//       })
//       .map(|line| {
//         let s = line.expect("here");
//         let (start, end) = s.split_once('-').unwrap();
//         (start.parse::<u64>().expect("int"), end.parse::<u64>().expect("int"))
//       })
//       .collect();

//   lines.map(|line| {
//       let current: u64 = line.unwrap().parse().expect("int");
//       fresh_ranges.iter().any(|(start, end)| *start <= current && current <= *end )
//     })
//     .filter(|x| *x)
//     .count()
// }

// Here's the magic. Efficiently store and consolidate ranges of numbers in a BTreeMap where
// the keys are the starts of each range and the values are the end of the range.
// Each insertion/consolidation should only traverse once.
fn insert_or_consolidate_range(fresh: &mut BTreeMap<u64, u64>, start: u64, end: u64) {
  let mut zero: u64 = 0;
  let mut cursor = fresh.lower_bound_mut(Bound::Excluded(&start));
  let (next_key, next_value) = cursor.as_cursor().peek_next()
    .map(|(k, v)| (*k,*v))
    .or(Some((0,0))).unwrap();
  let (prev_key, prev_value) = cursor.peek_prev()
    .map(|(k,v)| (*k,v))
    .or(Some((0,&mut zero))).unwrap();

  //println!("{}-{} / {}-{} / {}-{}", prev_key, *prev_value, start, end, next_key, next_value);

  if prev_key == 0 && next_key == 0 { // empty map
    cursor.insert_before(start, end).expect("no error");
  } else if next_key == 0 || end < next_key { // don't join with next
    if *prev_value + 1 < start { // don't join with either
      cursor.insert_after(start, end).expect("no error");
    } else if *prev_value < end { // join with prev
      *prev_value = end;
    } // else no new range to add
  } else if *prev_value == 0 || *prev_value < start { // join with next
    cursor.insert_before(start, std::cmp::max(next_value, end)).expect("hakuna matata");
    cursor.remove_next();
  } else { // join with both
    *prev_value = std::cmp::max(next_value, end);
    cursor.remove_next();
  }
}

fn initialize_btree(mut btree: &mut BTreeMap<u64, u64>, lines: &mut Lines<BufReader<File>>) {
  loop {
    let line = lines.next().expect("readable").expect("line");
    if line == "" { break }
    let mut indexes = line
      .split('-')
      .map(|n: &str| n.parse::<u64>().expect("int"));
    let start: u64 = indexes.next().expect("first");
    let end: u64 = indexes.next().expect("second");
    insert_or_consolidate_range(&mut btree, start, end);
  }
}

fn solve_part1(mut lines: &mut Lines<BufReader<File>>) -> u64 {
  let mut fresh = BTreeMap::new();
  initialize_btree(&mut fresh, &mut lines);

  lines.map(|line| {
    let current: u64 = line.unwrap().parse().expect("int");
    let cursor = fresh.lower_bound(Bound::Excluded(&current));
    let (start, end) = cursor.peek_prev()
      .map(|(k,v)| (*k,*v))
      .or(Some((0,0)))
      .unwrap();
      start <= current && current <= end
    })
    .filter(|x| *x)
    .count()
    .try_into().unwrap()
}

fn solve_part2(mut lines: &mut Lines<BufReader<File>>) -> u64 {
  let mut fresh = BTreeMap::new();
  initialize_btree(&mut fresh, &mut lines);

  fresh.iter().fold(0, |acc, (start, end)| {
    print!("{}-{} ", start, end);
    (end - start) + 1 + acc
  })
}

fn main() {
  let args: Vec<_> = env::args().collect();
  let file = File::open(&args[1]).expect("file opens");
  let mut lines = BufReader::new(file).lines();
  let answer: u64 = if &args[2] == "1" { solve_part1(&mut lines) } else { solve_part2(&mut lines) };
  println!("\nAnswer: {answer}")
}
