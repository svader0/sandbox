use macroquad::prelude::*;

// Constants
const CELL_SIZE: f32 = 8.0;
const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 96;

// Define enum for different types of elements
#[derive(Clone, Copy, PartialEq)]
enum Element {
    Air,
    Sand,
    Water,
    Wall,
}

impl Element {}

struct Grid {
    width: usize,
    height: usize,
    elements: Vec<Element>,
}

impl Grid {
    // Create a new grid with the given width and height
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            elements: vec![Element::Air; width * height],
        }
    }
    // Get the element at the given position
    fn get(&self, x: usize, y: usize) -> Option<Element> {
        if x < self.width && y < self.height {
            return Some(self.elements[y * self.width + x]);
        }
        None
    }
    // Set the element at the given position
    fn set(&mut self, x: usize, y: usize, value: Element) {
        if x < self.width && y < self.height {
            self.elements[y * self.width + x] = value;
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Sandbox".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: (GRID_WIDTH * CELL_SIZE as usize) as i32,
        window_height: (GRID_HEIGHT * CELL_SIZE as usize) as i32,
        ..Default::default()
    }
}

fn handle_input(grid: &mut Grid, selected_element: &mut Element) {
    if is_key_pressed(KeyCode::Z) {
        *selected_element = Element::Water;
    }
    if is_key_pressed(KeyCode::X) {
        *selected_element = Element::Sand;
    }
    if is_key_pressed(KeyCode::C) {
        *selected_element = Element::Wall;
    }
    if is_mouse_button_down(MouseButton::Left) {
        grid.set(
            (mouse_position().0 / CELL_SIZE) as usize,
            (mouse_position().1 / CELL_SIZE) as usize,
            *selected_element,
        );
    }
}
#[macroquad::main(window_conf())]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut selected_element = Element::Sand;

    // main game loop
    loop {
        clear_background(WHITE);

        handle_input(&mut grid, &mut selected_element);

        // Draw the grid
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if grid.get(x, y) != None {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        // Note to self: unwrap() is used to get the value from the Option. It's like fromJust in Haskell.
                        // Use only when you're sure the value is not None.
                        match grid.get(x, y).unwrap() {
                            Element::Air => WHITE,
                            Element::Sand => GOLD,
                            Element::Water => BLUE,
                            Element::Wall => DARKGRAY,
                        },
                    );
                }
            }
        }

        next_frame().await
    }
}
