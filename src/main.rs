use std::mem::swap;

use macroquad::prelude::*;

// Define enum for different types of elements
#[derive(Clone, Copy, PartialEq)]
enum Element {
    Empty,
    Sand,
    Water,
    Wall,
}

// Define a struct to represent the game grid
struct Grid {
    cells: Vec<Element>,
    width: usize,
    height: usize,
}

// Implement methods for the Grid struct
impl Grid {
    fn new(width: usize, height: usize) -> Self {
        // Initialize the grid with empty cells
        let cells = vec![Element::Empty; width * height];

        Grid {
            cells,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Element {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else {
            Element::Wall
        }
    }

    fn set(&mut self, x: usize, y: usize, element: Element) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = element;
        }
    }

    fn update(&mut self) {
        // Iterate through each cell in the grid
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                match self.get(x, y) {
                    Element::Sand => {
                        if x < 1 || x >= self.width - 1 || y >= self.height - 1 {
                            continue;
                        }
                        // If the cell below is empty, move sand down
                        if self.get(x, y + 1) == Element::Empty {
                            self.set(x, y + 1, Element::Sand);
                            self.set(x, y, Element::Empty);
                        } else if self.get(x - 1, y + 1) == Element::Empty {
                            // If the cell below and to the left is empty, move sand diagonally left
                            self.set(x - 1, y + 1, Element::Sand);
                            self.set(x, y, Element::Empty);
                        } else if self.get(x + 1, y + 1) == Element::Empty {
                            // If the cell below and to the right is empty, move sand diagonally right
                            self.set(x + 1, y + 1, Element::Sand);
                            self.set(x, y, Element::Empty);
                        } else if self.get(x, y - 1) == Element::Water {
                            swap_elements(self, x, y, x, y - 1);
                        } else if self.get(x - 1, y - 1) == Element::Water {
                            swap_elements(self, x, y, x - 1, y - 1);
                        } else if self.get(x + 1, y - 1) == Element::Water {
                            swap_elements(self, x, y, x + 1, y - 1);
                        }
                    }
                    Element::Water => {
                        if x < 1 || x >= self.width - 1 || y >= self.height - 1 {
                            continue;
                        }

                        // TODO: do this
                    }
                    _ => {}
                }
            }
        }
    }
}

// Constants
const CELL_SIZE: f32 = 5.0;
const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 96;

// Helper function to convert grid coordinates to screen coordinates
fn grid_to_screen(x: usize, y: usize) -> Vec2 {
    Vec2::new(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE)
}

fn swap_elements(grid: &mut Grid, x1: usize, y1: usize, x2: usize, y2: usize) {
    let temp = grid.get(x1, y1);
    grid.set(x1, y1, grid.get(x2, y2));
    grid.set(x2, y2, temp);
}

#[macroquad::main("Falling Sand Simulation")]
async fn main() {
    // Initialize the game grid
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    // Set up initial conditions
    grid.set(GRID_WIDTH / 2, 0, Element::Sand);

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_mouse_button_down(MouseButton::Left) {
            grid.set(
                (mouse_position().0 / CELL_SIZE) as usize,
                (mouse_position().1 / CELL_SIZE) as usize,
                Element::Sand,
            );
        }
        if is_key_down(KeyCode::G) {
            grid.set(
                (mouse_position().0 / CELL_SIZE) as usize,
                (mouse_position().1 / CELL_SIZE) as usize,
                Element::Water,
            );
        }
        // Update the grid
        grid.update();

        // Draw the elements on the screen
        clear_background(BLACK);
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let color = match grid.get(x, y) {
                    Element::Empty => BLACK,
                    Element::Sand => YELLOW,
                    Element::Water => BLUE,
                    Element::Wall => RED,
                };
                draw_rectangle(
                    grid_to_screen(x, y).x,
                    grid_to_screen(x, y).y,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }

        next_frame().await
    }
}
