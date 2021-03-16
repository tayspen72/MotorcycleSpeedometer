//==============================================================================
// Notes
//==============================================================================
// drivers::debug.rs
// The debug library is meant to be a scrolling log of entries. The log will 
// need to be built. Later.
// The log can be hidden in real-time, as needed. Maybe with a swipe up action?

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::Cell;
use cortex_m::interrupt::{free, Mutex};
use super::app;
use super::lcd::{lcd_api, font};

//==============================================================================
// Enums, Structs, and Types
//=============================================================================
#[derive(Copy, Clone)]
struct LogLine{
	active: bool,
	stale: bool,
	line: [char; 24]
}

//==============================================================================
// Variables
//==============================================================================
 const DEBUG_INITIAL_X: u16 = 0;
 const DEBUG_INITIAL_Y: u16 = 137;
 const DEBUG_SCALE: u16 = 2;
 const DEBUG_BACKGROUND: u16 = lcd_api::Color::Black as u16;
 const DEBUG_FOREGROUND: u16 = lcd_api::Color::White as u16;
 const DEBUG_WELCOME: [char; 24] = [
	'*', '*', ' ', ' ', ' ', ' ', 'D', 'e', 'b', 'u', 'g', ' ', 
	'O', 'u', 't', 'p', 'u', 't', ' ', ' ', ' ', ' ', '*', '*'
 ];

static LOG_LINES_ACTIVE: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));
static LOG_LINE_COUNT: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));
const LOG_LINE_ENTRIES: usize = 5;
static LOG_LINES: Mutex<Cell<[LogLine; 6]>> = Mutex::new(Cell::new( [
	LogLine { active: true, stale: true, line: DEBUG_WELCOME },
	LogLine { active: false, stale: true, line: [ '-'; 24 ] },
	LogLine { active: false, stale: true, line: [ '-'; 24 ] },
	LogLine { active: false, stale: true, line: [ '-'; 24 ] },
	LogLine { active: false, stale: true, line: [ '-'; 24 ] },
	LogLine { active: false, stale: true, line: [ '-'; 24 ] }
]));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {

}

#[allow(dead_code)]
pub fn push_log(string: &'static str) {
	if free(|cs| LOG_LINES_ACTIVE.borrow(cs).get()) == LOG_LINE_ENTRIES {
		pop_log();
	}

	free(|cs| LOG_LINES_ACTIVE.borrow(cs).set(LOG_LINES_ACTIVE.borrow(cs).get() + 1));
	
	let index = free(|cs| LOG_LINES_ACTIVE.borrow(cs).get());
	let len = if string.len() > 21 { 21 } else { string.len() };
	let string = string.as_bytes();
	let count = free(|cs| LOG_LINE_COUNT.borrow(cs).get());
	let count = if count + 1 == 100 { 0 } else { count + 1 };
	free(|cs| LOG_LINE_COUNT.borrow(cs).set(count));

	free(|cs| {
		let mut log_lines = LOG_LINES.borrow(cs).get();
		log_lines[index].active = true;
		log_lines[index].stale = true;
		// Print debug line count
		log_lines[index].line[0] = (0x30 + ((count / 10) % 10)) as char;
		log_lines[index].line[1] = (0x30 + (count % 10)) as char;
		log_lines[index].line[2] = ':';

		// Copy bytes from string into the log lines object
		for i in 0..len {
			log_lines[index].line[i+3] = string[i] as char;
		}
		if len < 21 {
			log_lines[index].line[len+3] = 0 as char;
		}

		LOG_LINES.borrow(cs).set(log_lines);
	});
}

