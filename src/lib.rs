mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Graph {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Graph {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_count(&self, row: u32, col: u32) -> u32 {
        let dir: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut count = 0;
        for (dx, dy) in dir {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;
            if new_row >= 0
                && new_row < self.height as i32
                && new_col >= 0
                && new_col < self.width as i32
            {
                let idx = self.get_index(new_row as u32, new_col as u32);
                if self.cells[idx] == Cell::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_count = self.live_count(row, col);
                let new_cell = match (cell, live_count) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                new_cells[idx] = new_cell;
            }
        }
        self.cells = new_cells;
    }

    pub fn new(w: u32, h: u32) -> Graph {
        let cells = (0..(w * h))
            .map(|i| {
                if i % 2 == 0 || i % 3 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect::<Vec<Cell>>();

        Graph {
            width: w,
            height: h,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
