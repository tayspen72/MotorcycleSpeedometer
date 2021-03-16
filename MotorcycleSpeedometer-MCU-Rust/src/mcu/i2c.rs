//==============================================================================
// Notes
//==============================================================================
// mcu::i2c.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use crate::config;
use nrf52832_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Copy, Clone)]
pub struct I2cLine {
	pub scl_pin: u8,
	pub sda_pin: u8,
	pub frequency: nrf52832_pac::twi0::frequency::FREQUENCY_A,
}

//==============================================================================
// Variables
//==============================================================================
static I2C_HANDLE: Mutex<RefCell<Option<nrf52832_pac::TWI0>>> = 
	Mutex::new(RefCell::new(None));

const I2C_LINE: I2cLine = I2cLine {
	scl_pin: config::I2C_SCL_PIN,
	sda_pin: config::I2C_SDA_PIN,
	frequency: config::I2C_FREQUENCY,
};

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(i2c: nrf52832_pac::TWI0){
	configure(&i2c);

	free(|cs| I2C_HANDLE.borrow(cs).replace(Some(i2c)));
}

#[allow(dead_code)]
pub fn write_byte(address: u8, byte: u8, send_start: bool, send_stop: bool) -> Option<bool> {	
	set_address(address);
	
	free(|cs| {
		let twi0 = I2C_HANDLE.borrow(cs).borrow();
		let i2c = twi0.as_ref().unwrap();

		// Preload the Txd register for sending
		i2c.txd.write(|w| unsafe { w.txd().bits(byte) } );
		
		if send_start {
			i2c.tasks_starttx.write(|w| unsafe { w.bits(1) } );
		}
		
		// Wait for rx event or error out
		while i2c.events_txdsent.read().bits() == i2c.events_error.read().bits() {}
		
		// If error, bail out
		if i2c.events_error.read().bits() > 0 {
			i2c.events_error.write(|w| unsafe { w.bits(0) });
			i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
			return None;
		}
		
		if send_stop {
			i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
		}
		
		// Clear out the Tx event flag
		i2c.events_txdsent.write(|w| unsafe { w.bits(0) });
		return Some(true);
	});

	Some(false)
}

#[allow(dead_code)]
pub fn write_data(address: u8, data: &[u8], send_start: bool, send_stop: bool) -> Option<bool> {
	if send_start {
		free(|cs| {
			let twi0 = I2C_HANDLE.borrow(cs).borrow();
			let i2c = twi0.as_ref().unwrap();
			i2c.tasks_starttx.write(|w| unsafe { w.bits(1) } );
		});
	}
	
	for i in 0..data.len() {
		match write_byte(address, data[i], false, send_stop && (i == (data.len() - 1))) {
			None => return None,
			_ => ()
		};
	}
	
	Some(true)
}

#[allow(dead_code)]
pub fn read_byte(address: u8, send_stop: bool) -> Option<u8> {
	set_address(address);

	free(|cs| {
		let twi0 = I2C_HANDLE.borrow(cs).borrow();
		let i2c = twi0.as_ref().unwrap();
	
		// Start Rx task
		i2c.tasks_startrx.write(|w| unsafe { w.bits(1) } );
		
		// Wait for rx event or error out
		while i2c.events_rxdready.read().bits() == i2c.events_error.read().bits() {}
		
		// If error, bail out
		if i2c.events_error.read().bits() > 0 {
			i2c.events_error.write(|w| unsafe { w.bits(0) });
			i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
			return None;
		}
		
		// Send stop before reading rxd as it could initiate another rx when read
		if send_stop {
			i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
		}
		
		// Clear out the Rx event flag
		i2c.events_rxdready.write(|w| unsafe { w.bits(0) });
		
		// Pull out byte
		Some(i2c.rxd.read().rxd().bits())
	});

	None
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(i2c: &nrf52832_pac::TWI0) {
	i2c.enable.write(|w| w.enable().disabled());
	
	i2c.pselscl.write(|w| unsafe { w.bits( I2C_LINE.scl_pin as u32) });
	i2c.pselsda.write(|w| unsafe { w.bits( I2C_LINE.sda_pin as u32) });
	i2c.frequency.write(|w| w.frequency().variant( I2C_LINE.frequency));
	
	i2c.enable.write(|w| w.enable().enabled());
}

fn set_address(address: u8) {
	static mut CURRENT_ADDRESS: u8 = 0;

	if address == unsafe { CURRENT_ADDRESS } {
		return;
	}

	free(|cs| {
		let twi0 = I2C_HANDLE.borrow(cs).borrow();
		let i2c = twi0.as_ref().unwrap();
	
		if i2c.address.read().bits() as u8 == address {
			return;
		}

		i2c.address.write(|w| unsafe { w.address().bits(address) } );
	});
	
	unsafe { CURRENT_ADDRESS = address };
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
