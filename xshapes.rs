fn dfs(i: i32, j: i32, m: i32, n: i32, grid: &mut Vec<Vec<char>>) {
    let i = i as usize;
    let j = j as usize;
    let m = m as usize;
    let n = n as usize;

    if i < 0 || i >= m || j < 0 || j >= n || grid[i][j] == 'O' {
        return;
    }
    grid[i][j] = 'O';

    dfs(i as i32 - 1, j as i32, m as i32, n as i32, grid);
    dfs(i as i32 + 1, j as i32, m as i32, n as i32, grid);
    dfs(i as i32, j as i32 - 1, m as i32, n as i32, grid);
    dfs(i as i32, j as i32 + 1, m as i32, n as i32, grid);
}

fn x_shape(grid: &mut Vec<Vec<char>>) -> i32 {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut res = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == 'X' {
                dfs(i, j, m, n, grid);
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let mut grid = vec![
        vec!['X', 'O', 'O', 'X', 'O'],
        vec!['X', 'X', 'O', 'X', 'X'],
        vec!['O', 'X', 'X', 'X', 'O'],
        vec!['O', 'O', 'O', 'O', 'X'],
    ];

    println!("{}", x_shape(&mut grid)); // Output: 2
}
