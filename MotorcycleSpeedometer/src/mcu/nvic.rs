//==============================================================================
// Notes
//==============================================================================
// mcu::nvic.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static NVIC_HANDLE: Mutex<RefCell<Option<cortex_m::peripheral::NVIC>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(nvic: cortex_m::peripheral::NVIC) {
	free(|cs| NVIC_HANDLE.borrow(cs).replace(Some(nvic)));
}

#[allow(dead_code)]
pub fn clear_pending(num: u8) {
	free(|cs| {
		if let Some(ref mut nvic) = NVIC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if num < 32 {
				let read = nvic.icpr[0].read();
				unsafe { nvic.icpr[0].write(read | (1 << num)) };
			}
			else {
				let num = num - 32;
				let read = nvic.icpr[1].read();
				unsafe { nvic.icpr[1].write(read | (1 << num)) };
			}
		}
	});
}

#[allow(dead_code)]
pub fn enable(num: u8) {
	free(|cs| {
		if let Some(ref mut nvic) = NVIC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if num < 32 {
				let read = nvic.iser[0].read();
				unsafe { nvic.iser[0].write(read | (1 << num)) };
			}
			else {
				let num = num - 32;
				let read = nvic.iser[1].read();
				unsafe { nvic.iser[1].write(read | (1 << num)) };
			}
		}
	});
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
