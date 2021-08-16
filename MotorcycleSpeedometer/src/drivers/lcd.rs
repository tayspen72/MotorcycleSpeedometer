//==============================================================================
// Notes
//==============================================================================
// drivers/lcd.rs
// 2x16 LCD driver

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::{gpio, timer};
use gpio::PinState as PinState;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
enum Command {
	ClearDisplay =	0x01,
	ReturnHome =	0x02,
	EntryModeSet =	0x04,
	DisplayOnOff =	0x08,
	CursorShift =	0x10,
	FunctionSet =	0x20,
	SetCgram =		0x40,
	SetDdram =		0x80,
}

#[allow(dead_code)]
enum DisplayModeBlink {
	Off = 0,
	On = 1,	
}

#[allow(dead_code)]
enum DisplayModeCursor {
	Off = 0,
	On = 1,	
}

#[allow(dead_code)]
enum DisplayModeDisplay {
	Off = 0,
	On = 1,	
}

#[allow(dead_code)]
enum EntryModeIncrement {
	Decrement = 0,
	Increment = 1,	
}

#[allow(dead_code)]
enum EntryModeShift {
	Right = 0,
	Left = 1,	
}

#[allow(dead_code)]
enum FunctionSetDataLength {
	Bits4 = 0,
	Bits8 = 1
}

#[allow(dead_code)]
enum FunctionSetDisplayLines {
	Lines1 = 0,
	Lines2 = 1
}

#[allow(dead_code)]
enum FunctionSetFontType {
	Type5x8,
	Type5x11
}

