//==============================================================================
// Notes
//==============================================================================
// drivers/speedometer.rs
// The means for determing speed

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::{app, config};
use crate::mcu;
use crate::mcu::{counter, gpio, rtc};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const COUNTER: counter::Counter = counter::Counter {
	taclk_port: config::COUNTER_TACLK_PORT,
	taclk_pin: config::COUNTER_TACLK_PIN,
	taclk: config::COUNTER_TACLK,
	function_select: config::COUNTER_FUNCTION_SELECT
};

const UPDATE_TIME_MS: u32 = 250;

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	counter::setup(&COUNTER);
	counter::start(&COUNTER, true);
	
	gpio::pin_setup(&gpio::PinConfig {
		port: mcu::Port::Port2,
		pin: 5,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh
	});
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(info: &mut app::Info){
	static mut LAST_TIME: u32 = 0;
	static mut LAST_TIME_MS: u32 = 0;
	
	// Clear any previously set flags
	if info.change_flags.speed {
		info.change_flags.speed = false;
	}
	
	unsafe { 
		if rtc::get_diff_ms(LAST_TIME, LAST_TIME_MS) > UPDATE_TIME_MS {
			LAST_TIME_MS = rtc::get_time_ms();
			LAST_TIME = rtc::get_time();
			let speed = counter::get_count(&COUNTER);
			
			if info.speed != speed {
				info.change_flags.speed = true;
				info.speed = speed;
			}
		}
	}
}