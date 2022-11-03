//! # Shape
//!
//! `shape` is a collection of utilities to work with shapes (square and rectangle).
//! 
pub mod square;
pub mod rectangle;

pub trait Shape {
    fn shape(&self) -> String;
}

impl Shape for square::Square {
    fn shape(&self) -> String {
        format!("Square side:{}.", &self.side)
    }
}

impl Shape for rectangle::Rectangle {
    fn shape(&self) -> String {
        format!("Rectangle width:{} height:{}", &self.width, &self.height)
    }
}