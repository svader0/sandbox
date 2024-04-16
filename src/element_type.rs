use crate::elements::Element;
use crate::grid::{self, Grid};

use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};
pub enum ElementType {
    ImmovableSolid,
    MoveableSolid,
    Liquid,
    Gas,
    PixelGenerator,
    Nothing
}



pub fn step_moveable_solid(element: &Element, grid: &mut Grid, x: usize, y: usize) {
    // Check if there is air below
    if y + 1 < grid::GRID_HEIGHT && grid.get((x, y + 1)) == Element::Nothing {
        // Fall down
        grid.move_element((x, y), (x, y + 1));
    } else if y + 1 < grid::GRID_HEIGHT && grid.get((x, y + 1)) == Element::Water {
        // Swap with water below
        grid.swap_elements((x, y), (x, y + 1));
    } else {
        let mut options = Vec::new();

        if y + 1 < grid::GRID_HEIGHT && x > 0 && grid.get((x - 1, y + 1)) == Element::Nothing {
            options.push((x - 1, y + 1));
        }

        if y + 1 < grid::GRID_HEIGHT
            && x + 1 < grid::GRID_WIDTH
            && grid.get((x + 1, y + 1)) == Element::Nothing
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

pub fn step_immoveable_solid(element: &Element, grid: &mut Grid, x: usize, y: usize) {
    if y + 1 < grid::GRID_HEIGHT && grid.get((x, y + 1)) == Element::Water {
        grid.swap_elements((x, y), (x, y + 1));
    }
}
pub fn step_liquid(element: &Element, grid: &mut Grid, x: usize, y: usize, dispersion_rate: usize) {
    // Check if the water can fall down
    // If it can, move the water down
    // Otherwise, attempt to disperse left or right
    if y < grid::GRID_HEIGHT - 1 {
        let below = grid.get((x, y + 1));
        if below == Element::Nothing {
            grid.move_element((x, y), (x, y + 1));
        } else {
            // Attempt to disperse left or right

            let direction = rand::gen_range(0, 2) * 2 - 1;

            for i in 1..=dispersion_rate {
                let new_x = (x as i32 + direction * i as i32) as usize;

                if new_x < grid::GRID_WIDTH {
                    let target = grid.get((new_x, y));

                    if target == Element::Nothing {
                        grid.move_element((x, y), (new_x, y));
                        break;
                    }
                }
            }
        }
    }
}

pub fn step_pixel_generator(element: &Element, grid: &mut Grid, x: usize, y: usize) {
    // Check if there is air below
    if y + 1 < grid::GRID_HEIGHT && grid.get((x, y + 1)) == Element::Nothing {
        grid.set((x, y + 1), Element::Water);
    }
}