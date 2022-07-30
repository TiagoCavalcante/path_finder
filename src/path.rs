pub fn find_paths(
  matrix: &Vec<Vec<usize>>,
  remaining_vertices: &mut Vec<usize>,
  start: usize,
  end: usize,
  path: &mut Vec<usize>,
) {
  // Subtract 1 here so we don't need to do it inside the loop.
  let last = remaining_vertices.len() - 1;

  for i in 0..=last {
    let vertex = remaining_vertices[i];

    if matrix[start][vertex] == 1 {
      path.push(vertex);

      if vertex == end {
        println!("{:?}", path);
      } else {
        // Swap the current element with the last and
        // decrement the length.
        remaining_vertices.swap_remove(i);

        find_paths(
          matrix,
          remaining_vertices,
          vertex,
          end,
          path,
        );

        // Revert our changes.
        remaining_vertices.push(vertex);
        remaining_vertices.swap(i, last);
      }

      path.pop();
    }
  }
}
