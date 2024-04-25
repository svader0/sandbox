use macroquad::prelude::*;
use crate::elements::{NOTHING, WATER};
use crate::grid::Grid;
use ::rand::{thread_rng, Rng};
#[derive(Clone, Copy, PartialEq)]

pub enum ElementType {
    ImmovableSolid,
    MoveableSolid,
    Liquid,
    Gas,
    PixelGenerator,
    Nothing,
}



pub fn step_moveable_solid(grid: &mut Grid, x: usize, y: usize) {
    // Check if there is air below
    if y + 1 < grid.height && grid.get((x, y + 1)) == NOTHING {
        // Fall down
        grid.move_element((x, y), (x, y + 1));
    } else if y + 1 < grid.height && grid.get((x, y + 1)) == WATER {
        // Swap with water below
        grid.swap_elements((x, y), (x, y + 1));
    } else {
        let mut options = Vec::new();

        if y + 1 < grid.height && x > 0 && grid.get((x - 1, y + 1)) == NOTHING {
            options.push((x - 1, y + 1));
        }

        if y + 1 < grid.height
            && x + 1 < grid.width
            && grid.get((x + 1, y + 1)) == NOTHING
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

pub fn step_immoveable_solid(grid: &mut Grid, x: usize, y: usize) {
    if y + 1 < grid.height && grid.get((x, y + 1)) == WATER {
        grid.swap_elements((x, y), (x, y + 1));
    }
}

pub fn step_gas(grid: &mut Grid, x: usize, y: usize, diffusion_rate: usize) {
    if y > 0 {
        let above = grid.get((x, y - 1));
        if above == NOTHING {
            grid.move_element((x, y), (x, y - 1));
        } else {
            // Attempt to disperse left or right

            let direction = rand::gen_range(0, 2) * 2;

            for i in 1..=diffusion_rate {
                let new_x = (x as usize + direction * i as usize) as usize;

                if new_x < grid.width {
                    if thread_rng().gen_range(0..100) < diffusion_rate * 10 {
                        let target = grid.get((new_x, y));

                        if target == NOTHING {
                            grid.move_element((x, y), (new_x, y));
                            break;
                        }
                    }
                }
            }
        }
    }
}

// This implementation looks pretty good, but is very poor for a couple reasons.
// 1. The water has a weird tendency to flow left.
// 2. We generate a new random number every single time instead of just using a preexisting pseudo-random number, like the
//      frame count.
// 3. The dispersion rate is buggy asf but does finally work.
pub fn step_liquid(grid: &mut Grid, x: usize, y: usize, dispersion_rate: usize) {
    // Check if the water can fall down
    // If it can, move the water down
    // Otherwise, attempt to disperse left or right
    if y < grid.height - 1 {
        let below = grid.get((x, y + 1));
        if below == NOTHING {
            grid.move_element((x, y), (x, y + 1));
        } else {
            // Attempt to disperse left or right

            let direction = rand::gen_range(0, 2) * 2 - 1;

            for i in 1..=dispersion_rate {
                let new_x = (x as i32 + direction * i as i32) as usize;

                if new_x < grid.width {
                    let target = grid.get((new_x, y));

                    if target == NOTHING {
                        grid.move_element((x, y), (new_x, y));
                        break;
                    }
                }
            }
        }
    }
}

pub fn step_pixel_generator(grid: &mut Grid, x: usize, y: usize) {
    // Check if there is air below
    if y + 1 < grid.height && grid.get((x, y + 1)) == NOTHING {
        grid.set((x, y + 1), WATER);
    }
}
