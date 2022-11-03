//! # Currency exchange
//!
//! `currencyexh` is a collection of utilities to calculate exchange currencies. 
//! 
#![warn(missing_docs)]

pub struct Dollars {
	pub amount: f64,
}

pub struct Euros {
	pub amount: f64,
}

pub struct Pounds {
	pub amount: f64,
}

pub trait Currency {
	const EXCHANGE_RATE_TO_CURRENCY1: f64;
	const EXCHANGE_RATE_TO_CURRENCY2: f64;
	fn to_euros(&self) -> f64;
    fn to_dollars(&self) -> f64;
	fn to_pounds(&self) -> f64;
}

impl Currency for Dollars {
	const EXCHANGE_RATE_TO_CURRENCY1: f64 = 1.0140049; //USD to EUR
	const EXCHANGE_RATE_TO_CURRENCY2: f64 = 0.87320172; //USD to GBP
	/// Dollars exchange to Euros function
	fn to_euros(&self) -> f64 {
		self.amount * Self::EXCHANGE_RATE_TO_CURRENCY1
	}
    /// Dollars exchange to Dollars function
	fn to_dollars(&self) -> f64 {
		self.amount * 1.0
	}
    /// Dollars exchange to Pounds function
	fn to_pounds(&self) -> f64 {
		self.amount * Self::EXCHANGE_RATE_TO_CURRENCY2
	}
}

impl Currency for Euros {
	const EXCHANGE_RATE_TO_CURRENCY1: f64 = 0.98610902;//EUR to USD
	const EXCHANGE_RATE_TO_CURRENCY2: f64 = 0.86077111;//EUR to GBP
	/// Euros exchange to Euros function
	fn to_euros(&self) -> f64 {
		self.amount * 1.0
	}
    /// Euros exchange to Dollars function
	fn to_dollars(&self) -> f64 {
		self.amount * Self::EXCHANGE_RATE_TO_CURRENCY1
	}
    /// Euros exchange to Pounds function
	fn to_pounds(&self) -> f64 {
		self.amount * Self::EXCHANGE_RATE_TO_CURRENCY2
	}
}

impl Currency for Pounds {
	const EXCHANGE_RATE_TO_CURRENCY1: f64 = 1.1616931;//GBP to EUR
	const EXCHANGE_RATE_TO_CURRENCY2: f64 = 1.1456422;//GBP to USD
	/// Pounds exchange to Euros function
	fn to_euros(&self) -> f64 {
		self.amount * Self::EXCHANGE_RATE_TO_CURRENCY1
	}
    /// Pounds exchange to Dollars function
	fn to_dollars(&self) -> f64 {
		self.amount * Self::EXCHANGE_RATE_TO_CURRENCY2
	}
    /// Pounds exchange to Pounds function
	fn to_pounds(&self) -> f64 {
		self.amount * 1.0
	}
}

/// Exchange function to Euros 
pub fn exchange_to_euros<C: Currency>(money: &C) -> f64 {
	money.to_euros()
}
/// Exchange function to Dollars
pub fn exchange_to_dollars<C: Currency>(money: &C) -> f64 {
	money.to_dollars()
}
/// Exchange function to Pounds
pub fn exchange_to_pounds<C: Currency>(money: &C) -> f64 {
	money.to_pounds()
}