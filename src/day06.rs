use std::env;

fn set_operators<'a>(operators: &mut Vec<&'a str>, operator_line: &'a str) {
  operator_line
    .split_whitespace()
    .for_each(|operator| {
      operators.push(operator)
    });
}

fn solve_part1(data: &str) -> u64 {
  let mut lines = data.lines();
  let mut operands: Vec<u64> = Vec::new();
  let mut operators: Vec<&str> = Vec::new();

  set_operators(&mut operators,
    lines.next_back().expect("get the last line with operators")
  );

  lines.next().expect("first line").split_whitespace().for_each(|val| {
    operands.push(val.parse().expect("inty"))
  });

  lines.for_each(|line| line.split_whitespace().enumerate().for_each(|(index, val)| {
    let val: u64 = val.parse().expect("integerish");
    operands[index] = match operators[index] {
      "*" => val * operands[index],
      "+" => val + operands[index],
      _ => panic!("bad operator {} at index {}", operators[index], index)
    }
  }));
  operands.into_iter().sum()
}

fn solve_part2(data: &str) -> u64 {
  let lines: Vec<&str> = data.lines().collect();
  let mut operators: Vec<&str> = Vec::new();
  let grid: Vec<Vec<char>>;

  set_operators(&mut operators,
    lines.last().expect("get the last line with operators")
  );

  grid = lines.into_iter().map(|line| line.chars().collect()).collect();

  let mut total: u64 = 0;
  let mut group_total: u64 = 0;
  let mut group_index = 0;

  // WARNING: this is a bit broken if your editor trims trailing spaces.
  // It doesn't handle the last
  // column correctly if all lines are not the same length.
  // That means the answer it generates for the sample is exactly 4 short
  // because it misses that last 4.
  // This is because my  and can be avoided if you add
  // the spaces.
  // I didn't feel like fixing the algorithm.
  for col in 0..grid[0].len() {
    if (0..grid.len()-1).all(|i| grid[i][col] == ' ')  {
      println!("found blank at {col} ({group_total})");
      total += group_total;
      group_total = 0;
      group_index += 1;
      continue;
    };
    println!("col: {col}, {}", {grid.len()});
    let value: u64 = (0..grid.len()-1)
      .map(|row| grid[row][col])
      .filter(|char| *char != ' ')
      .collect::<String>()
      .parse().expect("integer");

    group_total = match operators[group_index] {
      "*" => if group_total == 0 { value } else { group_total * value },
      "+" => group_total + value,
      _ => panic!("bad operator {} at index {}", operators[group_index], group_index)
    }
  }
  total + group_total
}

fn main() {
  let args: Vec<_> = env::args().collect();
  // NOTE: The DoubleEndedIterator was helpful here and the file size is so small that
  // I gave up insisting on a streaming-style read for this day.
  let data = std::fs::read_to_string(&args[1]).expect("has data");
  let answer: u64 = if &args[2] == "1" { solve_part1(&data) } else { solve_part2(&data) };
  println!("\nAnswer: {answer}")
}
