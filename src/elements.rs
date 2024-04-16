use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

use crate::grid::{self, Grid};
use crate::element_type::{ElementType, step_immoveable_solid, step_moveable_solid, step_liquid, step_pixel_generator};


#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Air,
    Sand,
    Water,
    Stone,
    Faucet,
    Nothing,
    Clay,
}

impl Element {
    pub fn to_string(&self) -> &str {
        match self {
            Element::Nothing => "Nothing",
            Element::Sand => "Sand",
            Element::Water => "Water",
            Element::Stone => "Stone",
            Element::Faucet => "Faucet",
            Element::Air => "Air",
            Element::Clay => "Clay"
        }
    }
    pub fn get_color(&self) -> Option<Color> {
        match self {
            Element::Air => Some(SKYBLUE),
            Element::Sand => Some(GOLD),
            Element::Water => Some(BLUE),
            Element::Stone => Some(DARKGRAY),
            Element::Faucet => Some(WHITE),
            Element::Nothing => None,
            Element::Clay => Some(BROWN),
        }
    }

    pub fn get_element_type(&self) -> ElementType {
        match self {
            Element::Air => ElementType::Gas,
            Element::Sand => ElementType::MoveableSolid,
            Element::Water => ElementType::Liquid,
            Element::Stone => ElementType::ImmovableSolid,
            Element::Faucet => ElementType::PixelGenerator,
            Element::Nothing => ElementType::Nothing,
            Element::Clay => ElementType::MoveableSolid,

        }
    }
    pub fn step(&self, grid: &mut Grid, x: usize, y: usize) {
        if !grid.is_within_bounds((x, y)) {
            return;
        }
        
        let element_type = self.get_element_type();
        match element_type {
            ElementType::ImmovableSolid => step_immoveable_solid(&self, grid, x, y),
            ElementType::MoveableSolid=> step_moveable_solid(&self, grid, x, y),
            ElementType::Liquid => step_liquid(&self, grid, x, y, 1),
            ElementType::Gas => step_liquid(&self, grid, x, y, 1),
            ElementType::PixelGenerator => step_pixel_generator(&self, grid, x, y),
            _ => {}
        }
    }
}
