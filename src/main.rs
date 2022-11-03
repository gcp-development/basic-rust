//! # Main
//!
//! `main` this is a rust demo. 
//! 
#[allow(dead_code)]
mod shape;
#[allow(dead_code)]
mod currencyexh;

fn main() {
    let sq =  shape::square::Square {
        side: 5,
    };
    
    println!(
        "The square perimeter is {}.", shape::square::Square::perimeter_of_a_square(&sq)
    );

    let rec =  shape::rectangle::Rectangle {
        width: 5,
        height: 3,
    };
    println!(
        "The rectangle perimeter is {}.", shape::rectangle::Rectangle::perimeter_of_a_rectangle(&rec)
    );

    let shapes: Vec<&dyn shape::Shape> = vec![&sq,&rec];
    
    for element in shapes.iter() {
        println!("{}",element.shape());
    }

    let dollars= currencyexh::Dollars {
	    amount: 100.0,
    };

    let euros= currencyexh::Euros {
	    amount: 100.0,
    };

    let pounds= currencyexh::Pounds {
	    amount: 100.0,
    };

    println!(
        "Dollars:${} to Euros:€{}.",&dollars.amount, currencyexh::exchange_to_euros(&dollars)
    );
    println!(
        "Pounds:${} to Euros:€{}.",&pounds.amount, currencyexh::exchange_to_euros(&pounds)
    );
    println!(
        "Euros:€{} to Dollars:${}.",&euros.amount, currencyexh::exchange_to_dollars(&euros)
    );
    println!(
        "Pounds:£{} to Dollars:${}.",&pounds.amount, currencyexh::exchange_to_dollars(&pounds)
    );
    println!(
        "Dollars:${} to Pounds:£{}.",&dollars.amount, currencyexh::exchange_to_pounds(&dollars)
    );
    println!(
        "Euros:${} to Pounds:£{}.",&euros.amount, currencyexh::exchange_to_pounds(&euros)
    );
    
}


