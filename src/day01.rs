use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  let input_file_path: String = args.get(1).expect("has a file path")
    .parse().expect("is integer");
  let part: i32 = (if args.len() > 2 { &args[2] } else { "1" }).parse().expect("integer");
  //println!("Reading input from {input_file_path}");

  let file_content = std::fs::read_to_string(input_file_path).expect("Failed to read input file");
  let mut arrow_index = 50i32;
  let mut hits_on_0 = 0;
  for line in file_content.lines() {
    let (direction, distance) = line.split_at(1);
    //println!("{direction}-{distance}");

    let distance: i32 = distance.parse().expect("is an integer");
    let mut index: i32 = arrow_index;
    let next_index;

    // Desperate, shnasty brute force algorithm since my fancy attempts failed tbh
    if direction == "L" {
      next_index = (index - distance).rem_euclid(100);
      if part == 2 {
        for _d in 0..distance {
          index -= 1;
          if index == 0 {
            hits_on_0 += 1;
          } else if index == -1 {
            index = 99
          }
          //print!("{index},")
        }
        if index != next_index {
          panic!("Expected index {index} to equal next index of {next_index}")
        }
      }
    } else if direction == "R" {
      next_index = (index + distance).rem_euclid(100);
      if part == 2 {
        for _d in 0..distance {
          index += 1;
          if index == 100 {
            hits_on_0 += 1;
            index = 0
          }
          //print!("{index},")
        }
        if index != next_index {
          panic!("Expected index {index} to equal next index of {next_index}")
        }
      }
    } else { panic!("invalid direction {direction}")}

    arrow_index = next_index;

    if part == 1 && arrow_index == 0 {
      hits_on_0 += 1;
    }

    println!("Current index: {arrow_index} (0 count: {hits_on_0})");
  }

  println!("Password: {hits_on_0}")
}
