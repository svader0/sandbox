use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

use crate::grid::{self, Grid};

#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Air,
    Sand,
    Water,
    Stone,
}

impl Element {
    pub fn to_string(&self) -> &str {
        match self {
            Element::Air => "Air",
            Element::Sand => "Sand",
            Element::Water => "Water",
            Element::Stone => "Stone",
        }
    }
    pub fn get_color(&self) -> Option<Color> {
        match self {
            Element::Air => None,
            Element::Sand => Some(GOLD),
            Element::Water => Some(BLUE),
            Element::Stone => Some(DARKGRAY),
        }
    }

    pub fn step(&self, grid: &mut Grid, x: usize, y: usize) {
        if !grid.is_within_bounds((x, y)) {
            return;
        }

        match self {
            Element::Sand => self.step_sand(grid, x, y),
            Element::Water => self.step_liquid(grid, x, y, 5),
            _ => {}
        }
    }
    pub fn step_sand(&self, grid: &mut Grid, x: usize, y: usize) {
        // Check if there is air below
        if y + 1 < grid::GRID_HEIGHT && grid.get((x, y + 1)) == Element::Air {
            // Fall down
            grid.move_element((x, y), (x, y + 1));
        } else if y + 1 < grid::GRID_HEIGHT && grid.get((x, y + 1)) == Element::Water {
            // Swap with water below
            grid.swap_elements((x, y), (x, y + 1));
        } else {
            let mut options = Vec::new();

            if y + 1 < grid::GRID_HEIGHT && x > 0 && grid.get((x - 1, y + 1)) == Element::Air {
                options.push((x - 1, y + 1));
            }

            if y + 1 < grid::GRID_HEIGHT
                && x + 1 < grid::GRID_WIDTH
                && grid.get((x + 1, y + 1)) == Element::Air
            {
                options.push((x + 1, y + 1));
            }

            if !options.is_empty() {
                let random_index = thread_rng().gen_range(0..options.len());
                let (new_x, new_y) = options[random_index];
                grid.move_element((x, y), (new_x, new_y));
            }
        }
    }

    pub fn step_liquid(&self, grid: &mut Grid, x: usize, y: usize, dispersion_rate: usize) {
        // Check if the water can fall down
        // If it can, move the water down
        // Otherwise, attempt to disperse left or right
        if y < grid::GRID_HEIGHT - 1 {
            let below = grid.get((x, y + 1));
            if below == Element::Air {
                grid.move_element((x, y), (x, y + 1));
            } else {
                // Attempt to disperse left or right

                let direction = rand::gen_range(0, 2) * 2 - 1;

                for i in 1..=dispersion_rate {
                    let new_x = (x as i32 + direction * i as i32) as usize;

                    if new_x < grid::GRID_WIDTH {
                        let target = grid.get((new_x, y));

                        if target == Element::Air {
                            grid.move_element((x, y), (new_x, y));
                            break;
                        }
                    }
                }
            }
        }
    }
}
