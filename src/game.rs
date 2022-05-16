use std::fmt::{Display, Formatter};
use itertools::Itertools;
use rand::{Rng, thread_rng};

pub(crate) const NUM_CELLS: i32 = 200;

#[derive(Debug, Copy, Clone)]
pub enum GameStatus {
    Started,
    Stopped,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GameStatus::Started => "STARTED",
            GameStatus::Stopped => "STOPPED",
        })
    }
}

#[derive(Debug)]
pub struct Game {
    pub grid: Vec<Vec<bool>>,
    pub game_status: GameStatus,
    length: i32,
}

impl Game {
    pub fn new(length: i32) -> Self {
        if length <= 0 {
            panic!("the length of the grid must be positive!");
        }

        Game {
            grid: vec![vec![false; length as usize]; length as usize],
            game_status: GameStatus::Stopped,
            length,
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = thread_rng();

        for row in &mut self.grid {
            for cell in row {
                *cell = matches!(rng.gen_range(1..=100), 1..=20);
            }
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                *cell = false;
            }
        }
    }

    fn valid_point(&self, x: i32, y: i32) -> bool {
        let length = self.length;
        x >= 0 && x < length && y >= 0 && y < length
    }

    fn count_live_neighbors(&self, row: i32, col: i32) -> usize {
        (-1..=1)
            .cartesian_product(-1..=1)
            .map(|(dx, dy)| {
                let x = col + dx;
                let y = row + dy;

                if !(dx == 0 && dy == 0) && self.valid_point(x, y) && self.grid[y as usize][x as usize] {
                    1usize
                } else {
                    0usize
                }
            }).sum()
    }

    pub fn step(&mut self) {
        let mut grid = self.grid.clone();

        for row in 0..self.length {
            for col in 0..self.length {
                grid[row as usize][col as usize] = {
                    let neighbors = self.count_live_neighbors(row, col);

                    if self.grid[row as usize][col as usize] {
                        neighbors == 2 || neighbors == 3
                    } else {
                        neighbors == 3
                    }
                };
            }
        }

        self.grid = grid;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let grid = self.grid.iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        if *cell {
                            "1"
                        } else {
                            "0"
                        }
                    }).join(" ")
            }).join("\n");

        write!(f, "{}\n{}", self.game_status, grid)
    }
}
