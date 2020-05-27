extern crate fixedbitset;

use fixedbitset::FixedBitSet;

#[derive(Default)]
pub struct CellBuffer {
    buffer_0: FixedBitSet,
    buffer_1: FixedBitSet,
    even_buffer_active: bool,
}

impl CellBuffer {
    pub fn new(size: usize) -> CellBuffer {
        let mut buffer_0 = FixedBitSet::with_capacity(size);
        let buffer_1 = FixedBitSet::with_capacity(size);

        for i in 0..size {
            buffer_0.set(i, i % 2 == 0 || i % 7 == 0)
        }

        CellBuffer {
            buffer_0,
            buffer_1,
            even_buffer_active: true,
        }
    }

    pub fn buffers(&self) -> (&FixedBitSet, &FixedBitSet) {
        if self.even_buffer_active {
            (&self.buffer_0, &self.buffer_1)
        } else {
            (&self.buffer_1, &self.buffer_0)
        }
    }

    pub fn buffers_mut(&mut self) -> (&mut FixedBitSet, &mut FixedBitSet) {
        if self.even_buffer_active {
            (&mut self.buffer_0, &mut self.buffer_1)
        } else {
            (&mut self.buffer_1, &mut self.buffer_0)
        }
    }

    pub fn swap(&mut self) {
        self.even_buffer_active = !self.even_buffer_active;
    }

    pub fn update_size(&mut self, size: usize) {
        self.even_buffer_active = true;
        self.buffer_0 = FixedBitSet::with_capacity(size);
        self.buffer_1 = FixedBitSet::with_capacity(size);

        for i in 0..size {
            self.buffer_0.set(i, i % 2 == 0 || i % 7 == 0)
        }
    }

    pub fn set_cells(&mut self, set_cell_indices: Vec<usize>) {
        let (active, inactive) = self.buffers_mut();
        inactive.clone_from(active);
        for i in set_cell_indices {
            inactive.set(i, true);
        }
        self.swap();
    }

    pub fn toggle_cell(&mut self, idx: usize) {
        let (active, inactive) = self.buffers_mut();
        inactive.clone_from(active);
        inactive.toggle(idx);
        self.swap();
    }
}
