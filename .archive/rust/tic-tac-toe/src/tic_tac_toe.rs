use bitvec::prelude::*;

use bitvec::{array::BitArray, order::Msb0};

// NOTE: We use 16 bits despite only needing 9.
pub type Grid = BitArray<u16, Msb0>;

#[derive(Default, Debug)]
pub struct TicTacToe {
    moves_played: u8,
    x_grid: Grid,
    o_grid: Grid,
}

impl std::fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = [[' '; 3]; 3];

        for i in 0..9 {
            let x = i % 3;
            let y = i / 3;
            grid[y][x] = char::from_digit(i as u32, 10).unwrap();
        }

        for i in self.x_grid.iter_ones() {
            let x = i % 3;
            let y = i / 3;
            grid[x][y] = 'x'
        }

        for i in self.o_grid.iter_ones() {
            let x = i % 3;
            let y = i / 3;
            grid[x][y] = 'o'
        }

        for col in grid {
            writeln!(f, "{} | {} | {}", col[0], col[1], col[2]);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum TicTacToeState {
    XWon,
    OWon,
    Draw,
    OnGoing,
}

impl std::fmt::Display for TicTacToeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::XWon => "X Won",
                Self::OWon => "O Won",
                Self::Draw => "Draw",
                Self::OnGoing => "On going",
            }
        )
    }
}

impl TicTacToe {
    pub fn x_to_play(&self) -> bool {
        self.moves_played % 2 == 0
    }

    pub fn play(&mut self, x: usize, y: usize) {
        if self.x_to_play() {
            self.x_grid.set(x + y * 3, true);
        } else {
            self.o_grid.set(x + y * 3, true);
        }

        self.moves_played += 1;
    }

    pub fn state(&self) -> TicTacToeState {
        if Self::is_winning(&self.x_grid) {
            return TicTacToeState::XWon;
        }

        if Self::is_winning(&self.o_grid) {
            return TicTacToeState::OWon;
        }

        if self.moves_played == 9 {
            return TicTacToeState::Draw;
        }

        TicTacToeState::OnGoing
    }

    pub fn is_winning(grid: &Grid) -> bool {
        // TODO: Make static
        let winning_patterns: Vec<BitArray<[u16; 1], Msb0>> = vec![
            bitarr![u16, Msb0; 0, 0, 0, 1, 0, 0, 1, 0, 0],
            bitarr![u16, Msb0; 0, 1, 0, 0, 1, 0, 0, 1, 0],
            bitarr![u16, Msb0; 0, 0, 1, 0, 0, 1, 0, 0, 1],
            bitarr![u16, Msb0; 1, 1, 1, 0, 0, 0, 0, 0, 0],
            bitarr![u16, Msb0; 0, 0, 0, 1, 1, 1, 0, 0, 0],
            bitarr![u16, Msb0; 0, 0, 0, 0, 0, 0, 1, 1, 1],
            bitarr![u16, Msb0; 1, 0, 0, 0, 1, 0, 0, 0, 1],
            bitarr![u16, Msb0; 0, 0, 1, 0, 1, 0, 1, 0, 0],
        ];

        for pat in winning_patterns {
            if *grid & pat == pat {
                return true;
            }
        }

        false
    }

    pub fn grid(&self) -> Grid {
        self.x_grid ^ self.o_grid
    }
}
