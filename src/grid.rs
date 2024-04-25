use crate::elements::{Element, NOTHING};

// constants
const GRID_WIDTH: usize = 244;
const GRID_HEIGHT: usize = 122;

pub type Vector2 = (usize, usize);

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    elements: Vec<Element>,
}

impl Grid {
    // Create a new grid with the given width and height
    pub fn new(screen_height: f32) -> Grid {
        Grid {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            cell_size: screen_height * 0.8 / GRID_HEIGHT as f32,
            elements: vec![NOTHING; GRID_WIDTH * GRID_HEIGHT],
        }
    }
    // Get the element at the given position
    pub fn get(&self, pos: Vector2) -> Element {
        // Return air if the position is out of bounds
        // Otherwise, return the element at the given position
        if pos.0 < self.width && pos.1 < self.height {
            return self.elements[pos.1 * self.width + pos.0];
        }
        NOTHING
    }
    // Set the element at the given position
    pub fn set(&mut self, pos: Vector2, value: Element) {
        if pos.0 < self.width && pos.1 < self.height {
            self.elements[pos.1 * self.width + pos.0] = value;
        }
    }

    // Move the element at the given position to the new position
    pub fn move_element(&mut self, pos: Vector2, new_pos: Vector2) {
        let element = self.get(pos);
        self.set(pos, NOTHING);
        self.set(new_pos, element);
    }

    // Swap the elements at the given positions
    pub fn swap_elements(&mut self, pos: Vector2, new_pos: Vector2) {
        let element1 = self.get(pos);
        let element2 = self.get(new_pos);
        self.set(pos, element2);
        self.set(new_pos, element1);
    }

    // Update the grid
    pub fn update(&mut self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let element = self.get((x, y));
                element.step(self, x, y);
            }
        }
    }
    
    pub fn update_cell_size(&mut self, screen_height: f32) {
        self.cell_size = screen_height * 0.8 / GRID_HEIGHT as f32;
    }

    // Apply the function to each element in between two positions
    pub fn traverse_line<F>(&mut self, start: Vector2, end: Vector2, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        let dx = end.0 as isize - start.0 as isize;
        let dy = end.1 as isize - start.1 as isize;
        let steps = if dx.abs() > dy.abs() {
            dx.abs()
        } else {
            dy.abs()
        } as f32;
        let x_increment = dx as f32 / steps;
        let y_increment = dy as f32 / steps;
        let mut x = start.0 as f32;
        let mut y = start.1 as f32;
        for _ in 0..steps as usize {
            f(x as usize, y as usize);
            x += x_increment;
            y += y_increment;
        }
    }
    pub fn is_within_bounds(&self, pos: Vector2) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    pub fn reset(&mut self) {
        self.elements = vec![NOTHING; self.width * self.height];
    }


}
