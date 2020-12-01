extern crate wasm_bindgen;

mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        //console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        //console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    change: Vec<u8>
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

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
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
                self.change[idx] = (next_cell != self.cells[idx]) as u8;
            }
        }

        self.cells = next.clone();
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        let change = vec![1; height as usize * width as usize];
        Universe {
            width,
            height,
            cells,
            change
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn change(&self) -> *const u8 {
        self.change.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn clear_cells(&mut self) {
        //self.cells.iter_mut().map(|i| *i = Cell::Dead).count();
        for i in 0..self.height * self.width {
            self.cells[i as usize] = Cell::Dead;
            self.change[i as usize] = 1;
        }
    } 
    pub fn set_gliders(&mut self) {
        self.clear_cells();
        let glider = vec![(1, 1), (1, 2), (1, 3), (2, 1), (3, 2)];
        for r in (0..self.height - 10).step_by(10) {
            for c in (0..self.width - 10).step_by(10) {
                for &pos in glider.iter() {
                    let idx = self.get_index(r + pos.0, c + pos.1);
                    self.cells[idx] = Cell::Alive;
                }
            }
        }
    }

    pub fn set_glidergun(&mut self) {
        self.clear_cells();

        let glidergun = 
        vec![(5, 1),(5, 2),(6, 1),(6, 2),(5, 11),(6, 11),(7, 11),(4, 12),(3, 13),(3, 14),
        (8, 12),(9, 13),(9, 14),(6, 15),(4, 16),(5, 17),(6,17),(7,17),(6, 18),(8, 16),(3,21),
        (4, 21),(5, 21),(3, 22),(4, 22),(5, 22),(2, 23),(6, 23),(1, 25),(2, 25),(6, 25),(7, 25), 
        (3, 35),(4, 35),(3, 36),(4, 36)];

        for &pos in glidergun.iter() {
            let idx = self.get_index(pos.0, pos.1);
            self.cells[idx] = Cell::Alive;
        }
        let glider_eater = 
        vec![(0, 0),(0, 1),(1, 0),(1, 2),(2, 2),(3, 2),(3, 3)];
        let offset_y = 90;
        let offset_x = 104;
        for &pos in glider_eater.iter() {
            let idx = self.get_index(pos.0 + offset_y, pos.1 + offset_x);
            self.cells[idx] = Cell::Alive;
        }
    }

    pub fn set_from_rle(&mut self, rle: &str) -> u32 {
        self.clear_cells();

        let mut itr = rle.chars();
        let (mut row, mut col) = (0, 0);
        let (mut colflag, mut ruleflag) = (false, false);
        let (mut pc, mut pr) = (0, 1);
        let mut step = 0;
        while let Some(x) = itr.next() {
            if x == '#' {
               while Some('\n') != itr.next() {} 
            }
            else if x.is_ascii_whitespace() {continue;}
            else if x == 'x' {
                loop {
                    match itr.next() {
                        Some(nx) => {
                            if !ruleflag && nx.is_ascii_digit() {
                                if colflag {
                                    col = col * 10 + nx as u32 - 48;
                                }
                                else {
                                    row = row * 10 + nx as u32 - 48;
                                }
                            }
                            else if nx == '\n' {
                                if row == 0 || col == 0 {return 1;}
                                else if row + 2 > self.height || col + 2 > self.width {return 2;}
                                break;
                            }
                            else if nx == 'y' {colflag = true;}
                            else if nx == 'r' {ruleflag = true;}
                        },
                        None => {break;}
                    }
                }
            }
            else if x == '$' {
                pr += 1;
                if step > 1 {pr += step - 1;} 
                pc = 0;
                step = 0;
                if pr >= self.height {return 1;}
            }
            else if x == '!' {
                break;
            }
            else if x.is_ascii_digit() {
                step = step * 10 + x as u32 - 48;
            }
            else {
                if pc + step > self.width {return 1;}
                match x {
                    'b' => {
                        pc += 1;
                        if step > 1 {pc += step - 1;} 
                        step = 0;
                    },
                    'o' => {
                        if step == 0 {step = 1;}
                        while step > 0 {
                            pc += 1;
                            let idx = self.get_index(pr, pc);
                            self.cells[idx] = Cell::Alive;
                            step -= 1;
                        }
                    },
                    _ => {return 1;}
                }
            }
        }
        return 0;
    }
}

impl fmt::Display for Universe {
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