#[allow(dead_code)]
pub fn push_log_number(string: &'static str, num: &u32) {
	if free(|cs| LOG_LINES_ACTIVE.borrow(cs).get()) == LOG_LINE_ENTRIES {
		pop_log();
	}

	free(|cs| LOG_LINES_ACTIVE.borrow(cs).set(LOG_LINES_ACTIVE.borrow(cs).get() + 1));
	
	let index = free(|cs| LOG_LINES_ACTIVE.borrow(cs).get());
	let string_len = if string.len() > 21 { 21 } else { string.len() };
	let string = string.as_bytes();
	let count = free(|cs| LOG_LINE_COUNT.borrow(cs).get());
	let count = if count + 1 == 100 { 0 } else { count + 1 };
	let num_len = get_num_len(*num);

	free(|cs| LOG_LINE_COUNT.borrow(cs).set(count));

	free(|cs| {
		let mut log_lines = LOG_LINES.borrow(cs).get();
		log_lines[index].active = true;
		log_lines[index].stale = true;
		// Print debug line count
		log_lines[index].line[0] = (0x30 + ((count / 10) % 10)) as char;
		log_lines[index].line[1] = (0x30 + (count % 10)) as char;
		log_lines[index].line[2] = ':';

		// Copy bytes from string into the log lines object
		for i in 0..string_len {
			log_lines[index].line[i+3] = string[i] as char;
		}
		let mut div: u32 = 1;
		for i in 0..num_len {
			let c = 3 + string_len + num_len - 1 - i;
			log_lines[index].line[c] = 
				((0x30 + ((num / div) % 10)) as u8) as char;
			div *= 10;
		}

		if string_len + num_len < 21 {
			log_lines[index].line[string_len+num_len+3] = 0 as char;
		}

		LOG_LINES.borrow(cs).set(log_lines);
	});
}

//==============================================================================
// Private Functions
//==============================================================================
fn clear_line(line_number: usize) {
	let y = DEBUG_INITIAL_Y + ((line_number as u16) * font::MINIMAL_CHARACTER_HEIGHT * DEBUG_SCALE);
	lcd_api::fill_rectangle(0, 240, y, font::MINIMAL_CHARACTER_HEIGHT * DEBUG_SCALE, DEBUG_BACKGROUND);
}

fn get_num_len(mut num: u32) -> usize {
	let mut len: usize = 1;
	num /= 10;
	while num > 0 {
		len += 1;
		num /= 10;
	}

	len
}

fn pop_log() {
	// Shift all entries up one - leaving the bottom entry available
	let num_entries = free(|cs| LOG_LINES_ACTIVE.borrow(cs).get());
	free(|cs| {
		let mut log_lines = LOG_LINES.borrow(cs).get();
		for i in 1..num_entries {
			log_lines[i].active = true;
			log_lines[i].stale = true;
			log_lines[i].line = log_lines[i+1].line;
		}
		log_lines[num_entries].active = false;
		LOG_LINES.borrow(cs).set(log_lines);
	});

	// Show that a line has just been popped
	free(|cs| LOG_LINES_ACTIVE.borrow(cs).set(LOG_LINES_ACTIVE.borrow(cs).get() - 1));
}

fn write_character(c: char, x: u16, y: u16) {
	font::write_minimal_character(c, x, y, DEBUG_FOREGROUND, DEBUG_BACKGROUND, DEBUG_SCALE);
}

fn write_line(line_number: usize) {
	let bytes = free(|cs| LOG_LINES.borrow(cs).get()[line_number].line);
	let len = bytes.len();

	let mut x = DEBUG_INITIAL_X;
	let y = DEBUG_INITIAL_Y + ((line_number as u16) * font::MINIMAL_CHARACTER_HEIGHT * DEBUG_SCALE);

	for i in 0..len {
		if bytes[i] == 0 as char{
			break;
		}
		
		write_character(bytes[i] as char, x, y);
		x += font::MINIMAL_CHARACTER_WIDTH * DEBUG_SCALE;
	}

	// Update the stale line flag showing it has been displayed
	free(|cs| {
		let mut log_lines = LOG_LINES.borrow(cs).get();
		log_lines[line_number].stale = false;
		LOG_LINES.borrow(cs).set(log_lines);
	});
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &app::DeviceInfo) {
	if d.flags.debug_log_active {
		let len = free(|cs| LOG_LINES_ACTIVE.borrow(cs).get());

		for i in 0..=len {
			// If log lines are current, do nothing
			if !free(|cs| LOG_LINES.borrow(cs).get()[i].active) {
				return;
			}

			if free(|cs| LOG_LINES.borrow(cs).get()[i].stale) {
				clear_line(i);
				write_line(i);
			}
		}
	}
}
