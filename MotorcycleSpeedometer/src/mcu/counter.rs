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
use crate::mcu::gpio;
use msp432p401r_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum TaClk {
	A0, 
	A1, 
	A2, 
	A3
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub struct Counter{
	pub taclk_port: mcu::Port,
	pub taclk_pin: u8,
	pub taclk: TaClk,
	pub function_select: u8,
}

//==============================================================================
// Variables
//==============================================================================
static TIMER_A0_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER_A0>>> = 
	Mutex::new(RefCell::new(None));
static TIMER_A1_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER_A1>>> = 
	Mutex::new(RefCell::new(None));
static TIMER_A2_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER_A2>>> = 
	Mutex::new(RefCell::new(None));
static TIMER_A3_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER_A3>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(
	timer_a0: msp432p401r_pac::TIMER_A0,
	timer_a1: msp432p401r_pac::TIMER_A1,
	timer_a2: msp432p401r_pac::TIMER_A2,
	timer_a3: msp432p401r_pac::TIMER_A3,
){
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| TIMER_A0_HANDLE.borrow(cs).replace(Some(timer_a0)));
	free(|cs| TIMER_A1_HANDLE.borrow(cs).replace(Some(timer_a1)));
	free(|cs| TIMER_A2_HANDLE.borrow(cs).replace(Some(timer_a2)));
	free(|cs| TIMER_A3_HANDLE.borrow(cs).replace(Some(timer_a3)));

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn setup(counter: &Counter) {
	gpio::pin_setup(&gpio::PinConfig {
		port: counter.taclk_port,
		pin: counter.taclk_pin,
		direction: gpio::PinDirection::Input,
		pull: gpio::PinPull::PullUp,
		state: gpio::PinState::PinHigh
	});
	gpio::set_pin_function_select(
		&gpio::PinConfig {
			port: counter.taclk_port,
			pin: counter.taclk_pin,
			direction: gpio::PinDirection::Input,
			pull: gpio::PinPull::PullUp,
			state: gpio::PinState::PinHigh
		},
		counter.function_select
	);
	
	free(|cs| {
		match counter.taclk {
			TaClk::A0 => {
				if let Some(ref mut timer) = TIMER_A0_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					// Stop the timer for config
					timer.tax_ctl.write(|w| w.mc().mc_0());
					
					timer.tax_ctl.modify(|_, w| w
						.tassel().tassel_0()	// Use TA CLK as count source
						.id().id_0()			// Use no divider
						.taclr().set_bit()		// Clear just for good measure
					);
				}
			},
			TaClk::A1 => {
				if let Some(ref mut timer) = TIMER_A1_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					// Stop the timer for config
					timer.tax_ctl.write(|w| w.mc().mc_0());
					
					timer.tax_ctl.modify(|_, w| w
						.tassel().tassel_0()	// Use TA CLK as count source
						.id().id_0()			// Use no divider
						.taclr().set_bit()		// Clear just for good measure
					);
				}
			},
			TaClk::A2 => {
				if let Some(ref mut timer) = TIMER_A2_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					// Stop the timer for config
					timer.tax_ctl.write(|w| w.mc().mc_0());
					
					timer.tax_ctl.modify(|_, w| w
						.tassel().tassel_0()	// Use TA CLK as count source
						.id().id_0()			// Use no divider
						.taclr().set_bit()		// Clear just for good measure
					);
				}
			},
			TaClk::A3 => {
				if let Some(ref mut timer) = TIMER_A3_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					// Stop the timer for config
					timer.tax_ctl.write(|w| w.mc().mc_0());
					
					timer.tax_ctl.modify(|_, w| w
						.tassel().tassel_0()	// Use TA CLK as count source
						.id().id_0()			// Use no divider
						.taclr().set_bit()		// Clear just for good measure
					);
				}
			},
		}
	})
}

#[allow(dead_code)]
pub fn start(counter: &Counter, start: bool) {
	free(|cs| {
		match counter.taclk {
			TaClk::A0 => {
				if let Some(ref mut timer) = TIMER_A0_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					if start {
						timer.tax_ctl.modify(|_, w| w.mc().mc_2());
					}
					else {
						timer.tax_ctl.modify(|_, w| w.mc().mc_0());
					}
				}
			},
			TaClk::A1 => {
				if let Some(ref mut timer) = TIMER_A1_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					if start {
						timer.tax_ctl.modify(|_, w| w.mc().mc_2());
					}
					else {
						timer.tax_ctl.modify(|_, w| w.mc().mc_0());
					}
				}
			},
			TaClk::A2 => {
				if let Some(ref mut timer) = TIMER_A2_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					if start {
						timer.tax_ctl.modify(|_, w| w.mc().mc_2());
					}
					else {
						timer.tax_ctl.modify(|_, w| w.mc().mc_0());
					}
				}
			},
			TaClk::A3 => {
				if let Some(ref mut timer) = TIMER_A3_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					if start {
						timer.tax_ctl.modify(|_, w| w.mc().mc_2());
					}
					else {
						timer.tax_ctl.modify(|_, w| w.mc().mc_0());
					}
				}
			},
		}
	});
}

#[allow(dead_code)]
pub fn get_count(counter: &Counter) -> u16 {
	free(|cs| {
		match counter.taclk {
			TaClk::A0 => {
				if let Some(ref mut timer) = TIMER_A0_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					timer.tax_ctl.modify(|_, w| w.mc().mc_0());
					let read: u16 = timer.tax_r.read().bits();
					timer.tax_ctl.modify(|_, w| w.mc().mc_2().taclr().set_bit());
					read
				}
				else {
					0
				}
			},
			TaClk::A1 => {
				if let Some(ref mut timer) = TIMER_A1_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					timer.tax_ctl.modify(|_, w| w.mc().mc_0());
					let read: u16 = timer.tax_r.read().bits();
					timer.tax_ctl.modify(|_, w| w.mc().mc_2().taclr().set_bit());
					read
				}
				else {
					0
				}
			},
			TaClk::A2 => {
				if let Some(ref mut timer) = TIMER_A2_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					timer.tax_ctl.modify(|_, w| w.mc().mc_0());
					let read: u16 = timer.tax_r.read().bits();
					timer.tax_ctl.modify(|_, w| w.mc().mc_2().taclr().set_bit());
					read
				}
				else {
					0
				}
			},
			TaClk::A3 => {
				if let Some(ref mut timer) = TIMER_A3_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					timer.tax_ctl.modify(|_, w| w.mc().mc_0());
					let read: u16 = timer.tax_r.read().bits();
					timer.tax_ctl.modify(|_, w| w.mc().mc_2().taclr().set_bit());
					read
				}
				else {
					0
				}
			},
		}
	})
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
