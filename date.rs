#![crate_name = "date"]
#![crate_type = "lib"]
#![license = "MIT"]
#![desc = "Library for simple date management"]
#![comment = "Example of library: date management"]
//! Date management
//!
//! Use "constructor", string manipulation
//! Tested with rust-0.10-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2013-10-24
//!
//! @todo : get_day_of_week(), get_week(), comparisons
extern crate debug;
use std::from_str::FromStr;
use std::fmt;

/**
 * Simple struct to handle date.
 */
pub struct Date {
	day: int,
	month: int,
	year: int
}

impl Date {
	/**
	 * Add days to the current day. Use negative to remove day.
	 */
	pub fn add_days(&mut self, days: int) {
		let mut day = self.day;
		let mut month = self.month;
		let mut year = self.year;
		day = day + days;
		if days > 0 {
			while day > Date::month_length(year, month) {
				day = day - Date::month_length(year, month);
				month = month + 1;
				if month > 12 {
					year = year + 1;
					month = 1;
				}
			}
		}
		if day == 0 {
			month = month - 1;
			if month < 1 {
				month = 12;
				year = year - 1;
			}
			day = Date::month_length(year, month);
		}
		if days < 0 {
			while day < 1 {
				month = month - 1;
				if month < 1 {
					year = year - 1;
					month = 12;
				}
				day = day + Date::month_length(year, month);
			}
		}
		self.day = day;
		self.month = month;
		self.year = year;
	}

	/**
	 * Get day of year.
	 */
	pub fn get_day_of_year(&self) -> int {
		let mut doy = self.day;
		for month in range(1, self.month) {
			doy += Date::month_length(self.year, month);
		}
		doy
	}

	/**
	 * Check if defined date is valid.
	 */
	pub fn is_valid(&self) -> bool {
		if self.month < 1 || self.month > 12 {
			false
		} else if self.day < 1 || self.day > Date::month_length(self.year, self.month) {
			false
		} else {
			true
		}
	}

	/*
	 * Static methods
	 */

	/**
	 * Static method to know if the year is a leap year.
	 */
	pub fn is_leap(year: int) -> bool {
		(year % 4 == 0 && year % 100 != 0) || year % 400 == 0
	}

	/**
	 * Static method to get the number of days in the month.
	 */
	pub fn month_length(year: int, month: int) -> int {
		match  month {
			1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
			2 => if Date::is_leap(year) { 29 } else { 28 },
			4 | 6 | 9 | 11 => 30,
			_ => fail!("Wrong month")
		}
	}

	/**
	 * "Constructor".
	 */
	pub fn new(year: int, month: int, day: int) -> Date {
		Date{day: day, month: month, year: year}
	}

	/**
	 * "Constructor" using string like "2013-10-24".
	 */
	pub fn new_from_string(string: &str) -> Date {
		if string.len() < 10 {
			fail!("Wrong format!");
		}
		let year = FromStr::from_str(string.slice_chars(0, 4)).unwrap();
		let month = FromStr::from_str(string.slice_chars(5, 7)).unwrap();
		let day = FromStr::from_str(string.slice_chars(8, 10)).unwrap();
		Date{day: day, month: month, year: year}
	}
}

/**
 * Operator overloading
 *
 * @see http://maniagnosis.crsr.net/2013/04/operator-overloading-in-rust.html
 */
impl fmt::Show for Date {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:4d}-{:2d}-{:2d}", self.year, self.month, self.day)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn add_days() {
		let mut date = ::Date::new(2013, 10, 24);
		let orig = date.to_string();
		let days = 2;
		date.add_days(days);
		let expected = "2013-10-26".to_string();
		let found = date.to_string();
		assert!(expected==found, format!("Adding {:d} days to {} should return {}, not {}", days, orig, expected, found));
	}
	#[test]
	fn get_day_of_year() {
		let date = ::Date::new(2014, 01, 01);
		let expected = 1;
		let found = date.get_day_of_year();
		assert!(expected==found, format!("{:?} must be day number {} of the year, not {}.", date, expected, found));
		let date = ::Date::new(2012, 12, 31);
		let expected = 366;
		let found = date.get_day_of_year();
		assert!(expected==found, format!("{:?} must be day number {} of the year, not {}.", date, expected, found));
	}
	#[test]
	fn is_leap() {
		assert!(!::Date::is_leap(1900), "1900 is not a leap year");
		assert!(!::Date::is_leap(1901), "1901 is not a leap year");
		assert!(::Date::is_leap(2000), "2000 is leap year");
		assert!(::Date::is_leap(2004), "2004 is leap year");
	}
	#[test]
	fn is_valid() {
		let mut date = ::Date::new(2013, 10, 24);
		assert!(date.is_valid(), "2013-10-24 is a valid date");
		date = ::Date::new(2013, 02, 29);
		assert!(!date.is_valid(), "2013-02-29 isn't a valid date");
		date = ::Date::new(2012, 02, 29);
		assert!(date.is_valid(), "2012-02-29 isn't a valid date");
	}
	#[test]
	fn to_string() {
		let date = ::Date::new(2013, 10, 24);
		let expected = "2013-10-24";
		let date_str = date.to_string();
		let found = date_str.as_slice();
		assert!(expected == found, format!("{}!={}", expected, found));
	}
	/*
	 * Static methods
	 */
	#[test]
	fn month_length() {
		assert!(::Date::month_length(2000, 2) == 29, "February 2000 has 29 days");
		assert!(::Date::month_length(2001, 2) == 28, "February 2001 has 28 days");
		assert!(::Date::month_length(2013, 2) == 28, "February 2013 has 28 days");
		assert!(::Date::month_length(2013, 9) == 30, "September 2013 has 30 days");
		assert!(::Date::month_length(2013, 10) == 31, "October 2013 has 31 days");
	}
	#[test]
	fn new() {
		let date = ::Date::new(2013, 10, 24);
		let expected = "2013-10-24".to_string();
		let found = date.to_string();
		assert!(expected == found, format!("{}!={}", expected, found));
	}
	#[test]
	fn new_from_string() {
		let date = ::Date::new_from_string("2013-10-24 23:24:34");
		let expected = "2013-10-24".to_string();
		let found = date.to_string();
		assert!(expected == found, format!("{}!={}", expected, found));
	}
}

