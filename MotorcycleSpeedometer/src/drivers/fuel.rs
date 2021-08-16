//==============================================================================
// Notes
//==============================================================================
// drivers/fuel.rs
// Fuel Level

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::{app, config};
use crate::mcu::{adc, rtc};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum FuelLevel {
	LevelUnknown,
	LevelEmpty,
	Level1,
	Level2,
	Level3,
	LevelFull,
}

const FUEL_ADC: adc::Adc = adc::Adc {
	port: config::FUEL_ADC_PORT,
	pin: config::FUEL_ADC_PIN,
	channel: config::FUEL_ADC_CHANNEL,
	function_select: config::FUEL_ADC_FUNCTION_SELECT,
	resolution: adc::Resolution::B14
};

//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
const FUEL_UPDATE_TIME: u32 = 5;	// Check fuel level every 5 seconds
const R1: f32 = 1000.0;
const V_REF: f32 = 3.30;

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	adc::configure(&FUEL_ADC);
}

#[allow(dead_code)]
pub fn get_fuel_level() -> FuelLevel {
	// Across the voltage divider, R2 is the Fuel Sensor
	let v_out = adc::read_ref(&FUEL_ADC, 2.5);
	let r2 = (R1 * v_out) / (V_REF - v_out );

	match r2 {
		i if i < 75.0 => FuelLevel::LevelEmpty,
		i if i < 100.0 => FuelLevel::Level1,
		i if i < 125.0 => FuelLevel::Level2,
		i if i < 150.0 => FuelLevel::Level3,
		i if i >= 150.0 => FuelLevel::LevelFull,
		_ => FuelLevel::LevelUnknown
	}
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(info: &mut app::Info){
	static mut LAST_TIME: u32 = 0;

	if info.change_flags.fuel_level {
		info.change_flags.fuel_level = false;
	}
	
	unsafe { 
		if rtc::get_diff(LAST_TIME) > FUEL_UPDATE_TIME {
			LAST_TIME = rtc::get_time();

			let level = get_fuel_level();
			if info.fuel_level != level {
				info.fuel_level = level;
				info.change_flags.fuel_level = true;
			}
		}
	}
}