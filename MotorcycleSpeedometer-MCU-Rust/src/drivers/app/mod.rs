//==============================================================================
// Notes
//==============================================================================
// drivers::app::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use cortex_m::asm::wfi;
use crate::drivers;
pub mod app;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub struct DeviceInfoFlags{
	pub debug_log_active: bool,
}

pub struct DeviceInfo {
	pub flags: DeviceInfoFlags,
    pub time: drivers::clock::Time,
}

pub enum AppState {
    BusyLcd,
    BusyTimer,
    Idle,
}

//==============================================================================
// Variables
//==============================================================================
static mut DEVICE_INFO: bool = false;

const DEVICE_INFO_DEFAULTS: DeviceInfo = DeviceInfo {
    flags: DeviceInfoFlags {
        debug_log_active: true 
    },
    time: drivers::clock::Time {
        hours: 0,
        minutes: 0, 
        seconds: 0
    }
};

//==============================================================================
// Public Functions
//==============================================================================
impl DeviceInfo {
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_INFO } {
                None
            } else {
                Some(unsafe { DeviceInfo::steal() })
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


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {
    let state = app::get_state();
    match state {
        AppState::Idle => wfi(),
        _ => ()
    }
}