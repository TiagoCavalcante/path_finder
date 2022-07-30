pub fn read_tuple() -> (usize, usize) {
  let mut iter = read_row().into_iter();

  (iter.next().unwrap_or(0), iter.next().unwrap_or(0))
}

fn read_row() -> Vec<usize> {
  let mut buffer = String::new();
  match std::io::stdin().read_line(&mut buffer) {
    Ok(_) => {}
    Err(error) => {
      println!("error: {error}");
      std::process::exit(1);
    }
  };

  buffer
    .split_whitespace()
    .map(|s| s.parse().unwrap_or(0))
    .collect()
}

pub fn read_matrix() -> Vec<Vec<usize>> {
  let first_row = read_row();
  let dimension = first_row.len();

  let mut matrix = vec![first_row];
  for _ in 1..dimension {
    matrix.push(read_row())
  }
  matrix
}
