//! # Square
//!
//! `square` is a collection of utilities to calculate the perimeter and area of a square. 
//! 
#![warn(missing_docs)]

pub struct Square {
    pub side: i32,
}

impl Square {
    /// Perimeter of a square function
    pub fn perimeter_of_a_square(square: &Square) -> i32 {
        4 * square.side
    }
    /// Area of a square function
    pub fn area_of_a_square(square: &Square) -> i32 {
        square.side * square.side
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_perimeter_of_a_square() {
        const RESULT: i32 = 20;
        let arg =  super::Square {
            side: 5,
        };
        assert_eq!(super::Square::perimeter_of_a_square(&arg), RESULT);
    }
    #[test]
    fn test_area_of_a_square() {
        const RESULT: i32 = 25;
        let arg =  super::Square {
            side: 5,
        };
        assert_eq!(super::Square::area_of_a_square(&arg), RESULT);
    }
}