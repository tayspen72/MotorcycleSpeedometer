//==============================================================================
// Notes
//==============================================================================
// mcu::rtc.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::{Cell, RefCell};
use cortex_m::interrupt::{free, Mutex};
use nrf52832_pac;
use nrf52832_pac::interrupt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum WakeInterval {
	Interval125MS	= 512,
	Interval250MS	= 1024,
	Interval500MS	= 2048,
	Interval1S 		= 4096
}

//==============================================================================
// Variables
//==============================================================================
static _WAKE_INTERVAL: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static _FRACTION: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static _SECONDS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

static RTC_HANDLE: Mutex<RefCell<Option<nrf52832_pac::RTC0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(rtc: nrf52832_pac::RTC0, clock: &nrf52832_pac::CLOCK, interval: WakeInterval) {
	free(|cs| _WAKE_INTERVAL.borrow(cs).set(interval as u32));

	// Enable after HANDLE has been initialized so the mutex is not 'None'
	configure(&rtc, clock);

	// Pull the RTC0 reference from the peripherals and add to the mutex here
	free(|cs| RTC_HANDLE.borrow(cs).replace(Some(rtc)));
}

#[allow(dead_code)]
pub fn get_timestamp() -> u32 {
	free(|cs| _SECONDS.borrow(cs).get())
}

#[allow(dead_code)]
pub fn get_timediff(seconds: u32) -> u32 {
	let app_seconds = free(|cs| _SECONDS.borrow(cs).get());
	app_seconds - seconds
}

#[allow(dead_code)]
pub fn get_timestamp_fraction() -> u32 {
	free(|cs| _FRACTION.borrow(cs).get())
}

#[allow(dead_code)]
pub fn get_timediff_fraction(seconds: u32, fraction: u32) -> u32 {
	let app_seconds = free(|cs| _SECONDS.borrow(cs).get());
	let app_fraction = free(|cs| _FRACTION.borrow(cs).get());

	((app_seconds * 1000) + app_fraction) - ((seconds * 1000) + fraction)
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(rtc: &nrf52832_pac::RTC0, clock: &nrf52832_pac::CLOCK) {
	nrf52832_pac::NVIC::mask(nrf52832_pac::Interrupt::RTC0);

	// Enable XTAL for Low Freq Clock Source
	clock.lfclksrc.write(|w| w.src().xtal());
	clock.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
	//TODO: waiting indefinitely here
	while clock.events_lfclkstarted.read().bits() == 0 {};

	//Disable RTC
	rtc.tasks_stop.write(|w| unsafe { w.bits(1) });
	
	//prescale by 8 : 32768 / 8 = 4096 Hz
	rtc.prescaler.write(|w| unsafe { w.prescaler().bits(7) });


	//define the wake interval
	rtc.cc[0].write(|w| unsafe { w.bits(
		free(|cs| _WAKE_INTERVAL.borrow(cs).get())
	) });

	//connect the interrupt event signal on compare0 match
	rtc.intenset.write(|w| w.compare0().set_bit());

	unsafe {
		nrf52832_pac::NVIC::unpend(nrf52832_pac::Interrupt::RTC0);
		nrf52832_pac::NVIC::unmask(nrf52832_pac::Interrupt::RTC0);
	}

	//Enable RTC
	rtc.tasks_start.write(|w| unsafe { w.bits(1) });

	free(|cs| _SECONDS.borrow(cs).set(0));
	free(|cs| _FRACTION.borrow(cs).set(0));
}


#[allow(dead_code)]
fn empty_function(){}

// =============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
fn RTC0() {
	let mut fraction: u32 = free(|cs| _FRACTION.borrow(cs).get());
	let mut seconds: u32 = free(|cs| _SECONDS.borrow(cs).get());
	let wake_interval: u32 = free(|cs| _WAKE_INTERVAL.borrow(cs).get());

	free(|cs| {
		let rtc = RTC_HANDLE.borrow(cs).borrow();
		let rtc0 = rtc.as_ref().unwrap();

		if rtc0.events_compare[0].read().bits() > 0 {
				fraction += wake_interval;
		
			if fraction >= WakeInterval::Interval1S as u32 {
				seconds += fraction / WakeInterval::Interval1S as u32;
				fraction = 0;
			}
		}

		rtc0.events_compare[0].write(|w| unsafe { w.bits(0) });
		rtc0.tasks_clear.write(|w| unsafe { w.bits(1) });
	});

	free(|cs| _FRACTION.borrow(cs).set(fraction));
	free(|cs| _SECONDS.borrow(cs).set(seconds));
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(){

}