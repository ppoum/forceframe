use std::cmp::min;
use std::sync::{Arc, RwLock};
use crate::engine::engine_object::EngineObject;

pub type ThreadEngineObject = dyn EngineObject + Send + Sync;
pub type ThreadEngineObjectPtr = Arc<RwLock<Box<ThreadEngineObject>>>;

pub struct CellMatrix {
    width: u32,
    height: u32,
    cell_size: u32,
    cells: Vec<Vec<Vec<ThreadEngineObjectPtr>>>,
}

impl CellMatrix {
    pub fn new(width: u32, height: u32, cell_size: u32) -> Self {
        CellMatrix {
            width,
            height,
            cell_size,
            cells: vec![vec![Vec::new(); width as usize]; height as usize],
        }
    }

    pub fn get_width(&self) -> u32 { return self.width; }

    pub fn get_height(&self) -> u32 { return self.height; }

    pub fn add_object_to_cell(&mut self, row: usize, col: usize, obj: ThreadEngineObjectPtr) {
        self.cells[row][col].push(obj)
    }

    pub fn get_objects_in_cell(&self, row: usize, col: usize) -> Vec<ThreadEngineObjectPtr> {
        self.cells[row][col].clone()
    }

    pub fn get_objects_in_neighbouring_cells(&self, row: usize, col: usize) -> Vec<ThreadEngineObjectPtr> {
        let mut ans = Vec::new();
        for dx in -1..=1 {
            let c = col as i64 + dx;
            if c < 0 || c >= self.width as i64 { continue; }

            for dy in -1..=1 {
                let r = row as i64 + dy;
                if r < 0 || r >= self.height as i64 { continue; }

                // Valid coords, get cells0
                ans.append(&mut self.get_objects_in_cell(r as usize, c as usize))
            }
        }
        ans
    }

    pub fn update_positions(&mut self) {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                let cell = self.cells[row][col].clone();
                let mut remove_cnt = 0;
                for (i, obj) in cell.iter().enumerate() {
                    let (new_row, new_col) = obj.read().unwrap().get_cell_pos(self.cell_size);
                    // Cap to matrix bounds
                    let new_row = min(new_row, (self.height as usize) - 1);
                    let new_col = min(new_col, (self.width as usize) - 1);
                    if row != new_row || col != new_col {
                        // Object changed cell, update matrix
                        // Remove from old cell
                        self.cells[row][col].remove(i - remove_cnt);
                        remove_cnt += 1;

                        // Add to new cell
                        self.cells[new_row][new_col].push(obj.clone())
                    }
                }
            }
        }
    }
}