//==============================================================================
// Notes
//==============================================================================
// mcu::rtc.rs
// RTC Driver

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::mcu::nvic;
use msp432p401r_pac;
use msp432p401r_pac::interrupt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum WakeInterval {
	Wake125Ms = 125,
	Wake250Ms = 250,
	Wake500Ms = 500,
	Wake1000Ms = 1000
}

//==============================================================================
// Variables
//==============================================================================
static RTC_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::RTC_C>>> = 
	Mutex::new(RefCell::new(None));
static CURRENT_TIME: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static CURRENT_TIME_MS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static mut INITIALIZED: bool = false;
static mut INTERVAL: u32 = 0;
const RTC_C_IRQ_N: u8 = 29;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(rtc: msp432p401r_pac::RTC_C, wake_interval: WakeInterval) {
	unsafe { if INITIALIZED {
		return;
	}}

	configure(&rtc, wake_interval);

	free(|cs| RTC_HANDLE.borrow(cs).replace(Some(rtc)));

	unsafe {
		INTERVAL = wake_interval as u32;
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn get_diff(time: u32) -> u32 {
	unsafe { if !INITIALIZED {
		return 0;
	}}

	let current = free(|cs| CURRENT_TIME.borrow(cs).get());
	if current < time {
		0
	}
	else {
		current - time
	}
}

#[allow(dead_code)]
pub fn get_diff_ms(time: u32, time_ms: u32) -> u32 {
	unsafe { if !INITIALIZED {
		return 0;
	}}

	let current = free(|cs| CURRENT_TIME.borrow(cs).get());
	let current_ms = free(|cs| CURRENT_TIME_MS.borrow(cs).get());
	let current_actual = current + current_ms;
	let time_actual = time + time_ms;

	if current_actual < time_actual {
		0
	}
	else {
		current_actual - time_actual
	}
}

#[allow(dead_code)]
pub fn get_time() -> u32 {
	unsafe { if !INITIALIZED {
		return 0;
	}}

	free(|cs| CURRENT_TIME.borrow(cs).get())
}

#[allow(dead_code)]
pub fn get_time_ms() -> u32 {
	unsafe { if !INITIALIZED {
		return 0;
	}}

	free(|cs| CURRENT_TIME_MS.borrow(cs).get())
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(rtc: &msp432p401r_pac::RTC_C, wake_interval: WakeInterval) {
	// Unlock the RTC registers
	rtc.rtcctl0.write(|w| unsafe { w.rtckey().bits(0xA5) });

	// Hold the RTC during config
	rtc.rtcctl13.write(|w| w.rtchold().set_bit());

	// Set RTOPS to output 32768 / 256 = 128Hz clock
	rtc.rtcps0ctl.write(|w| w.rt0ip().rt0ip_7());

	// Set RT1IP to fire at the wake interval and trigger an interrupt
	let interval = match wake_interval {
		WakeInterval::Wake125Ms => 3,
		WakeInterval::Wake250Ms => 4,
		WakeInterval::Wake500Ms => 5,
		WakeInterval::Wake1000Ms => 6,
	};
	rtc.rtcps1ctl.write(|w| w
		.rt1ip().bits(interval)
		.rt1psie().set_bit()
		.rt1psifg().clear_bit()
	);

	// Set the interrupt bit
	nvic::clear_pending(RTC_C_IRQ_N);
	nvic::enable(RTC_C_IRQ_N);

	// Release hold on the RTC
	rtc.rtcctl13.modify(|_, w| w.rtchold().clear_bit());

	// Lock the refisters when finished
	rtc.rtcctl0.write(|w| unsafe { w.rtckey().bits(0xFF) });
}

//==============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
fn RTC_C_IRQ () {
	// Service interrupt flag
	free(|cs| {
		if let Some(ref mut rtc) = RTC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Unlock register access
			rtc.rtcctl0.write(|w| unsafe { w.rtckey().bits(0xA5) });

			// Clear interrupt flag bit
			rtc.rtcps1ctl.modify(|_, w| w.rt1psifg().clear_bit());

			// Lock when finished
			rtc.rtcctl0.write(|w| unsafe { w.rtckey().bits(0xFF) });
		}
	});

	let mut time = free(|cs| CURRENT_TIME.borrow(cs).get());
	let mut time_ms = unsafe { free(|cs| CURRENT_TIME_MS.borrow(cs).get()) + INTERVAL };

	if time_ms >= 1000 {
		time_ms = 0;
		time += 1;
	}

	free(|cs| CURRENT_TIME.borrow(cs).set(time));
	free(|cs| CURRENT_TIME_MS.borrow(cs).set(time_ms));
}

//==============================================================================
// Task Handler
//==============================================================================
