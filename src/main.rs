use macroquad::prelude::*;

pub mod elements;
pub mod grid;
pub mod element_type;
use elements::Element;
use grid::Grid;

/*
    TODO:
    1. use traverse_line to draw smoother lines when drawing with the mouse
    2. add a way to clear the grid
    3. fix sand behavior (it does the same thing that water used to do)
        - It also doesn't displace the water properly
    4. add realism to elements
    5. multithreading?
*/

// Constants
const CELL_SIZE: f32 = 3.0;
const BACKGROUND_COLOR: Color = BLACK;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sandbox".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: (grid::GRID_WIDTH * CELL_SIZE as usize) as i32,
        window_height: (grid::GRID_HEIGHT * CELL_SIZE as usize) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut grid = Grid::new(grid::GRID_WIDTH, grid::GRID_HEIGHT);
    let mut selected_element = Element::Sand;
    let mut brush_size = 1;

    // main game loop
    loop {
        grid.update();

        clear_background(BACKGROUND_COLOR);
        if !handle_input(&mut grid, &mut selected_element, &mut brush_size) {
            break;
        }

        // Draw text
        let framerate: String = String::from("fps: ") + &get_fps().to_string();
        draw_text(&framerate, 10.0, 20.0, 30.0, WHITE);
        let selected_element_text =
            String::from("Selected element: ") + selected_element.to_string();
        draw_text(&selected_element_text, 10.0, 50.0, 30.0, WHITE);
        let brush_size_text = String::from("Brush size: ") + &brush_size.to_string();
        draw_text(&brush_size_text, 10.0, 80.0, 30.0, WHITE);

        for y in 0..grid::GRID_HEIGHT {
            for x in 0..grid::GRID_WIDTH {
                let cell = grid.get((x, y));
                let color = match cell.get_color() {
                    Some(color) => color,
                    None => continue,
                };
                // Draw the grid
                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }

        // Draw a box of size brush_size around the mouse
        let brush_offset = (brush_size - 1) / 2;
        let x_brush_box = (mouse_position().0 / CELL_SIZE) as isize - brush_offset as isize;
        let y_brush_box = (mouse_position().1 / CELL_SIZE) as isize - brush_offset as isize;
        draw_rectangle_lines(
            x_brush_box as f32 * CELL_SIZE,
            y_brush_box as f32 * CELL_SIZE,
            brush_size as f32 * CELL_SIZE,
            brush_size as f32 * CELL_SIZE,
            2.0,
            RED,
        );

        next_frame().await
    }
}

fn handle_input(grid: &mut Grid, selected_element: &mut Element, brush_size: &mut usize) -> bool {
    if is_key_pressed(KeyCode::R) {
        grid.reset();
    }
    if is_key_pressed(KeyCode::Z) {
        *selected_element = Element::Water;
    }
    if is_key_pressed(KeyCode::X) {
        *selected_element = Element::Sand;
    }
    if is_key_pressed(KeyCode::C) {
        *selected_element = Element::Stone;
    }
    if is_key_pressed(KeyCode::V) {
        *selected_element = Element::Air;
    }
    if is_key_pressed(KeyCode::B) {
        *selected_element = Element::Faucet;
    }
    if is_key_pressed(KeyCode::L) {
        *selected_element = Element::Clay;
    }

    if is_mouse_button_down(MouseButton::Left) {
        place_element(grid, selected_element, brush_size);
    }
    if is_mouse_button_down(MouseButton::Right) {
        place_element(grid, &mut Element::Air, brush_size);
    }
    if is_key_pressed(KeyCode::LeftBracket) {
        *brush_size = (*brush_size).saturating_sub(1);
    }
    if is_key_pressed(KeyCode::RightBracket) {
        *brush_size = (*brush_size).saturating_add(1);
    }

    if is_key_pressed(KeyCode::Escape) {
        return false;
    }

    true
}

fn place_element(grid: &mut Grid, selected_element: &mut Element, brush_size: &mut usize) {
    let brush_offset = (*brush_size - 1) / 2;
    for i in 0..*brush_size {
        for j in 0..*brush_size {
            let x = (mouse_position().0 / CELL_SIZE) as isize + i as isize - brush_offset as isize;
            let y = (mouse_position().1 / CELL_SIZE) as isize + j as isize - brush_offset as isize;
            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;
                grid.set((x, y), *selected_element);
            }
        }
    }
}
