extern crate fixedbitset;
extern crate js_sys;
extern crate web_sys;

mod utils;

use fixedbitset::FixedBitSet;
use std::fmt;
use wasm_bindgen::prelude::*;
// use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Default)]
pub struct Universe {
    alive_coords: Vec<f32>,
    alive_count: u32,
    cell_gap: u32,
    cell_size: u32,
    cells: FixedBitSet,
    grid_line_coords: Vec<f32>,
    height: u32,
    width: u32,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    fn update_alive_count(&mut self) {
        self.alive_count = Universe::alive_cells_count(&self.cells);
    }

    fn update_alive_coords(&mut self) {
        self.alive_coords = Universe::alive_cells_coords(
            &self.cells,
            self.width,
            self.height,
            self.cell_size,
            self.cell_gap,
        );
    }

    fn update_grid_line_coords(&mut self) {
        self.grid_line_coords = Universe::grid_lines(self.width, self.height);
    }

    fn alive_cells_count(cells: &FixedBitSet) -> u32 {
        cells.count_ones(..) as u32
    }

    fn alive_cells_coords(
        cells: &FixedBitSet,
        width: u32,
        height: u32,
        cell_size: u32,
        cell_gap: u32,
    ) -> Vec<f32> {
        cells
            .ones()
            .flat_map(|c| -> Vec<f32> {
                let row = (c as f32 / width as f32).floor() as u32;
                let col = c as u32 % width;
                let total_width: f32 = (width * (cell_gap + cell_size) + cell_gap) as f32;
                let total_height: f32 = (height * (cell_gap + cell_size) + cell_gap) as f32;
                let x1: f32 =
                    -1.0 + (col * 2 * (cell_gap + cell_size) + 2 * cell_gap) as f32 / total_width;
                let x2: f32 = x1 + (2 * cell_size) as f32 / total_width;
                let y1: f32 =
                    1.0 - (row * 2 * (cell_gap + cell_size) + 2 * cell_gap) as f32 / total_height;
                let y2: f32 = y1 - (2 * cell_size) as f32 / total_height;
                vec![x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2]
            })
            .collect()
    }

    fn grid_lines(width: u32, height: u32) -> Vec<f32> {
        let vert_lines: Vec<Vec<f32>> = (0..=width)
            .map(|i| {
                let x: f32 = -1.0 + 2.0 * i as f32 / width as f32;
                vec![x, 1.0, x, -1.0]
            })
            .collect();

        let hori_lines: Vec<Vec<f32>> = (0..=height)
            .map(|i| {
                let y = -1.0 + 2.0 * i as f32 / height as f32;
                vec![1.0, y, -1.0, y]
            })
            .collect();

        vert_lines
            .into_iter()
            .chain(hori_lines.into_iter())
            .flatten()
            .collect()
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);

            self.cells.set(idx, true);
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((width * self.height) as usize);
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * height) as usize);
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn alive_count(&mut self) -> u32 {
        self.update_alive_count();

        self.alive_count
    }

    pub fn cell_coords(&mut self) -> *const f32 {
        self.update_alive_coords();

        self.alive_coords.as_ptr()
    }

    pub fn cell_coords_count(&self) -> u32 {
        self.alive_coords.len() as u32
    }

    pub fn grid_line_coords(&mut self) -> *const f32 {
        self.update_grid_line_coords();

        self.grid_line_coords.as_ptr()
    }

    pub fn grid_line_coords_count(&self) -> u32 {
        self.grid_line_coords.len() as u32
    }

    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe tick");

        let mut next = {
            // let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            // let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbours = self.live_neighbour_count(row, col);

                    next.set(
                        idx,
                        match (cell, live_neighbours) {
                            // Rule 1: Any live cell with fewer than two live neighbours
                            // dies, as if caused by underpopulation.
                            (true, x) if x < 2 => false,
                            // Rule 2: Any live cell with two or three live neighbours
                            // lives on to the generation.
                            (true, 2) | (true, 3) => true,
                            // Rule 3: Any live cell with more than three live
                            // neighbours dies, as if by overpopulation.
                            (true, x) if x > 3 => false,
                            // Rule 4: Any dead cell with exactly three live neighbours
                            // becomes a live cell, as if by reproduction.
                            (false, 3) => true,
                            // All other cells remain in the same state.
                            (otherwise, _) => otherwise,
                        },
                    );
                }
            }
        }

        // let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    pub fn new(width: u32, height: u32, cell_size: u32, cell_gap: u32) -> Universe {
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0)
        }

        let alive_count = Universe::alive_cells_count(&cells);
        let alive_coords = Universe::alive_cells_coords(&cells, width, height, cell_size, cell_gap);
        let grid_line_coords = Universe::grid_lines(width, height);

        Universe {
            alive_coords,
            alive_count,
            cell_gap,
            cell_size,
            cells,
            grid_line_coords,
            height,
            width,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
    }

    pub fn insert_glider(&mut self, row: u32, column: u32) {
        self.set_cells(&[
            (row - 1, column - 1),
            (row, column),
            (row, column + 1),
            (row + 1, column - 1),
            (row + 1, column),
        ]);
    }

    pub fn insert_pulsar(&mut self, row: u32, column: u32) {
        self.set_cells(&[
            (row - 6, column - 4),
            (row - 6, column - 3),
            (row - 6, column - 2),
            (row - 6, column + 2),
            (row - 6, column + 3),
            (row - 6, column + 4),
            (row - 4, column - 6),
            (row - 4, column - 1),
            (row - 4, column + 1),
            (row - 4, column + 6),
            (row - 3, column - 6),
            (row - 3, column - 1),
            (row - 3, column + 1),
            (row - 3, column + 6),
            (row - 2, column - 6),
            (row - 2, column - 1),
            (row - 2, column + 1),
            (row - 2, column + 6),
            (row - 1, column - 4),
            (row - 1, column - 3),
            (row - 1, column - 2),
            (row - 1, column + 2),
            (row - 1, column + 3),
            (row - 1, column + 4),
            (row + 1, column - 4),
            (row + 1, column - 3),
            (row + 1, column - 2),
            (row + 1, column + 2),
            (row + 1, column + 3),
            (row + 1, column + 4),
            (row + 2, column - 6),
            (row + 2, column - 1),
            (row + 2, column + 1),
            (row + 2, column + 6),
            (row + 3, column - 6),
            (row + 3, column - 1),
            (row + 3, column + 1),
            (row + 3, column + 6),
            (row + 4, column - 6),
            (row + 4, column - 1),
            (row + 4, column + 1),
            (row + 4, column + 6),
            (row + 6, column - 4),
            (row + 6, column - 3),
            (row + 6, column - 2),
            (row + 6, column + 2),
            (row + 6, column + 3),
            (row + 6, column + 4),
        ]);
    }

    pub fn random_reset(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                next.set(idx, js_sys::Math::random() > 0.5);
            }
        }

        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                writeln!(f, "{}", symbol)?;
            }
        }

        Ok(())
    }
}

// pub struct Timer<'a> {
//     name: &'a str,
// }

// impl<'a> Timer<'a> {
//     pub fn new(name: &'a str) -> Timer<'a> {
//         console::time_with_label(name);
//         Timer { name }
//     }
// }

// impl<'a> Drop for Timer<'a> {
//     fn drop(&mut self) {
//         console::time_end_with_label(self.name);
//     }
// }
