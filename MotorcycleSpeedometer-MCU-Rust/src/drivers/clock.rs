//==============================================================================
// Notes
//==============================================================================
// drivers::clock.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::Cell;
use cortex_m::interrupt::{free, Mutex};
use crate::drivers::{app, lcd};
use crate::mcu::rtc;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Clone, Copy)]
pub struct Time {
	pub hours: u8,
	pub minutes: u8,
	pub seconds: u8
}

//==============================================================================
// Variables
//==============================================================================
static TIME: Mutex<Cell<Time>> = Mutex::new(Cell::new( Time {
	hours: 22,
	minutes: 54,
	seconds: 0
}));

const DIGITS_X: [u16; 6] = [ 0, 40, 80, 120, 160, 200 ];
const DIGITS_Y: [u16; 6] = [ 60; 6 ];

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	free(|cs| {
		let mut time = TIME.borrow(cs).get();
		time.seconds += 1;

		if time.seconds >= 60 {
			time.seconds = 0;
			time.minutes += 1;
		
			if time.minutes >= 60 {
				time.minutes = 0;
				time.hours += 1;
				
				if time.hours >= 24 {
					time.hours = 0;
				}
			}
		}

		TIME.borrow(cs).set(time);
	});
	write_time();
}

pub fn write_time() {
	static mut LAST_DIGITS: [u8; 6] = [10; 6];	// Init at 10 to force write the first time
	
	let time = free(|cs| TIME.borrow(cs).get());
	let digits: [u8; 6] = [
		(time.hours/10)%10,
		time.hours%10,
		(time.minutes/10)%10,
		time.minutes%10,
		(time.seconds/10)%10,
		time.seconds%10,
	];

	unsafe {
		for i in 0..6 {
			if digits[i] != LAST_DIGITS[i] {
				LAST_DIGITS[i] = digits[i];
				lcd::font::write_time_character(digits[i], DIGITS_X[i], DIGITS_Y[i], lcd::lcd_api::Color::Blue as u16, lcd::lcd_api::Color::White as u16);
			}
		}
	}
}

//==============================================================================
// Private Functions
//==============================================================================
fn update_add_second() {
	free(|cs| {
		let mut time = TIME.borrow(cs).get();
		time.seconds += 1;

		if time.seconds >= 60 {
			time.seconds = 0;
			time.minutes += 1;
		
			if time.minutes >= 60 {
				time.minutes = 0;
				time.hours += 1;
				
				if time.hours >= 24 {
					time.hours = 0;
				}
			}
		}

		TIME.borrow(cs).set(time);
	});
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut app::DeviceInfo) {
	static mut LAST_TIMESTAMP: u32 = 0;

	unsafe {
		if rtc::get_timediff(LAST_TIMESTAMP) >= 1 {
			LAST_TIMESTAMP = rtc::get_timestamp();
			update_add_second();
			d.time = free(|cs| TIME.borrow(cs).get());

			write_time();
		}
	}

}