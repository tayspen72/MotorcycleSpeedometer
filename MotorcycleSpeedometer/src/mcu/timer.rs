//==============================================================================
// Notes
//==============================================================================
// mcu::counter.rs
// A GPIO Pin Counter Using the Internal Timer_A Peripheral

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::mcu;
use crate::mcu::nvic;
use msp432p401r_pac;
use msp432p401r_pac::interrupt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
const TIMER32_INT1_IRQN: u8 = 25;

//==============================================================================
// Variables
//==============================================================================
static TIMER32_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER32>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;
static mut TIME_ELAPSED: u32 = 0;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(timer: msp432p401r_pac::TIMER32) {
	unsafe { if INITIALIZED {
		return;
	}}

	configure(&timer);
	
	free(|cs| TIMER32_HANDLE.borrow(cs).replace(Some(timer)));

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn delay(time: u32) {
	unsafe {
		if !INITIALIZED {
			return;
		}
	}
	start(true);
	
	unsafe { while TIME_ELAPSED < time {} }
	
	start(false);
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(timer: &msp432p401r_pac::TIMER32) {
	timer.t32control1.write(|w| w.enable().clear_bit());
	
	timer.t32control1.write(|w| w
		.mode().mode_1()			// Periodic
		.ie().ie_1()				// Enable Interrupt
		.prescale().prescale_1()	// MCLK / 256
		.size().size_1()			// 32-bit Timer
		.oneshot().oneshot_0()		// Wrapping
	);
	
	// Set the reload to fire with 1ms precision
	let timer_clock = mcu::get_system_clock().m_clk / 256;
	timer.t32load1.write(|w| unsafe { w.load().bits(timer_clock / 100)});
	
	nvic::clear_pending(TIMER32_INT1_IRQN);
	nvic::enable(TIMER32_INT1_IRQN);
}

fn start(enabled: bool) {
	free(|cs| {
		if let Some(ref mut timer) = TIMER32_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if timer.t32control1.read().enable().is_enable_1() {
				timer.t32control1.modify(|_, w| w.enable().clear_bit());
			}
			
			unsafe { TIME_ELAPSED = 0; }

			if enabled { 
				timer.t32control1.modify(|_, w| w.enable().set_bit());
			}
		}
	});
}

//==============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
fn T32_INT1_IRQ() {
	free(|cs| {
		if let Some(ref mut timer) = TIMER32_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			timer.t32intclr1.write(|w| unsafe { w.bits(0xFFFF) });
		}
	});
	unsafe { TIME_ELAPSED += 1; }
}

//==============================================================================
// Task Handler
//==============================================================================
