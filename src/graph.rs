extern crate fixedbitset;

use fixedbitset::FixedBitSet;

#[derive(Default)]
pub struct Graph {
    cell_dimensions: (f32, f32),
    even_line_buffer_active: bool,
    even_rect_buffer_active: bool,
    line_buffer_0: Vec<f32>,
    line_buffer_1: Vec<f32>,
    rect_buffer_0: Vec<f32>,
    rect_buffer_1: Vec<f32>,
}

impl Graph {
    pub fn new(width: u32, height: u32, cell_size: f32, cell_gap: f32) -> Graph {
        let min_line_capacity = ((width + height + 2) * 2) as usize;
        let initial_rect_capacity = (width * height * 6) as usize;

        Graph {
            cell_dimensions: (cell_size, cell_gap),
            even_line_buffer_active: true,
            even_rect_buffer_active: true,
            line_buffer_0: Vec::with_capacity(min_line_capacity),
            line_buffer_1: Vec::with_capacity(min_line_capacity),
            rect_buffer_0: Vec::with_capacity(initial_rect_capacity),
            rect_buffer_1: Vec::with_capacity(initial_rect_capacity),
        }
    }

    pub fn update_size(&mut self, width: u32, height: u32, cell_size: f32, cell_gap: f32) {
        let min_line_capacity = ((width + height + 2) * 2) as usize;
        let rect_capacity = (width * height * 6) as usize;

        self.cell_dimensions = (cell_size, cell_gap);
        self.even_line_buffer_active = true;
        self.even_rect_buffer_active = true;
        let current_line_capacity = self.line_buffer_0.capacity();
        if min_line_capacity > current_line_capacity {
            self.line_buffer_0
                .reserve(min_line_capacity - current_line_capacity);
            self.line_buffer_1
                .reserve(min_line_capacity - current_line_capacity);
            self.line_buffer_0.resize(min_line_capacity, 0.0);
            self.line_buffer_1.resize(min_line_capacity, 0.0);
        } else {
            self.line_buffer_0.resize(min_line_capacity, 0.0);
            self.line_buffer_1.resize(min_line_capacity, 0.0);
            self.line_buffer_0.shrink_to_fit();
            self.line_buffer_1.shrink_to_fit();
        }

        let current_rect_capacity = self.rect_buffer_0.capacity();
        if rect_capacity > current_rect_capacity {
            self.rect_buffer_0
                .reserve(rect_capacity - current_rect_capacity);
            self.rect_buffer_1
                .reserve(rect_capacity - current_rect_capacity);
            self.rect_buffer_0.resize(rect_capacity, 0.0);
            self.rect_buffer_1.resize(rect_capacity, 0.0);
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
        let (cell_size, gap) = self.cell_dimensions;
        let (_, inactive) = self.rect_buffer_mut();
        let line_x_size = gap / (2.0 * cell_size * width as f32);
        let line_y_size = gap / (2.0 * cell_size * height as f32);
        let col_size = 1.0 / width as f32;
        let row_size = 1.0 / height as f32;

        inactive.clear();

        for c in cells.ones() {
            let row = (c as f32 / width as f32).floor() as f32;
            let col = c as f32 - row * width as f32;
            let x1 = -1.0 + 2.0 * col * col_size + line_x_size;
            let x2 = x1 + 2.0 * col_size - line_x_size;
            let y1 = 1.0 - 2.0 * row * row_size - line_y_size;
            let y2 = y1 - 2.0 * row_size + line_y_size;
            let mut rect = vec![x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
            inactive.append(&mut rect);
        }

        self.rect_buffer_swap();
    }
}
