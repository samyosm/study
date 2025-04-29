fn is_grid_winning(grid: [[bool; 3]; 3]) -> bool {
    // This is here for stylistic reasons
    grid.is_empty()
    // Checking for rows
        || grid[0][0] && grid[0][1] && grid[0][2]
        || grid[1][0] && grid[1][1] && grid[1][2]
        || grid[2][0] && grid[2][1] && grid[2][2]
    // Checking for columns
        || grid[0][0] && grid[1][0] && grid[2][0]
        || grid[0][1] && grid[1][1] && grid[2][1]
        || grid[0][2] && grid[1][2] && grid[2][2]
    // Checking for diagonals
        || grid[0][0] && grid[1][1] && grid[2][2]
        || grid[0][2] && grid[1][1] && grid[2][0]
}

fn print_game(moves: &[Vec<i32>]) {
    let mut grid = [[' '; 3]; 3];

    for (i, m) in moves.iter().enumerate() {
        let x = m[0] as usize;
        let y = m[1] as usize;
        if i % 2 == 0 {
            grid[x][y] = 'x';
        } else {
            grid[x][y] = 'o';
        }
    }

    for col in grid {
        println!("{} | {} | {}", col[0], col[1], col[2]);
    }
}

pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    print_game(&moves);

    let mut x_grid = [[false; 3]; 3];
    let mut o_grid = [[false; 3]; 3];

    for (i, m) in moves.iter().enumerate() {
        let x = m[0] as usize;
        let y = m[1] as usize;
        if i % 2 == 0 {
            x_grid[x][y] = true;

            if is_grid_winning(x_grid) {
                return "A".to_string();
            }
        } else {
            o_grid[x][y] = true;

            if is_grid_winning(o_grid) {
                return "B".to_string();
            }
        }
    }

    if moves.len() == 9 {
        "Draw".to_string()
    } else {
        "Pending".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_column_win() {
        let result = tictactoe(vec![
            vec![0, 0],
            vec![1, 0],
            vec![0, 1],
            vec![1, 1],
            vec![0, 2],
        ]);
        assert_eq!(result, "A");
    }

    #[test]
    fn case_a() {
        let result = tictactoe(vec![
            vec![0, 0],
            vec![2, 0],
            vec![1, 1],
            vec![2, 1],
            vec![2, 2],
        ]);
        assert_eq!(result, "A");
    }

    #[test]
    fn case_b() {
        let result = tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0],
        ]);
        assert_eq!(result, "B");
    }

    #[test]
    fn case_draw() {
        let result = tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2],
        ]);
        assert_eq!(result, "Draw");
    }

    #[test]
    fn case_row_win() {
        let result = tictactoe(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 1],
            vec![2, 0],
        ]);
        assert_eq!(result, "A");
    }
}
