//==============================================================================
// Notes
//==============================================================================
// drivers/quadalpha.rs
// Quad-Alpha-Numeric Backpack from Adafruit
// I2C Interface controlling (4) 14-segment digits

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::{app, config};
use crate::mcu::eusci;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
type QuadAlphaCharacter = (u8, u8);

//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
const CHARACTERS: [QuadAlphaCharacter; 38] = [
	(0x3F, 0x0C),	// 0
	(0x06, 0x04),	// 1
	(0xDB, 0x00),	// 2
	(0x8F, 0x00),	// 3
	(0xE6, 0x00),	// 4
	(0x69, 0x20),	// 5
	(0xFD, 0x00),	// 6
	(0x07, 0x00),	// 7
	(0xFF, 0x00),	// 8
	(0xEF, 0x00),	// 9
	(0xF7, 0x00),	// A
	(0x8F, 0x12),	// B
	(0x39, 0x00),	// C
	(0x0F, 0x12),	// D
	(0x79, 0x00),	// E
	(0x71, 0x00),	// F
	(0xBD, 0x00),	// G
	(0xF6, 0x00),	// H
	(0x09, 0x12),	// I
	(0x1E, 0x00),	// J
	(0x70, 0x24),	// K
	(0x38, 0x00),	// L
	(0x36, 0x05),	// M
	(0x36, 0x21),	// N
	(0x3F, 0x00),	// O
	(0xF3, 0x00),	// P
	(0x3F, 0x20),	// Q
	(0xF3, 0x20),	// R
	(0xED, 0x00),	// S
	(0x01, 0x12),	// T
	(0x3E, 0x00),	// U
	(0x30, 0x0C),	// V
	(0x36, 0x28),	// W
	(0x00, 0x2D),	// X
	(0xEE, 0x00),	// Y
	(0x09, 0x0C),	// Z
	(0xC0, 0x00),	// -
	(0x00, 0x00),	// ' '
];

#[allow(dead_code)]
enum QuadAlphaRegister {	
	DisplayAddress = 0x00,
	SystemSetup = 0x20,
	KeyAddress = 0x40,
	IntAddress = 0x60,
	DisplaySetup = 0x80,
	RowInt = 0xA0,
	Dimming = 0xE0,
}

const I2C: eusci::I2C = eusci::I2C {
	sda_port: config::I2C_SDA_PORT,
	sda_pin: config::I2C_SDA_PIN,
	scl_port: config::I2C_SCL_PORT,
	scl_pin: config::I2C_SCL_PIN,
	eusci: config::I2C_EUSCI,
	function_select: config::I2C_FUNCTION_SELECT,
	address: config::I2C_ADDRESS,
	speed: config::I2C_SPEED,
};

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	eusci::i2c_init(&I2C);
	configure();
	write(&['-', '-', '-', '-'], false);
}

//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn configure() {
	set_system_setup(true, false);
	set_dimming(0x7, false);
	set_display_setup(0x0, true, true);
}

#[allow(dead_code)]
fn get_character(c: char) -> QuadAlphaCharacter {
	match c {
		'0' => CHARACTERS[0],
		'1' => CHARACTERS[1],
		'2' => CHARACTERS[2],
		'3' => CHARACTERS[3],
		'4' => CHARACTERS[4],
		'5' => CHARACTERS[5],
		'6' => CHARACTERS[6],
		'7' => CHARACTERS[7],
		'8' => CHARACTERS[8],
		'9' => CHARACTERS[9],
		'A' => CHARACTERS[10],
		'B' => CHARACTERS[11],
		'C' => CHARACTERS[12],
		'D' => CHARACTERS[13],
		'E' => CHARACTERS[14],
		'F' => CHARACTERS[15],
		'G' => CHARACTERS[16],
		'H' => CHARACTERS[17],
		'I' => CHARACTERS[18],
		'J' => CHARACTERS[19],
		'K' => CHARACTERS[20],
		'L' => CHARACTERS[21],
		'M' => CHARACTERS[22],
		'N' => CHARACTERS[23],
		'O' => CHARACTERS[24],
		'P' => CHARACTERS[25],
		'Q' => CHARACTERS[26],
		'R' => CHARACTERS[27],
		'S' => CHARACTERS[28],
		'T' => CHARACTERS[29],
		'U' => CHARACTERS[30],
		'V' => CHARACTERS[31],
		'W' => CHARACTERS[32],
		'X' => CHARACTERS[33],
		'Y' => CHARACTERS[34],
		'Z' => CHARACTERS[35],
		'-' => CHARACTERS[36],
		' ' => CHARACTERS[37],
		_ => (0, 0)
	}
}

