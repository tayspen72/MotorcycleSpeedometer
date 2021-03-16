//==============================================================================
// Notes
//==============================================================================
// drivers::lcd.rs

//==============================================================================
// Crates and Mods
//==============================================================================
// use crate::config;
// use crate::mcu::spi;
use super::{images, lcd, st7789};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
//TODO: Fix LCD colors
#[allow(dead_code)]
pub enum Color { // 5-6-5		R,  G,  B
	Black		= 0x0000,	//  0,  0,  0
	Red			= 0xF800,	// 1F, 00, 00
	Orange		= 0xFFE0,	// 1F, 1F, 00
	Yellow		= 0x07FF,	// 1F, 3F, 00
	Green		= 0x07E0,	// 00, 3F, 00
	Blue		= 0x001F,	// 00, 00, 1F
	Purple		= 0xF81F,	// 1F, 00, 1F
	White		= 0xFFFF,	// 1F, 3F, 1F

	GrayDark	= 0x0001,	// 08, 10, 08
	Gray		= 0x0002,	// 0F, 1F, 0F
	GrayLight	= 0x0003,	// 18, 30, 18

	YellowGreen	= 0x0008,	// 0F, 3F, 00
	TealGreen	= 0x000a,	// 00, 3F, 0F
	Teal		= 0x000b,	// 00, 3F, 1F
	TealBlue	= 0x000c,	// 00, 1F, 1F
	Navy		= 0x000e,	// 00, 00, 0F
	Magenta		= 0x000f,	// 0F, 00, 1F
	Pink		= 0x0020,	// 1F, 00, 0F
}

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Private Functions
//==============================================================================
pub fn init() {
	lcd::init();

	// // TODO: Fix the colors...
	// fill_background(p, Color::Red as u16);
	// fill_background(p, Color::Orange as u16);
	// fill_background(p, Color::Yellow as u16);
	// fill_background(p, Color::Green as u16);
	// fill_background(p, Color::Blue as u16);
	// fill_background(p, Color::Purple as u16);
	fill_background(Color::Black as u16);
	lcd::set_backlight(lcd::BacklightBrightness::Brightness7);
	write_image();
}

pub fn get_busy() -> bool {
	// For now, not using DMA, this library will never be busy
	false
}

pub fn fill_background(color: u16) {
	set_window(0, 240, 0, 240);
	lcd::write_command(st7789::COMMAND::MEMORY_WRITE);
	for _ in 0..57600 {
		lcd::write_data(&[ ((color & 0xFF00) >> 8)as u8, (color & 0xFF) as u8 ]);
	}
}

pub fn fill_rectangle(x: u16, width: u16, y: u16, height: u16, color: u16) {
	set_window(x, width, y, height);
	lcd::write_command(st7789::COMMAND::MEMORY_WRITE);
	for _ in 0..(width * height) {
		lcd::write_data(&[ ((color & 0xFF00) >> 8)as u8, (color & 0xFF) as u8 ]);
	}
}

pub fn set_window(x: u16, width: u16, y: u16, height: u16) {
	let x_end = x + width - 1;
	let y_end = y + height - 1;

	// Define the window column size
	lcd::write_command(st7789::COMMAND::COLUMN_ADDRESS);
	lcd::write_data( &[ 
		((x & 0xFF00) >> 8) as u8, (x & 0x00FF) as u8,
		((x_end & 0xFF00) >> 8) as u8, (x_end & 0x00FF) as u8,
	]);

	// Define the window row size
	lcd::write_command(st7789::COMMAND::ROW_ADDRESS);
	lcd::write_data( &[ 
		((y & 0xFF00) >> 8) as u8, (y & 0x00FF) as u8,
		((y_end & 0xFF00) >> 8) as u8, (y_end & 0x00FF) as u8,
	]);
}

fn write_image() {
	set_window(0, 80, 0, 53);
	lcd::write_command(st7789::COMMAND::MEMORY_WRITE);
	lcd::write_data(&images::RUSTACEAN);
}

//==============================================================================
// Public Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================

