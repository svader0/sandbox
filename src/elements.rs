use macroquad::prelude::*;

use crate::grid::Grid;
use crate::element_type::{ElementType, step_immoveable_solid, step_moveable_solid, step_liquid, step_pixel_generator, step_gas};


#[derive(Clone, Copy, PartialEq)]

pub struct Element {
    pub element_type: ElementType,
    pub color: Option<Color>,
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
    name: "Air",
};

pub static SAND: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Some(GOLD),
    name: "Sand",
};

pub static WATER: Element = Element {
    element_type: ElementType::Liquid,
    color: Some(BLUE),
    name: "Water",
};

pub static STONE: Element = Element {
    element_type: ElementType::ImmovableSolid,
    color: Some(DARKGRAY),
    name: "Stone",
};

pub static FAUCET: Element = Element {
    element_type: ElementType::PixelGenerator,
    color: Some(WHITE),
    name: "Faucet",
};

pub static CLAY: Element = Element {
    element_type: ElementType::MoveableSolid,
    color: Some(BROWN),
    name: "Clay",
};

pub static NOTHING: Element = Element {
    element_type: ElementType::Nothing,
    color: None,
    name: "Nothing",
};
