use macroquad::prelude::*;

use crate::element_type::{
    step_fire, step_gas, step_immoveable_solid, step_liquid, step_maze, step_moveable_solid,
    step_pixel_destroyer, step_pixel_generator, ElementType,
};
use crate::grid::Grid;

#[derive(Clone, Copy, PartialEq)]

pub struct Element {
    pub element_type: ElementType,
    pub color: Option<Color>,
    pub color_variance: f32,
    pub name: &'static str,
}

impl Element {
    pub fn step(&self, grid: &mut Grid, x: usize, y: usize) {
        if !grid.is_within_bounds((x, y)) {
            return;
        }
        match self.element_type {
            ElementType::ImmovableSolid => step_immoveable_solid(grid, x, y),
            ElementType::MoveableSolid => step_moveable_solid(grid, x, y),
            ElementType::Liquid => step_liquid(grid, x, y, 4),
            ElementType::Gas => step_gas(grid, x, y, 1),
            ElementType::PixelGenerator => step_pixel_generator(grid, x, y),
            ElementType::PixelDestroyer => step_pixel_destroyer(grid, x, y),
            ElementType::Maze => step_maze(grid, x, y),
            ElementType::Fire => step_fire(grid, x, y),
            _ => {}
        }
    }
    pub fn to_string(&self) -> &str {
        return self.name;
    }
    pub fn get_color(&self) -> Option<Color> {
        return self.color;
    }

    pub fn get_element_type(&self) -> ElementType {
        return self.element_type;
    }
}

pub static AIR: Element = Element {
    element_type: ElementType::Gas,
    color: Some(SKYBLUE),
    color_variance: 0.05,
    name: "Air",
};

pub static SAND: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Some(GOLD),
    color_variance: 0.07,
    name: "Sand",
};

pub static WATER: Element = Element {
    element_type: ElementType::Liquid,
    color: Some(BLUE),
    color_variance: 0.15,
    name: "Water",
};

pub static STONE: Element = Element {
    element_type: ElementType::ImmovableSolid,
    color: Some(DARKGRAY),
    color_variance: 0.0,
    name: "Stone",
};

pub static FAUCET: Element = Element {
    element_type: ElementType::PixelGenerator,
    color: Some(WHITE),
    color_variance: 0.0,
    name: "Faucet",
};

pub static CLAY: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Some(BROWN),
    color_variance: 0.0,
    name: "Clay",
};

pub static NOTHING: Element = Element {
    element_type: ElementType::Nothing,
    color: None,
    color_variance: 0.0,
    name: "Nothing",
};

pub static MAZE: Element = Element {
    element_type: ElementType::Maze,
    color: Some(WHITE),
    color_variance: 0.0,
    name: "Maze",
};

pub static FIRE: Element = Element {
    element_type: ElementType::Fire,
    color: Some(RED),
    color_variance: 0.0,
    name: "Fire",
};

pub static DRAIN: Element = Element {
    element_type: ElementType::PixelDestroyer,
    color: Some(DARKGRAY),
    color_variance: 0.0,
    name: "Drain",
};
