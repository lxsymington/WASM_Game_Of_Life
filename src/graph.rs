extern crate fixedbitset;

use fixedbitset::FixedBitSet;

#[derive(Default)]
pub struct Graph {
    cell_dimensions: (f32, f32, f32),
    even_line_buffer_active: bool,
    even_rect_buffer_active: bool,
    line_buffer_0: Vec<f32>,
    line_buffer_1: Vec<f32>,
    rect_buffer_0: Vec<f32>,
    rect_buffer_1: Vec<f32>,
}

impl Graph {
    pub fn new(width: u32, height: u32, cell_width: f32, cell_height: f32, cell_gap: f32) -> Graph {
        let line_capacity = ((width + height + 2) * 2) as usize;
        let rect_capacity = (width * height * 6) as usize;

        Graph {
            cell_dimensions: (cell_width, cell_height, cell_gap),
            even_line_buffer_active: true,
            even_rect_buffer_active: true,
            line_buffer_0: Vec::with_capacity(line_capacity),
            line_buffer_1: Vec::with_capacity(line_capacity),
            rect_buffer_0: Vec::with_capacity(rect_capacity),
            rect_buffer_1: Vec::with_capacity(rect_capacity),
        }
    }

    pub fn grid_line_coords(&mut self, width: u32, height: u32) -> *const f32 {
        self.update_grid_lines(width, height);

        if self.even_line_buffer_active {
            self.line_buffer_0.as_ptr()
        } else {
            self.line_buffer_1.as_ptr()
        }
    }

    pub fn grid_line_coords_count(&self) -> u32 {
        let (active, _) = self.line_buffer();
        active.len() as u32
    }

    fn line_buffer(&self) -> (&Vec<f32>, &Vec<f32>) {
        if self.even_line_buffer_active {
            (&self.line_buffer_0, &self.line_buffer_1)
        } else {
            (&self.line_buffer_1, &self.line_buffer_0)
        }
    }

    fn line_buffer_mut(&mut self) -> (&mut Vec<f32>, &mut Vec<f32>) {
        if self.even_line_buffer_active {
            (&mut self.line_buffer_0, &mut self.line_buffer_1)
        } else {
            (&mut self.line_buffer_1, &mut self.line_buffer_0)
        }
    }

    fn line_buffer_swap(&mut self) {
        self.even_line_buffer_active = !self.even_line_buffer_active;
    }

    fn update_grid_lines(&mut self, width: u32, height: u32) {
        let (_, inactive) = self.line_buffer_mut();
        inactive.clear();

        for i in 0..=width {
            let x: f32 = -1.0 + 2.0 * i as f32 / width as f32;
            let mut line = vec![x, 1.0, x, -1.0];
            inactive.append(&mut line);
        }

        for i in 0..=height {
            let y = -1.0 + 2.0 * i as f32 / height as f32;
            let mut line = vec![1.0, y, -1.0, y];
            inactive.append(&mut line);
        }

        self.line_buffer_swap()
    }

    pub fn cell_coords(&mut self, cells: &FixedBitSet, width: u32, height: u32) -> *const f32 {
        self.update_cell_coords(cells, width, height);

        if self.even_rect_buffer_active {
            self.rect_buffer_0.as_ptr()
        } else {
            self.rect_buffer_1.as_ptr()
        }
    }

    pub fn cell_coords_count(&self) -> u32 {
        let (active, _) = self.rect_buffer();
        active.len() as u32
    }

    fn rect_buffer(&self) -> (&Vec<f32>, &Vec<f32>) {
        if self.even_rect_buffer_active {
            (&self.rect_buffer_0, &self.rect_buffer_1)
        } else {
            (&self.rect_buffer_1, &self.rect_buffer_0)
        }
    }

    fn rect_buffer_mut(&mut self) -> (&mut Vec<f32>, &mut Vec<f32>) {
        if self.even_rect_buffer_active {
            (&mut self.rect_buffer_0, &mut self.rect_buffer_1)
        } else {
            (&mut self.rect_buffer_1, &mut self.rect_buffer_0)
        }
    }

    fn rect_buffer_swap(&mut self) {
        self.even_rect_buffer_active = !self.even_rect_buffer_active;
    }

    fn update_cell_coords(&mut self, cells: &FixedBitSet, width: u32, height: u32) {
        let (cell_width, cell_height, gap) = self.cell_dimensions;
        let get_gl_coord = |i: u32, size: f32| (i as f32 * (gap + size) + gap);
        let (_, inactive) = self.rect_buffer_mut();

        inactive.clear();

        for c in cells.ones() {
            let row = (c as f32 / width as f32).floor() as u32;
            let col = c as u32 % width;
            let total_width: f32 = get_gl_coord(width, cell_width);
            let total_height: f32 = get_gl_coord(height, cell_height);
            let x1: f32 = -1.0 + (2.0 * get_gl_coord(col, cell_width)) / total_width;
            let x2: f32 = x1 + (2.0 * cell_width) / total_width;
            let y1: f32 = 1.0 - (2.0 * get_gl_coord(row, cell_height)) / total_height;
            let y2: f32 = y1 - (2.0 * cell_height) / total_height;
            let mut rect = vec![x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
            inactive.append(&mut rect);
        }

        self.rect_buffer_swap();
    }
}
