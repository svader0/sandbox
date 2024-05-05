use macroquad::prelude::*;

pub mod element_type;
pub mod elements;
pub mod grid;
use ::rand::{thread_rng, Rng};
use elements::{Element, AIR, CLAY, FAUCET, NOTHING, SAND, STONE, WATER};
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
const BACKGROUND_COLOR: Color = BLACK;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sandbox".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut grid = Grid::new(screen_height());
    let mut selected_element = SAND;
    let mut brush_size = 1;

    let mut control_manager = ControlManager::new();

    control_manager.add_control(
        KeyCode::Z,
        Box::new(|elem| *elem = WATER),
        String::from("Z: water"),
    );
    control_manager.add_control(
        KeyCode::X,
        Box::new(|elem| *elem = SAND),
        String::from("X: sand"),
    );
    control_manager.add_control(
        KeyCode::C,
        Box::new(|elem| *elem = STONE),
        String::from("C: stone"),
    );
    control_manager.add_control(
        KeyCode::V,
        Box::new(|elem| *elem = AIR),
        String::from("V: air"),
    );
    control_manager.add_control(
        KeyCode::B,
        Box::new(|elem| *elem = FAUCET),
        String::from("B: faucet"),
    );
    control_manager.add_control(
        KeyCode::L,
        Box::new(|elem| *elem = CLAY),
        String::from("L: clay"),
    );

    // Define brush size controls
    control_manager.add_brush_control(
        KeyCode::LeftBracket,
        |size| size - 1,
        String::from("[: brush-1"),
    );
    control_manager.add_brush_control(
        KeyCode::RightBracket,
        |size| size + 1,
        String::from("]: brush+1"),
    );

    // main game loop
    loop {
        grid.update();
        if grid.width != screen_width().round() as usize
            || screen_height().round() as usize != grid.height
        {
            //change in window
            grid.update_cell_size(screen_height());
        }

        let mut rng = thread_rng();

        //inputs
        if is_key_pressed(KeyCode::R) {
            //put here so it has access to grid. Temp?
            grid.reset();
        }
        clear_background(BACKGROUND_COLOR);
        if !control_manager.handle_input(&mut selected_element, &mut brush_size) {
            break; // Escape was pressed
        }
        handle_mouse_input(&mut grid, &selected_element, &brush_size);
        // Draw text
        let top_of_text = (screen_height() * 0.9).round();
        let framerate: String = String::from("fps: ") + &get_fps().to_string();
        draw_text(&framerate, 10.0, top_of_text + 20.0, 30.0, WHITE);
        let selected_element_text =
            String::from("Selected element: ") + selected_element.to_string();
        draw_text(
            &selected_element_text,
            10.0,
            top_of_text + 50.0,
            30.0,
            WHITE,
        );
        let brush_size_text = String::from("Brush size: ") + &brush_size.to_string();
        draw_text(&brush_size_text, 10.0, top_of_text + 80.0, 30.0, WHITE);
        draw_text(
            &control_manager.controls_string(),
            10.0,
            (screen_height() * 0.9).round(),
            30.0,
            WHITE,
        );

        //render grid
        for y in 0..screen_height() as usize - 20 {
            for x in 0..screen_width() as usize - 20 {
                let cell = grid.get((x, y));
                let color = match cell.get_color() {
                    Some(color) => {
                        let variance = cell.color_variance;
                        // Add some variance to the color of each cell, per frame.
                        // Creates a sort of "shimmering" effect.
                        let r = color.r * (1.0 - variance) + (variance * rng.gen_range(0.0..1.0));
                        let g = color.g * (1.0 - variance) + (variance * rng.gen_range(0.0..1.0));
                        let b = color.b * (1.0 - variance) + (variance * rng.gen_range(0.0..1.0));
                        Color::new(r, g, b, color.a)
                    }
                    None => continue,
                };
                // Draw the grid
                draw_rectangle(
                    x as f32 * grid.cell_size,
                    y as f32 * grid.cell_size,
                    grid.cell_size,
                    grid.cell_size,
                    color,
                );
            }
        }

        // Draw a box of size brush_size around the mouse
        let brush_offset = (brush_size - 1) / 2;
        let x_brush_box = (mouse_position().0 / grid.cell_size) as isize - brush_offset as isize;
        let y_brush_box = (mouse_position().1 / grid.cell_size) as isize - brush_offset as isize;
        draw_rectangle_lines(
            x_brush_box as f32 * grid.cell_size,
            y_brush_box as f32 * grid.cell_size,
            brush_size as f32 * grid.cell_size,
            brush_size as f32 * grid.cell_size,
            2.0,
            RED,
        );

        next_frame().await
    }
}

fn place_element(grid: &mut Grid, selected_element: &Element, brush_size: &mut usize) {
    let brush_offset = (*brush_size - 1) / 2;
    for i in 0..*brush_size {
        for j in 0..*brush_size {
            let x =
                (mouse_position().0 / grid.cell_size) as isize + i as isize - brush_offset as isize;
            let y =
                (mouse_position().1 / grid.cell_size) as isize + j as isize - brush_offset as isize;
            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;
                grid.set((x, y), *selected_element);
            }
        }
    }
}

struct Control {
    key: KeyCode,
    action: Box<dyn Fn(&mut Element)>,
    description: String,
}

struct BrushControl {
    key: KeyCode,
    action: fn(usize) -> usize,
    description: String,
}

struct ControlManager {
    controls: Vec<Control>,
    brush_size_controls: Vec<BrushControl>,
}

impl ControlManager {
    fn new() -> Self {
        ControlManager {
            controls: vec![],
            brush_size_controls: vec![],
        }
    }

    fn add_control(
        &mut self,
        key: KeyCode,
        action: Box<dyn Fn(&mut Element)>,
        description: String,
    ) {
        self.controls.push(Control {
            key,
            action,
            description,
        });
    }

    fn add_brush_control(&mut self, key: KeyCode, action: fn(usize) -> usize, description: String) {
        self.brush_size_controls.push(BrushControl {
            key,
            action,
            description,
        });
    }

    fn handle_input(&self, selected_element: &mut Element, brush_size: &mut usize) -> bool {
        for control in &self.controls {
            if is_key_pressed(control.key) {
                (control.action)(selected_element);
            }
        }

        for b_control in &self.brush_size_controls {
            if is_key_pressed(b_control.key) {
                *brush_size = (b_control.action)(*brush_size);
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            return false;
        }

        true
    }

    fn controls_string(&self) -> String {
        let mut result = String::from("Controls:\n");
        for control in &self.controls {
            result += &control.description.to_string();
            result += "\n"
        }
        for bc in &self.brush_size_controls {
            result += &bc.description;
        }

        //manually added controls
        result += "\nesc: quit, r: reset";
        result
    }
}

fn handle_mouse_input(grid: &mut Grid, selected_element: &Element, brush_size: &usize) {
    if is_mouse_button_down(MouseButton::Left) {
        place_element(grid, &mut selected_element.clone(), &mut brush_size.clone());
    }
    if is_mouse_button_down(MouseButton::Right) {
        place_element(grid, &NOTHING, &mut brush_size.clone());
    }
}