//==============================================================================
// Variables
//==============================================================================
const RS: gpio::PinConfig = gpio::PinConfig {
	port: config::LCD_RS_PORT,
	pin: config::LCD_RS_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const RW: gpio::PinConfig = gpio::PinConfig {
	port: config::LCD_RW_PORT,
	pin: config::LCD_RW_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const EN: gpio::PinConfig = gpio::PinConfig {
	port: config::LCD_EN_PORT,
	pin: config::LCD_EN_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const D: [gpio::PinConfig; 8] = [
	gpio::PinConfig {
		port: config::LCD_D0_PORT,
		pin: config::LCD_D0_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D1_PORT,
		pin: config::LCD_D1_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D2_PORT,
		pin: config::LCD_D2_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D3_PORT,
		pin: config::LCD_D3_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D4_PORT,
		pin: config::LCD_D4_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D5_PORT,
		pin: config::LCD_D5_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D6_PORT,
		pin: config::LCD_D6_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D7_PORT,
		pin: config::LCD_D7_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
];
const BLA: gpio::PinConfig = gpio::PinConfig {
	port: config::LCD_BLA_PORT,
	pin: config::LCD_BLA_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinHigh,
};

const LCD_ROW_LENGTH: usize = 16;
static mut DISPLAY: [[u8; LCD_ROW_LENGTH]; 2] = [ [ ' ' as u8; LCD_ROW_LENGTH ]; 2 ];

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	gpio::pin_setup(&RS);
	gpio::pin_setup(&RW);
	gpio::pin_setup(&EN);
	gpio::pin_setup(&BLA);
	
	for d in D.iter() {
		gpio::pin_setup(&d);
	}
	
	configure();
	
	unsafe {
		DISPLAY[0][0] = '1' as u8;
		DISPLAY[0][1] = '2' as u8;
		DISPLAY[0][2] = '.' as u8;
		DISPLAY[0][3] = '0' as u8;
		DISPLAY[0][4] = 'v' as u8;
		
		DISPLAY[0][12] = '4' as u8;
		DISPLAY[0][13] = '5' as u8;
		DISPLAY[0][14] = '.' as u8;
		DISPLAY[0][15] = '8' as u8;

		DISPLAY[1][0] = 'F' as u8;
		DISPLAY[1][1] = ':' as u8;
		DISPLAY[1][2] = 0xFF;
		DISPLAY[1][3] = 0xFF;
		DISPLAY[1][4] = 0xFF;
		DISPLAY[1][5] = 0x00;

		DISPLAY[1][10] = '2' as u8;
		DISPLAY[1][11] = '5' as u8;
		DISPLAY[1][12] = ',' as u8;
		DISPLAY[1][13] = '1' as u8;
		DISPLAY[1][14] = '2' as u8;
		DISPLAY[1][15] = '3' as u8;
	}
	
	write_display();
}

//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn clear_display() {
	write_command(PinState::PinLow, PinState::PinLow, Command::ClearDisplay as u8);
}

#[allow(dead_code)]
fn configure() {
	write_command(gpio::PinState::PinLow, gpio::PinState::PinLow, 0x03);
	timer::delay(5);
	write_command(gpio::PinState::PinLow, gpio::PinState::PinLow, 0x03);
	timer::delay(5);
	write_command(gpio::PinState::PinLow, gpio::PinState::PinLow, 0x03);
	timer::delay(5);
	write_command(gpio::PinState::PinLow, gpio::PinState::PinLow, 0x02);
	
	function_set(FunctionSetDataLength::Bits8, FunctionSetDisplayLines::Lines2, FunctionSetFontType::Type5x11);	
	display_on_off_set(DisplayModeDisplay::On, DisplayModeCursor::Off, DisplayModeBlink::Off);
	clear_display();
	entry_mode_set(EntryModeIncrement::Increment, EntryModeShift::Right);
	
	set_cgram_address(0x00);
	write_data(0x1F);
	write_data(0x11);
	write_data(0x11);
	write_data(0x11);
	write_data(0x11);
	write_data(0x11);
	write_data(0x11);
	write_data(0x1F);
}

#[allow(dead_code)]
fn display_on_off_set(display: DisplayModeDisplay, cursor: DisplayModeCursor, blink: DisplayModeBlink) {
	let mut val = Command::DisplayOnOff as u8;
	if let DisplayModeDisplay::On = display {
		val |= 0x4;
	}
	if let DisplayModeCursor::On = cursor {
		val |= 0x2;
	}
	if let DisplayModeBlink::On = blink {
		val |= 0x1;
	}
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn entry_mode_set(increment: EntryModeIncrement, shift: EntryModeShift) {
	let mut val = Command::EntryModeSet as u8;
	if let EntryModeIncrement::Increment = increment {
		val |= 0x2;
	}
	if let EntryModeShift::Left = shift {
		val |= 0x1;
	}
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn function_set(length: FunctionSetDataLength, lines: FunctionSetDisplayLines, font: FunctionSetFontType) {
	let mut val = Command::FunctionSet as u8;
	if let FunctionSetDataLength::Bits8 = length {
		val |= 0x10;
	}
	if let FunctionSetDisplayLines::Lines2 = lines {
		val |= 0x8;
	}
	if let FunctionSetFontType::Type5x11 = font {
		val |= 0x4;
	}
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn return_home() {
	write_command(PinState::PinLow, PinState::PinLow, Command::ReturnHome as u8);
}

#[allow(dead_code)]
fn set_backlight(is_on: bool) {
	let state = if is_on { PinState::PinLow } else { PinState::PinHigh };
	gpio::set_pin_state(BLA.port, BLA.pin, state);
}

#[allow(dead_code)]
fn set_cgram_address(address: u8) {
	let val = (address & 0x3F) | Command::SetCgram as u8;
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn set_ddram_address(address: u8) {
	let val = (address & 0x7F) | Command::SetDdram as u8;
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn write_address(address: u8) {
	write_command(PinState::PinHigh, PinState::PinHigh, address);
}

#[allow(dead_code)]
fn write_data(data: u8) {
	write_command(PinState::PinHigh, PinState::PinLow, data);
}

fn write_display() {
	unsafe {
		set_ddram_address(0x00);
		for c in 0..LCD_ROW_LENGTH {
			write_data(DISPLAY[0][c] as u8);
		}
		
		set_ddram_address(0x40);
		for c in 0..LCD_ROW_LENGTH {
			write_data(DISPLAY[1][c] as u8);
		}
	}
}

#[allow(dead_code)]
fn write_command(rs_state: gpio::PinState, rw_state: gpio::PinState, val: u8) {
	gpio::set_pin_state(RS.port, RS.pin, rs_state);
	gpio::set_pin_state(RS.port, RW.pin, rw_state);
		
	for i in 0..8 {
		let state = if (val >> i) & 1 > 0 { PinState::PinHigh } else { PinState::PinLow };
		gpio::set_pin_state(D[i].port, D[i].pin, state);
	}

	gpio::set_pin_state(EN.port, EN.pin, PinState::PinHigh);

	timer::delay(1);
	
	gpio::set_pin_state(EN.port, EN.pin, PinState::PinLow);
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(){
	
}