use std::fmt::{Display, Formatter};
use itertools::Itertools;
use rand::{Rng, thread_rng};

pub(crate) const NUM_CELLS: i32 = 200;
// pub(crate) const NUM_CELLS: i32 = 10;

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

        self.grid.iter_mut()
            .flatten()
            .map(|cell| {
                *cell = matches!(rng.gen_range(1..=100), 1..=20);
            }).collect_vec();
    }

    pub fn clear(&mut self) {
        self.grid.iter_mut()
            .flatten()
            .map(|cell| {
                *cell = false
            }).collect_vec();
    }

    fn valid_point(&self, x: i32, y: i32) -> bool {
        let length = self.length;
        x >= 0 && x < length && y >= 0 && y < length
    }

    fn count_live_neighbors(&self, row: i32, col: i32) -> usize {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(dx, dy)| {
                let x = col + dx;
                let y = row + dy;

                !(*dx == 0 && *dy == 0) && self.valid_point(x, y) && self.grid[y as usize][x as usize]
            }).count()
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

    // TODO: make a declarative macro do this
    pub fn create_glider_gun(&mut self) {
        self.grid[10][10] = true;
        self.grid[10][11] = true;
        self.grid[11][10] = true;
        self.grid[11][11] = true;

        self.grid[8][23] = true;
        self.grid[8][22] = true;
        self.grid[9][21] = true;
        self.grid[10][20] = true;
        self.grid[11][20] = true;
        self.grid[12][20] = true;
        self.grid[13][21] = true;
        self.grid[14][22] = true;
        self.grid[14][23] = true;

        self.grid[11][24] = true;

        self.grid[9][25] = true;
        self.grid[10][26] = true;
        self.grid[11][26] = true;
        self.grid[11][27] = true;
        self.grid[12][26] = true;
        self.grid[13][25] = true;

        self.grid[10][30] = true;
        self.grid[9][30] = true;
        self.grid[8][30] = true;
        self.grid[10][31] = true;
        self.grid[9][31] = true;
        self.grid[8][31] = true;
        self.grid[7][32] = true;
        self.grid[11][32] = true;

        self.grid[6][34] = true;
        self.grid[7][34] = true;
        self.grid[11][34] = true;
        self.grid[12][34] = true;

        self.grid[8][44] = true;
        self.grid[9][44] = true;
        self.grid[8][45] = true;
        self.grid[9][45] = true;
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
