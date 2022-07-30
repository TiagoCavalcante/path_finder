pub fn find_paths(
  matrix: &Vec<Vec<usize>>,
  remaining_vertices: &Vec<usize>,
  start: usize,
  end: usize,
  path: &mut Vec<usize>,
) {
  for (i, vertex) in remaining_vertices.iter().enumerate() {
    if matrix[start][*vertex] == 1 {
      path.push(*vertex);

      if *vertex == end {
        println!("{:?}", path);
      } else {
        let mut remaining_vertices =
          remaining_vertices.clone();
        remaining_vertices.swap_remove(i);

        find_paths(
          matrix,
          &remaining_vertices,
          *vertex,
          end,
          path,
        );
      }

      path.pop();
    }
  }
}
