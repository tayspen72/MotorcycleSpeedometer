//==============================================================================
// Notes
//==============================================================================
// mcu::wdt.rs
// Watchdog Timer Control

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use msp432p401r_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static WDT_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::WDT_A>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(wdt: msp432p401r_pac::WDT_A){
	unsafe { if INITIALIZED {
		return;
	}}

	// Disable the watchdog (indefinitely, for now)
	wdt.wdtctl.write(|w| unsafe { w
		.wdtpw().bits(0x5A)
		.wdthold().wdthold_1()
		.wdtis().wdtis_0()
	});
	
	free(|cs| WDT_HANDLE.borrow(cs).replace(Some(wdt)));

	unsafe {
		INITIALIZED = true;
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