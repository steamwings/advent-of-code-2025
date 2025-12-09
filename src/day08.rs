#![feature(core_float_math)]
#![feature(f128)]

use std::env;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::Itertools;

fn add_node(line: &str, nodes: &mut Vec<(u64, u64, u64)>) -> usize {
  let (x, y, z) = line.splitn(3, ',')
    .map(|c| c.parse::<u64>().expect("smallish int"))
    .collect_tuple().expect("thruple");
  nodes.push((x,y,z));
  nodes.len()
}

fn euclidean_distance_squared_3d((x1,y1,z1): (u64, u64, u64), (x2,y2,z2): (u64, u64, u64)) -> u128 {
  let dx = x2.abs_diff(x1) as u128;
  let dy = y2.abs_diff(y1) as u128;
  let dz = z2.abs_diff(z1) as u128;
  dx.pow(2) + dy.pow(2) + dz.pow(2)
}

fn calculate_distances(closest: &mut BTreeMap<u128, (usize, usize)>, nodes: &Vec<(u64, u64, u64)>) {
  for i in 0..nodes.len() {
    for j in i+1..nodes.len() {
      //print!("({i},{j}),");
      let distance_squared = euclidean_distance_squared_3d(nodes[i], nodes[j]);
      let old = closest.insert(distance_squared, (i,j));
      if old != None { panic!("non-unique distance {distance_squared}") }
    }
  }
}

// fn print_node((x, y, z): (u64, u64, u64)) {
//   print!("({x},{y},{z})-");
// }

// fn print_circuit_summary(circuits: &Vec<Vec<usize>>, nodes: &Vec<(u64, u64, u64)>) {
//   circuits
//     .iter()
//     .sorted_by(|a,b| a.len().cmp(&b.len()))
//     .chunk_by(|x| x.len())
//     .into_iter()
//     .for_each(|(size, group)| {
//       print!("[{size}]: ");
//       group.for_each(|circuit| {
//         circuit.iter().for_each(|node_index| {
//           print_node(nodes[*node_index]);
//         });
//         print!(" | ");
//       });
//     println!();
//   })
// }

fn connect_a_cable(
  nodes: &Vec<(u64, u64, u64)>,
  closest_map: &mut BTreeMap<u128, (usize, usize)>,
  circuits: &mut Vec<Vec<usize>>,
  circuits_map: &mut HashMap<usize, usize>
) -> Option<(u64,u64)> {
  let (_distance, (a, b)) = closest_map.pop_first().expect("another one");

  let (x1, y1,z1) = nodes[a];
  let (x2, y2, z2) = nodes[b];
  //println!("Connecting ({x1},{y1},{z1}) and ({x2},{y2},{z2})");

  let mut circuit_a_index = circuits_map[&a];
  let circuit_b_index = circuits_map[&b];

  if circuit_a_index == circuit_b_index {
    return None;
  } else if circuit_a_index > circuit_b_index {
    circuit_a_index -= 1; // since we're about to remove b
  }

  // I'm sure there's a more efficient way...
  // Values pointing to the circuit b index will be updated below
  circuits_map.iter_mut().for_each(|(_k, v)| {
    if *v > circuit_b_index {
      *v -= 1;
    }
  });

  let mut b_circuit = circuits.remove(circuit_b_index);
  b_circuit.iter().for_each(|c| { circuits_map.insert(*c, circuit_a_index); } );
  circuits[circuit_a_index].append(&mut b_circuit);

  //print_circuit_summary(&circuits, &nodes);
  Some((x1,x2))
}

fn solve_part1(lines: &mut Lines<BufReader<File>>) -> usize {
  const CONNECTIONS: u16 = 1000; // sorry, hardcoded. Edit between sample and real input.
  let mut nodes: Vec<(u64, u64, u64)> = Vec::new();
  let mut closest_map: BTreeMap<u128, (usize, usize)> = BTreeMap::new();
  let mut circuits: Vec<Vec<usize>> = Vec::new();
  let mut circuits_map: HashMap<usize, usize> = HashMap::new();

  lines.for_each(|line| {
    let size = add_node(&line.expect("exists"), &mut nodes);
    circuits.push(vec![size-1]);
    circuits_map.insert(size-1, size-1);
  });

  calculate_distances(&mut closest_map, &nodes);

  for _c in 1..=CONNECTIONS {
    connect_a_cable(&nodes, &mut closest_map, &mut circuits, &mut circuits_map);
  }

  circuits
    .into_iter()
    .map(|circuit: Vec<usize>| circuit.len())
    .k_largest(3)
    .reduce(|acc, x| acc * x)
    .expect("has a value")
}

fn solve_part2(mut lines: &mut Lines<BufReader<File>>) -> usize {
  let mut nodes: Vec<(u64, u64, u64)> = Vec::new();
  let mut closest_map: BTreeMap<u128, (usize, usize)> = BTreeMap::new();
  let mut circuits: Vec<Vec<usize>> = Vec::new();
  let mut circuits_map: HashMap<usize, usize> = HashMap::new();

  lines.for_each(|line| {
    let size = add_node(&line.expect("exists"), &mut nodes);
    circuits.push(vec![size-1]);
    circuits_map.insert(size-1, size-1);
  });

  calculate_distances(&mut closest_map, &nodes);

  let mut result: Option<(u64,u64)> = None;
  while circuits.len() > 1 {
    result = connect_a_cable(&nodes, &mut closest_map, &mut circuits, &mut circuits_map);
  }
  let (x1, x2) = result.expect("last connected");
  (x1 * x2).try_into().expect("fits")
}

fn main() {
  let args: Vec<_> = env::args().collect();
  let file = File::open(&args[1]).expect("file opens");
  let mut lines = BufReader::new(file).lines();
  let answer: usize = if &args[2] == "1" { solve_part1(&mut lines) } else { solve_part2(&mut lines) };
  println!("\nAnswer: {answer}")
}
