use crate::elements::{Element, AIR, FIRE, MAZE, NOTHING, WATER};
use crate::grid::Grid;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
#[derive(Clone, Copy, PartialEq)]

pub enum ElementType {
    ImmovableSolid,
    MoveableSolid,
    Liquid,
    Gas,
    PixelGenerator,
    Maze,
    Nothing,
    Fire,
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

        if y + 1 < grid.height && x + 1 < grid.width && grid.get((x + 1, y + 1)) == NOTHING {
            options.push((x + 1, y + 1));
        }

        if !options.is_empty() {
            let random_index = rand::gen_range(0, options.len());
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

pub fn step_fire(grid: &mut Grid, x: usize, y: usize) {
    let mut rng = thread_rng();
    let upward_chance = 0.7;

    // Check if the pixel above is empty and within grid bounds
    if y > 0 && grid.get((x, y - 1)) == NOTHING {
        // Move upward with a chance based on upward_chance
        if rng.gen::<f32>() < upward_chance {
            grid.move_element((x, y), (x, y - 1));
            return; // Fire moves only once per step
        }
    }

    // If no upward movement occurred, the fire drifts randomly
    let drift_direction = rng.gen_range(-1..=1); // -1 for left, 0 for no drift, 1 for right
    let new_x = (x as i32 + drift_direction) as usize;

    // Check if the new position is within grid bounds and empty
    if new_x < grid.width && grid.get((new_x, y)) == NOTHING {
        grid.move_element((x, y), (new_x, y));
    } else {
        // If no movement is possible, the fire dies out, turning into NOTHING
        grid.set((x, y), NOTHING);
    }
}

pub fn step_pixel_generator(grid: &mut Grid, x: usize, y: usize) {
    // Check if there is air below
    if y + 1 < grid.height && grid.get((x, y + 1)) == NOTHING {
        grid.set((x, y + 1), WATER);
    }
}

// Maze is a Life-like cellular automaton in which cells survive from one generation to the next if they have at least 1 and at most 5 neighbours. Cells are born if they have exactly 3 neighbours. This resembles Conway's Game of Life in some ways, but it is rather more difficult for cells to die off, and random starting patterns tend to evolve into complex growing maze-like structures with well-defined walls outlining corridors.
// https://conwaylife.com/wiki/OCA:Maze
pub fn step_maze(grid: &mut Grid, x: usize, y: usize) {
    let mut maze_neighbors = 0;

    // Check all neighboring cells
    for dx in -1..=1 {
        for dy in -1..=1 {
            // Skip the current cell
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // Check if the neighboring cell is within the grid bounds
            if nx >= 0 && nx < grid.width as i32 && ny >= 0 && ny < grid.height as i32 {
                let neighbor = grid.get((nx as usize, ny as usize));

                // Check if the neighboring cell is not a maze cell
                if neighbor != MAZE {
                    // Check if the neighboring cell has 3 neighbors
                    let neighbor_neighbors = count_maze_neighbors(grid, nx as usize, ny as usize);

                    // Set the neighboring cell to maze if it has 3 neighbors
                    if neighbor_neighbors == 3 {
                        grid.set((nx as usize, ny as usize), MAZE);
                    }
                }
            }
        }
    }

    // Check the current cell
    if grid.get((x, y)) == MAZE {
        let current_neighbors = count_maze_neighbors(grid, x, y);

        // Set the current cell to nothing if it has less than 1 or more than 5 neighbors
        if current_neighbors < 1 || current_neighbors > 5 {
            grid.set((x, y), NOTHING);
        }
    }
}

fn count_maze_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    let mut neighbor_neighbors = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            // Skip the current cell and the neighboring cell
            if (dx == 0 && dy == 0) || (dx == 0 && dy == 0) {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // Check if the neighbor of the current cell is within the grid bounds
            if nx >= 0 && nx < grid.width as i32 && ny >= 0 && ny < grid.height as i32 {
                let neighbor = grid.get((nx as usize, ny as usize));

                // Check if the neighbor of the current cell is a maze cell
                if neighbor == MAZE {
                    neighbor_neighbors += 1;
                }
            }
        }
    }

    neighbor_neighbors
}
