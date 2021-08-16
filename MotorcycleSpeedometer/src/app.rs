//==============================================================================
// Notes
//==============================================================================
// app::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use super::drivers::{fuel, odometer};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub struct InfoChangeFlags{
	pub battery_voltage: bool,
	pub fuel_level: bool,
	pub speed: bool,
	pub odometer: bool,
}

pub struct InfoFlags{
	
}

pub struct Info {
	pub change_flags: InfoChangeFlags,
	pub flags: InfoFlags,

	pub battery_voltage: u16,
	pub fuel_level: fuel::FuelLevel,
	pub speed: u16,
	pub odometer: odometer::Odometer,
}

//==============================================================================
// Variables
//==============================================================================
static mut DEVICE_INFO: bool = false;

const DEVICE_INFO_DEFAULTS: Info = Info {
	change_flags: InfoChangeFlags {
		battery_voltage: false,
		fuel_level: false,
		speed: false,
		odometer: false,
	},
	flags: InfoFlags { },
	battery_voltage: 0,
	fuel_level: fuel::FuelLevel::LevelUnknown,
	speed: 0,
	odometer: odometer::Odometer {
		odometer: 0,
		trip: 0,
	}
};

//==============================================================================
// Public Functions
//==============================================================================
impl Info {
	pub fn take() -> Option<Self> {
		cortex_m::interrupt::free(|_| {
			if unsafe { DEVICE_INFO } {
				None
			} else {
				Some(unsafe { Info::steal() })
			}
		})
	}
	pub unsafe fn steal() -> Self {
		DEVICE_INFO = true;
		DEVICE_INFO_DEFAULTS
	}
}


//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn get_unhandled_flags(_flags: &InfoChangeFlags) -> bool {
	false
}

#[allow(dead_code)]
fn get_busy(_info: &mut Info) -> bool {
	false
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(_info: &mut Info) {

}