fn get_digits(mut value: u16) -> [char; 4] {
	let mut digits: [char; 4] = [ ' ', ' ', ' ', ' '];

	if value > 10000 {
		value = value % 10000;
	}

	let mut divider = 1000;
	for i in 0..4 {
		if value > divider || i == 3{
			digits[i] = (((value / divider) % 10) as u8 + 0x30) as char;
		}
		divider = divider / 10;
	}
	
	digits
}

#[allow(dead_code)]
fn get_int_address(send_stop: bool) -> u8 {
	let data = QuadAlphaRegister::IntAddress as u8;
	eusci::i2c_write_block(&I2C, &[data], false);
	
	let mut read: [u8; 1] = [0x0];
	eusci::i2c_read_block(&I2C, &mut read, send_stop);
	
	read[0]
}

#[allow(dead_code)]
fn get_key_address(send_stop: bool) -> u8 {
	let data = QuadAlphaRegister::KeyAddress as u8;
	eusci::i2c_write_block(&I2C, &[data], false);
	
	let mut read: [u8; 1] = [0x0];
	eusci::i2c_read_block(&I2C, &mut read, send_stop);
	
	read[0] & 0x07
}

#[allow(dead_code)]
fn set_display_address(address: u8, send_stop: bool) {
	let data = QuadAlphaRegister::DisplayAddress as u8 | (address & 0x0F);
	eusci::i2c_write_block(&I2C, &[data], send_stop);
}

#[allow(dead_code)]
fn set_display_setup(blink: u8, status: bool, send_stop: bool) {
	let mut data = QuadAlphaRegister::DisplaySetup as u8;
	if status {
		data |= 1;
	}
	data |= (blink & 0x3) << 1;
	
	eusci::i2c_write_block(&I2C, &[data], send_stop);
}

#[allow(dead_code)]
fn set_dimming(dimming: u8, send_stop: bool) {
	let data = QuadAlphaRegister::Dimming as u8 | (dimming & 0x0F);
	eusci::i2c_write_block(&I2C, &[data], send_stop);
}

#[allow(dead_code)]
fn set_row_int(row: bool, polarity: bool, send_stop: bool) {
	let mut data = QuadAlphaRegister::RowInt as u8;
	if row {
		data |= 0x2;
	}
	if polarity {
		data |= 0x1;
	}
		
	eusci::i2c_write_block(&I2C, &[data], send_stop);
}

#[allow(dead_code)]
fn set_system_setup(enable: bool, send_stop: bool) {
	let data = if enable {
		QuadAlphaRegister::SystemSetup as u8 | 0x01
	}
	else {
		QuadAlphaRegister::SystemSetup as u8
	};
	
	eusci::i2c_write_block(&I2C, &[data], send_stop);
}

#[allow(dead_code)]
fn write(buf: &[char; 4], decimal: bool){
	let mut data = [
		get_character(buf[0]), 
		get_character(buf[1]), 
		get_character(buf[2]), 
		get_character(buf[3])
	];
	
	if decimal {
		data[2].1 = data[2].1 | 0x40;
	}

	eusci::i2c_write_block(
		&I2C, &[
			QuadAlphaRegister::DisplayAddress as u8, 
			data[0].0, data[0].1,
			data[1].0, data[1].1,
			data[2].0, data[2].1,
			data[3].0, data[3].1
		],
		true
	);
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(info: &mut app::Info) {
	if info.change_flags.speed {
		let digits: [char; 4] = get_digits(info.speed);
		write(&digits, true);
	}
}