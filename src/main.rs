#![feature(linked_list_remove)]

mod io;
mod path;

fn main() {
  let (start, end) = io::read_tuple();

  let matrix = io::read_matrix();

  let mut vertices =
    (0..matrix.len()).filter(|i| *i != start).collect();

  path::find_paths(
    &matrix,
    &mut vertices,
    start,
    end,
    &mut vec![start],
  );
}
