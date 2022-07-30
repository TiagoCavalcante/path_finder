# path_finder

Find non-looping paths in a graph

## How to compile?

```sh
cargo build --release
```

## How to use?

The graph is represented as a symmetrical square matrix with zeros for unconnected vertices and ones for connected vertices.

For example, the following graph

![3-vertice graph](https://user-images.githubusercontent.com/62714153/181867147-e104901a-62e9-4c93-a9d6-ab870cad7e98.png)

Is going to be represented with the following matrix:
$$\LARGE{\begin{bmatrix} 0 & 1 & 0 \\\ 1 & 0 & 1 \\\ 0 & 1 & 0 \end{bmatrix}}$$

Note that the diagonal of the matrix is filled with zeros because a vertex doesn't connect to itself.

The first input of the program is the start vertex, the second is the end vertex and the third is the matrix.

The output are all the paths from the start to the end vertex that don't repeat any vertex.

For example, for discovering the paths from vertex $1$ to vertex $3$ you would execute:

```sh
./target/release/path_finder
0 2
0 1 0
1 0 1
0 1 0
[0, 1, 2]
```

(note: remember that in programming numbers start at $0$ and not $1$)
