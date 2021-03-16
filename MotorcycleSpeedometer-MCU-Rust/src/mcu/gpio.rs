//==============================================================================
// Notes
//==============================================================================
// mcu::gpio.rs
// Basic control over gpio pins

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use nrf52832_pac;

use nrf52832_pac::p0::pin_cnf::DIR_A as DIR;
use nrf52832_pac::p0::pin_cnf::PULL_A as PULL;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinDirection{
	Input,
	Output
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinPull{
	PullUp,
	PullDown,
	PullDisabled
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinState{
	PinLow,
	PinHigh
}

//==============================================================================
// Variables
//==============================================================================
static GPIO_HANDLE: Mutex<RefCell<Option<nrf52832_pac::P0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(p0: nrf52832_pac::P0) {
	free(|cs| GPIO_HANDLE.borrow(cs).replace(Some(p0)));
}

#[allow(dead_code)]
pub fn get_pin_state(pin: u8) -> PinState {
	let read = free(|cs| GPIO_HANDLE.borrow(cs).borrow().as_ref().unwrap().in_.read().bits());
	match read & (1 << pin) {
		0 => PinState::PinLow,
		_ => PinState::PinHigh
	}
}

#[allow(dead_code)]
pub fn pin_disable(pin: u8) {
	free(|cs| {
		let p = GPIO_HANDLE.borrow(cs).borrow();
		let p0 = p.as_ref().unwrap();
		// Set as input and disconnect the buffer
		p0.pin_cnf[pin as usize].modify(|_, w| w.dir().input());
		p0.pin_cnf[pin as usize].modify(|_, w| w.input().disconnect());

	});
}

#[allow(dead_code)]
pub fn pin_setup(pin: u8, dir: DIR, state: PinState, pull: PULL){
	free(|cs| {
		let p = GPIO_HANDLE.borrow(cs).borrow();
		let p0 = p.as_ref().unwrap();
		
		// Set direction
		p0.pin_cnf[pin as usize].modify(|_, w| w.dir().variant(dir));
		if let DIR::INPUT = dir {
			p0.pin_cnf[pin as usize].modify(|_, w| w.input().connect());
		}
		else {
			p0.pin_cnf[pin as usize].modify(|_, w| w.input().disconnect());
		}
		
		// Set pin pull
		p0.pin_cnf[pin as usize].modify(|_, w| w.pull().variant(pull));

		// Set output state
		match state {
			PinState::PinLow => p0.outclr.write(|w| unsafe {w.bits(1 << pin)}),
			PinState::PinHigh => p0.outset.write(|w| unsafe {w.bits(1 << pin)})
		}
	});
}

#[allow(dead_code)]
pub fn set_pin_state(pin: u8, state: PinState){
	match state {
		PinState::PinLow => {
			free(|cs| GPIO_HANDLE.borrow(cs).borrow().as_ref().unwrap().outclr.write(|w| unsafe { w.bits(1 << pin) }))
		},
		PinState::PinHigh => {
			free(|cs| GPIO_HANDLE.borrow(cs).borrow().as_ref().unwrap().outset.write(|w| unsafe { w.bits(1 << pin) }))
		}
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
