//==============================================================================
// Notes
//==============================================================================
// mcu::adc.rs
// ADC Driver

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::mcu;
use crate::mcu::gpio;
use msp432p401r_pac;
use msp432p401r_pac::interrupt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Channel{
	A0 = 0,
	A1 = 1,
	A2 = 2,
	A3 = 3,
	A4 = 4,
	A5 = 5,
	A6 = 6,
	A7 = 7,
	A8 = 8,
	A9 = 9,
	A10 = 10,
	A11 = 11,
	A12 = 12,
	A13 = 13,
	A14 = 14,
	A15 = 15,
	A16 = 16,
	A17 = 17,
	A18 = 18,
	A19 = 19,
	A20 = 20,
	A21 = 21,
	Temperature = 22,
	Battery = 23
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
enum InternalChannelMap{
	ChMap0,
	ChMap1,
	ChMap2,
	ChMap3,
	TcMap,
	BatMap,
	None
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum TriggerSource{
	Software = 0,
	Ta0c1 = 1,
	Ta0c2 = 2,
	Ta1c1 = 3,
	Ta1c2 = 4,
	Ta2c1 = 5,
	Ta2c2 = 6,
	Ta3c1 = 7,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Resolution{
	B8 = 0xFF,
	B10 = 0x3FF,
	B12 = 0xFFF,
	B14	= 0x3FFF
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub struct Adc{
	pub port: mcu::Port,
	pub pin: u8,
	pub channel: Channel,
	pub function_select: u8,
	pub resolution: Resolution,
}

//==============================================================================
// Variables
//==============================================================================
static ADC_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::ADC14>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;



//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(adc: msp432p401r_pac::ADC14) {
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| ADC_HANDLE.borrow(cs).replace(Some(adc)));

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn configure(adc: &Adc) {
	gpio::pin_setup(&gpio::PinConfig {
		port: adc.port,
		pin: adc.pin,
		direction: gpio::PinDirection::Input,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow
	});
	gpio::set_pin_function_select(
		&gpio::PinConfig {
			port: adc.port,
			pin: adc.pin,
			direction: gpio::PinDirection::Input,
			pull: gpio::PinPull::PullDisabled,
			state: gpio::PinState::PinLow
		},
		adc.function_select
	);

	free(|cs| {
		if let Some(ref mut adc14) = ADC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Take these out later
			adc14.adc14ctl0.write(|w| w
				.adc14pdiv().adc14pdiv_0()
				.adc14shs().bits(TriggerSource::Software as u8)
				.adc14shp().set_bit()
				.adc14div().adc14div_0()
				.adc14ssel().adc14ssel_3()
				.adc14conseq().adc14conseq_0()
				.adc14sht0().adc14sht0_7()
				.adc14sht1().adc14sht1_7()
				.adc14msc().clear_bit()
				.adc14on().set_bit()
				.adc14enc().clear_bit()
			);

			adc14.adc14ier0.modify(|_, w| w.adc14ie0().set_bit());
		}
	});

	// unsafe {
	// 	mcu::nvic_enable(24);
	// 	cortex_m::interrupt::enable();
	// }
}

#[allow(dead_code)]
pub fn read(adc: &Adc) -> u16 {
	free(|cs| {
		if let Some(ref mut adc14) = ADC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Assign object config
			let channel_map = get_internal_channel(adc.channel);
			adc14.adc14ctl1.write(|w| unsafe { w
				.adc14ch3map().bit(channel_map == InternalChannelMap::ChMap3)
				.adc14ch2map().bit(channel_map == InternalChannelMap::ChMap2)
				.adc14ch1map().bit(channel_map == InternalChannelMap::ChMap1)
				.adc14ch0map().bit(channel_map == InternalChannelMap::ChMap0)
				.adc14tcmap().bit(channel_map == InternalChannelMap::TcMap)
				.adc14batmap().bit(channel_map == InternalChannelMap::BatMap)
				.adc14cstartadd().bits(0)
				.adc14res().bits(adc.resolution as u8)
				.adc14df().clear_bit()
				.adc14refburst().set_bit()
				.adc14pwrmd().adc14pwrmd_0()
			} );

			adc14.adc14mctl[0].write(|w| w
				.adc14dif().clear_bit()
				.adc14vrsel().adc14vrsel_1()
				.adc14eos().set_bit()
				.adc14inch().bits(adc.channel as u8)
			);

			// Clear the conversion flag before starting
			adc14.adc14clrifgr0.write(|w| w.clradc14ifg0().set_bit());

			// Set software trigger to start read
			adc14.adc14ctl0.modify(|_, w| w
				.adc14enc().set_bit()
				.adc14sc().set_bit()
			);
			
			// Wait for config to finish
			while adc14.adc14ifgr0.read().adc14ifg0().is_adc14ifg0_0() {}

			// Clear flag when finished
			adc14.adc14clrifgr0.write(|w| w.clradc14ifg0().set_bit());
			adc14.adc14ctl0.modify(|_, w| w
				.adc14enc().clear_bit()
			);

			adc14.adc14mem[0].read().conversion_results().bits()
		}
		else {
			0
		}
	})
}

#[allow(dead_code)]
pub fn read_ref(adc: &Adc, v_ref: f32) -> f32 {
	let read = read(adc);
	(read as f32) * v_ref / (adc.resolution as u16 as f32) 
}

//==============================================================================
// Private Functions
//==============================================================================
fn get_internal_channel(channel: Channel) -> InternalChannelMap {
	match channel {
		Channel::A18 => InternalChannelMap::ChMap3,
		Channel::A19 => InternalChannelMap::ChMap2,
		Channel::A20 => InternalChannelMap::ChMap1,
		Channel::A21 => InternalChannelMap::ChMap0,
		Channel::Temperature => InternalChannelMap::TcMap,
		Channel::Battery => InternalChannelMap::BatMap,
		_ => InternalChannelMap::None
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
fn ADC14_IRQ () {
	let _raw = free(|cs| {
		if let Some(ref mut adc14) = ADC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			let read = adc14.adc14ifgr0.read().bits();
			adc14.adc14clrifgr0.write(|w| unsafe { w.bits(read) });
			
			adc14.adc14mem[0].read().bits()
		}
		else {
			0
		}
	});
}

//==============================================================================
// Task Handler
//==============================================================================
