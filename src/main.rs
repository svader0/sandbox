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
const BACKGROUND_COLOR: Color = BLACK;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sandbox".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut grid = Grid::new(226,126, ((screen_height() * 0.01) / 126.0));
    let mut selected_element = Element::Sand;
    let mut brush_size = 1;


    let mut control_manager = ControlManager::new();

    control_manager.add_control(
        KeyCode::Z, 
        Box::new(|elem| *elem = Element::Water),
        String::from("Z: water"));
    control_manager.add_control(
        KeyCode::X,
        Box::new(|elem| *elem = Element::Sand),
        String::from("X: sand"));
    control_manager.add_control(
        KeyCode::C, 
        Box::new(|elem| *elem = Element::Stone),
        String::from("C: stone"));
    control_manager.add_control(
        KeyCode::V, 
        Box::new(|elem| *elem = Element::Air),
        String::from("V: air"));    
    control_manager.add_control(
        KeyCode::B, 
        Box::new(|elem| *elem = Element::Faucet),
        String::from("B: faucet")); 
    control_manager.add_control(
        KeyCode::L, 
        Box::new(|elem| *elem = Element::Clay),
        String::from("L: clay")); 

    // Define brush size controls
    control_manager.add_brush_control(KeyCode::LeftBracket, |size| size.saturating_sub(1), String::from("[: brush-1"));
    control_manager.add_brush_control(KeyCode::RightBracket, |size| size.saturating_add(1), String::from("]: brush+1"));
    


    // main game loop
    loop {
        grid.update();


        if (grid.width != screen_width().round() as usize || screen_height().round() as usize != grid.height){ //change in window
            grid.cell_size = (screen_height() * 0.8) as f32 / 126.0;
        }

        if is_key_pressed(KeyCode::R) { //put here so it has access to grid. Temp?
            grid.reset();
        }

        clear_background(BACKGROUND_COLOR);
        if !control_manager.handle_input(&mut selected_element,&mut brush_size) {
            break; // Escape was pressed
        }
        handle_mouse_input(&mut grid, &mut selected_element, &mut brush_size);
        // if !handle_input(&mut grid, &mut selected_element, &mut brush_size) {
        //     break;
        // }

        // Draw text
        let top_of_text = (screen_height()*0.9).round();
        let framerate: String = String::from("fps: ") + &get_fps().to_string();
        draw_text(&framerate, 10.0, top_of_text + 20.0, 30.0, WHITE);
        let selected_element_text =
            String::from("Selected element: ") + selected_element.to_string();
        draw_text(&selected_element_text, 10.0, top_of_text + 50.0, 30.0, WHITE);
        let brush_size_text = String::from("Brush size: ") + &brush_size.to_string();
        draw_text(&brush_size_text, 10.0, top_of_text + 80.0, 30.0, WHITE);
        draw_text(&control_manager.controls_string(), 10.0, (screen_height()*0.9).round(), 30.0, WHITE);

        for y in 0..screen_height()as usize-20 {
            for x in 0..screen_width()as usize-20 {
                let cell = grid.get((x, y));
                let color = match cell.get_color() {
                    Some(color) => color,
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



fn place_element(grid: &mut Grid, selected_element: &mut Element, brush_size: &mut usize) {
    let brush_offset = (*brush_size - 1) / 2;
    for i in 0..*brush_size {
        for j in 0..*brush_size {
            let x = (mouse_position().0 / grid.cell_size) as isize + i as isize - brush_offset as isize;
            let y = (mouse_position().1 / grid.cell_size) as isize + j as isize - brush_offset as isize;
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

struct ControlManager {
    controls: Vec<Control>,
    brush_size_controls: Vec<(KeyCode, fn(usize) -> usize, String)>,
}

impl ControlManager {
    fn new() -> Self {
        ControlManager {
            controls: vec![],
            brush_size_controls: vec![],
        }
    }

    fn add_control(&mut self, key: KeyCode, action: Box<dyn Fn(&mut Element)>, description: String) {
        self.controls.push(Control { key, action, description });
    }

    fn add_brush_control(&mut self, key: KeyCode, func: fn(usize) -> usize, description: String) {
        self.brush_size_controls.push((key, func, description));
    }

    fn handle_input(&self, selected_element: &mut Element, brush_size: &mut usize) -> bool {
        for control in &self.controls {
            if is_key_pressed(control.key) {
                (control.action)(selected_element);
            }
        }

        for &(key, func, ref _description) in &self.brush_size_controls {
            if is_key_pressed(key) {
                *brush_size = func(*brush_size);
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
        // for control in &self.controls {
        //     result += &format!("Key: {:?}, Action: {}\n", control.key, control.description);
        // }
        for &(key, _, ref desc) in &self.brush_size_controls {
            result += desc;
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
        place_element(grid, &mut Element::Air, &mut brush_size.clone());
    }
}