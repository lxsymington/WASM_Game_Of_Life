extern crate fixedbitset;
extern crate js_sys;

mod cell_buffer;
mod graph;
mod timer;
mod utils;

use cell_buffer::CellBuffer;
use fixedbitset::FixedBitSet;
use graph::Graph;
use std::fmt;
use timer::Timer;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Default)]
pub struct Universe {
    cell_buffer: CellBuffer,
    graph: Graph,
    height: u32,
    width: u32,
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&mut self) -> &FixedBitSet {
        let (active, _) = self.cell_buffer.buffers();
        active
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        let set_cell_indices: Vec<usize> = cells
            .iter()
            .map(|(row, col)| self.get_index(*row, *col))
            .collect();

        self.cell_buffer.set_cells(set_cell_indices);
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cell_buffer.update_size((width * self.height) as usize);
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cell_buffer.update_size((self.width * height) as usize);
    }

    pub fn new(
        width: u32,
        height: u32,
        cell_width: f32,
        cell_height: f32,
        cell_gap: f32,
    ) -> Universe {
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let cell_buffer = CellBuffer::new(size);
        let graph = Graph::new(width, height, cell_width, cell_height, cell_gap);

        Universe {
            cell_buffer,
            graph,
            height,
            width,
        }
    }

    pub fn random_reset(&mut self) {
        let (_, inactive) = self.cell_buffer.buffers_mut();

        for idx in 0..inactive.len() {
            inactive.set(idx, js_sys::Math::random() > 0.5);
        }

        self.cell_buffer.swap();
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let (active, _) = self.cell_buffer.buffers();
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
        count += active[nw] as u8;

        let n = self.get_index(north, column);
        count += active[n] as u8;

        let ne = self.get_index(north, east);
        count += active[ne] as u8;

        let w = self.get_index(row, west);
        count += active[w] as u8;

        let e = self.get_index(row, east);
        count += active[e] as u8;

        let sw = self.get_index(south, west);
        count += active[sw] as u8;

        let s = self.get_index(south, column);
        count += active[s] as u8;

        let se = self.get_index(south, east);
        count += active[se] as u8;

        count
    }

    pub fn tick(&mut self, iterations: usize) {
        let _timer = Timer::new("Universe tick");

        for _i in 0..iterations {
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = {
                        let (active, _) = { self.cell_buffer.buffers() };
                        active[idx]
                    };
                    let live_neighbours = self.live_neighbour_count(row, col);
                    let (_, inactive) = self.cell_buffer.buffers_mut();

                    inactive.set(
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

        self.cell_buffer.swap();
    }

    pub fn toggle_cell(&mut self, idx: usize) {
        self.cell_buffer.toggle_cell(idx);
    }

    fn insert_pattern(&mut self, row: u32, col: u32, pattern: Vec<(i32, i32)>) {
        let pattern_indices = pattern
            .iter()
            .map(|(row_delta, col_delta)| {
                self.get_index(
                    (row as i32 + *row_delta) as u32,
                    (col as i32 + *col_delta) as u32,
                )
            })
            .collect();

        self.cell_buffer.set_cells(pattern_indices);
    }

    pub fn insert_glider(&mut self, row: u32, col: u32) {
        let glider: Vec<(i32, i32)> = vec![(-1, -1), (0, 0), (0, 1), (1, -1), (1, 0)];

        self.insert_pattern(row, col, glider);
    }

    pub fn insert_pulsar(&mut self, row: u32, col: u32) {
        let pulsar = vec![
            (-6, -4),
            (-6, -3),
            (-6, -2),
            (-6, 2),
            (-6, 3),
            (-6, 4),
            (-4, -6),
            (-4, -1),
            (-4, 1),
            (-4, 6),
            (-3, -6),
            (-3, -1),
            (-3, 1),
            (-3, 6),
            (-2, -6),
            (-2, -1),
            (-2, 1),
            (-2, 6),
            (-1, -4),
            (-1, -3),
            (-1, -2),
            (-1, 2),
            (-1, 3),
            (-1, 4),
            (1, -4),
            (1, -3),
            (1, -2),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, -6),
            (2, -1),
            (2, 1),
            (2, 6),
            (3, -6),
            (3, -1),
            (3, 1),
            (3, 6),
            (4, -6),
            (4, -1),
            (4, 1),
            (4, 6),
            (6, -4),
            (6, -3),
            (6, -2),
            (6, 2),
            (6, 3),
            (6, 4),
        ];

        self.insert_pattern(row, col, pulsar);
    }

    pub fn cell_coords(&mut self) -> *const f32 {
        let _timer = Timer::new("Cell Coords");
        let (active, _) = self.cell_buffer.buffers();

        self.graph.cell_coords(active, self.width, self.height)
    }

    pub fn cell_coords_count(&self) -> u32 {
        let _timer = Timer::new("Cell Coords Count");
        self.graph.cell_coords_count()
    }

    pub fn grid_line_coords(&mut self) -> *const f32 {
        let _timer = Timer::new("Grid Line Coords");
        self.graph.grid_line_coords(self.width, self.height)
    }

    pub fn grid_line_coords_count(&self) -> u32 {
        let _timer = Timer::new("Grid Line Coords Count");
        self.graph.grid_line_coords_count()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (active, _) = self.cell_buffer.buffers();
        for line in active.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                writeln!(f, "{}", symbol)?;
            }
        }

        Ok(())
    }
}
