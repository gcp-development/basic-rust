//! # Rectangle
//!
//! `rectangle` is a collection of utilities to calculate the perimeter and area of a rectangle. 
//! 
#![warn(missing_docs)]

pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    /// Perimeter of a rectangle function
    pub fn perimeter_of_a_rectangle(rectangle: &Rectangle) -> i32 {
        2 * (rectangle.width * rectangle.height)
    }
    /// Area of a rectangle function
    pub fn area_of_a_rectangle(rectangle: &Rectangle) -> i32 {
        rectangle.width * rectangle.height
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_perimeter_of_a_rectangle() {
        const RESULT: i32 = 30;
        let arg =  super::Rectangle {
            width: 5,
            height: 3,
        };
        assert_eq!(super::Rectangle::perimeter_of_a_rectangle(&arg), RESULT);
    }
    #[test]
    fn test_area_of_a_rectangle() {
        const RESULT: i32 = 15;
        let arg =  super::Rectangle {
            width: 5,
            height: 3,
        };
        assert_eq!(super::Rectangle::area_of_a_rectangle(&arg), RESULT);
    }
}