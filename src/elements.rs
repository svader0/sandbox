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
        match self {
            Element::Sand => self.step_sand(grid, x, y),
            Element::Water => self.step_liquid(grid, x, y, 5),
            _ => {}
        }
    }
    pub fn step_sand(&self, grid: &mut Grid, x: usize, y: usize) {
        // Check if the sand can fall down
        // If it can, move the sand down
        // Otherwise, check if the sand can move diagonally to the bottom left or right.
        // Randomly choose to move to the left or right if both are possible.
        // If the sand can't move diagonally, it will stay in place.
        if y < grid::GRID_HEIGHT - 1 {
            let below = grid.get((x, y + 1));
            if below == Element::Air {
                grid.move_element((x, y), (x, y + 1));
            } else if below == Element::Water {
                grid.swap_elements(x, y, x, y + 1);
            } else if x > 0 && x < grid::GRID_WIDTH - 1 {
                // Check if the sand can fall to the left or right
                let left = grid.get((x - 1, y + 1));
                let right = grid.get((x + 1, y + 1));
                if left == Element::Air && right == Element::Air {
                    let direction = rand::gen_range(0, 2);
                    // Randomly choose to fall to the left or right
                    if direction == 0 {
                        grid.move_element((x, y), (x - 1, y + 1));
                    } else {
                        grid.move_element((x, y), (x + 1, y + 1));
                    }
                } else if left == Element::Air {
                    grid.move_element((x, y), (x - 1, y + 1));
                } else if right == Element::Air {
                    grid.move_element((x, y), (x + 1, y + 1));
                }
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
