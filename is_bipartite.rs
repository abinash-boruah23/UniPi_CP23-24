use std::collections::VecDeque;

fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let n = graph.len();
    let mut color = vec![0; n];
    let mut q = VecDeque::new();

    for i in 0..n {
        if color[i] != 0 {
            continue;
        }

        color[i] = 1;
        q.push_back(i);

        while let Some(node) = q.pop_front() {
            for &neighbour in &graph[node] {
                if color[neighbour as usize] == 0 {
                    color[neighbour as usize] = -color[node as usize];
                    q.push_back(neighbour as usize);
                } else if color[neighbour as usize] == color[node as usize] {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let graph = vec![
        vec![1, 3],
        vec![0, 2],
        vec![1, 3],
        vec![0, 2],
    ];

    println!("{}", is_bipartite(graph)); // Output: true
}
