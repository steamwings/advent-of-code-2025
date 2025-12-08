use std::env;

fn print_grid(grid: &Vec<Vec<char>>) {
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      print!("{}", grid[i][j]);
    }
    println!();
  }
}

fn solve_part1(data: &str) -> u64 {
  let mut grid: Vec<Vec<char>> = data.lines().into_iter()
    .map(|line| line.chars().collect()).collect();
  let mut split_count = 0;

  let (s_index, _char) = grid[0].iter().enumerate().find(|(_index, value)| **value == 'S').unwrap();
  grid[1][s_index] = '|';
  for i in 2..grid.len() {
    for j in 0..grid[0].len() {
      match grid[i][j] {
        '|' => (),
        '^' => if grid[i-1][j] == '|' {
          split_count += 1;
          grid[i][j-1] = '|';
          grid[i][j+1] = '|';
          print!("({i},{j}),")
        },
        '.' => if grid[i-1][j] == '|' {
          grid[i][j] = '|'
        },
        _ => panic!("unexpected char {}", grid[i][j])
      }
    }
  }

  print_grid(&grid);
  split_count
}

// Note that I stuck with using the grid here because it's nice to print
// but I suspect that updating the grid could be omitted and you could just check `paths`
// as you go. (...which would also make a nice streaming algorithm...)
fn solve_part2(data: &str) -> usize {
  let mut grid: Vec<Vec<char>> = data.lines().into_iter()
    .map(|line| line.chars().collect()).collect();
  let mut paths: Vec<usize> = vec![0; grid.len()];

  let (s_index, _char) = grid[0].iter().enumerate().find(|(_index, value)| **value == 'S').unwrap();
  grid[1][s_index] = '|';
  paths[s_index] = 1;
  for i in 2..grid.len() {
    for j in 0..grid[0].len() {
      match grid[i][j] {
        '|' => (),
        '^' => if grid[i-1][j] == '|' {
          paths[j+1] += paths[j];
          paths[j-1] += paths[j];
          paths[j] = 0;
          grid[i][j-1] = '|';
          grid[i][j+1] = '|';
        },
        '.' => if grid[i-1][j] == '|' {
          grid[i][j] = '|'
        },
        _ => panic!("unexpected char {}", grid[i][j])
      }
    }
    //let _ = paths.iter().for_each(|p| print!("{}-", *p));
    //println!();
  }

  print_grid(&grid);
  paths.into_iter().sum()
}
fn main() {
  let args: Vec<_> = env::args().collect();
  // Skipped streaming again in the name of code-writing speed
  let data = std::fs::read_to_string(&args[1]).expect("has data");
  let answer: u64 = if &args[2] == "1" { solve_part1(&data) } else { solve_part2(&data).try_into().unwrap() };
  println!("\nAnswer: {answer}")
}
