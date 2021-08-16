//==============================================================================
// Notes
//==============================================================================
// mcu::gpio.rs
// Basic control over gpio pins

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::mcu;
use crate::mcu::{input, nvic};
use msp432p401r_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinDirection{
	Input = 0,
	Output = 1
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinPull{
	PullUp = 1,
	PullDown = 0,
	PullDisabled = 2
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinState{
	PinLow = 0,
	PinHigh = 1
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PinConfig{
	pub port: mcu::Port,
	pub pin: u8,
	pub direction: PinDirection,
	pub pull: PinPull,
	pub state: PinState,
}

//==============================================================================
// Variables
//==============================================================================
static DIO_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::DIO>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(dio: msp432p401r_pac::DIO){
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| DIO_HANDLE.borrow(cs).replace(Some(dio)));

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn get_pin_state(port: mcu::Port, pin: u8) -> PinState {
	unsafe { if !INITIALIZED {
		return PinState::PinLow;
	}}
	
	let read = free(|cs|
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			match port {
				mcu::Port::Port1  => dio.pain.read().p1in().bits(),
				mcu::Port::Port2  => dio.pain.read().p2in().bits(),
				mcu::Port::Port3  => dio.pbin.read().p3in().bits(),
				mcu::Port::Port4  => dio.pbin.read().p4in().bits(),
				mcu::Port::Port5  => dio.pcin.read().p5in().bits(),
				mcu::Port::Port6  => dio.pcin.read().p6in().bits(),
				mcu::Port::Port7  => dio.pdin.read().p7in().bits(),
				mcu::Port::Port8  => dio.pdin.read().p8in().bits(),
				mcu::Port::Port9  => dio.pein.read().p9in().bits(),
				mcu::Port::Port10 => dio.pein.read().p10in().bits(),
				mcu::Port::PortJ  => (dio.pjin.read().pjin().bits() & 0xFF) as u8,
				mcu::Port::PortDisabled => 0,
			}
		}
		else {
			0
		}
	);
	
	match read & (1 << pin) {
		0 => PinState::PinLow,
		_ => PinState::PinHigh
	}
}

#[allow(dead_code)]
pub fn interrupt_edge(port: mcu::Port, pin: u8, edge: input::EdgeTrigger){
	free(|cs| {
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			let val = if edge == input::EdgeTrigger::Falling { 1 } else { 0 };
			match port {
				mcu::Port::Port1 =>  dio.paies.modify(|r, w| unsafe { w.p1ies().bits(r.p1ies().bits() & !(1 << pin) | (val << pin)) }),
				mcu::Port::Port2 =>  dio.paies.modify(|r, w| unsafe { w.p2ies().bits(r.p2ies().bits() & !(1 << pin) | (val << pin)) }),
				mcu::Port::Port3 =>  dio.pbies.modify(|r, w| unsafe { w.p3ies().bits(r.p3ies().bits() & !(1 << pin) | (val << pin)) }),
				mcu::Port::Port4 =>  dio.pbies.modify(|r, w| unsafe { w.p4ies().bits(r.p4ies().bits() & !(1 << pin) | (val << pin)) }),
				mcu::Port::Port5 =>  dio.pcies.modify(|r, w| unsafe { w.p5ies().bits(r.p5ies().bits() & !(1 << pin) | (val << pin)) }),
				mcu::Port::Port6 =>  dio.pcies.modify(|r, w| unsafe { w.p6ies().bits(r.p6ies().bits() & !(1 << pin) | (val << pin)) }),
				mcu::Port::Port7 => (),
				mcu::Port::Port8 => (),
				mcu::Port::Port9 => (),
				mcu::Port::Port10 => (),
				mcu::Port::PortJ => (),
				mcu::Port::PortDisabled => (),
			}
		}
	});
}


#[allow(dead_code)]
pub fn interrupt_enable(port: mcu::Port, pin: u8) {
	free(|cs|
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			match port {
				mcu::Port::Port1  => dio.paie.modify(|r, w| unsafe { w.p1ie().bits(r.p1ie().bits() | (1 << pin)) }),
				mcu::Port::Port2  => dio.paie.modify(|r, w| unsafe { w.p2ie().bits(r.p2ie().bits() | (1 << pin)) }),
				mcu::Port::Port3  => dio.pbie.modify(|r, w| unsafe { w.p3ie().bits(r.p3ie().bits() | (1 << pin)) }),
				mcu::Port::Port4  => dio.pbie.modify(|r, w| unsafe { w.p4ie().bits(r.p4ie().bits() | (1 << pin)) }),
				mcu::Port::Port5  => dio.pcie.modify(|r, w| unsafe { w.p5ie().bits(r.p5ie().bits() | (1 << pin)) }),
				mcu::Port::Port6  => dio.pcie.modify(|r, w| unsafe { w.p6ie().bits(r.p6ie().bits() | (1 << pin)) }),
				mcu::Port::Port7  => (),
				mcu::Port::Port8  => (),
				mcu::Port::Port9  => (),
				mcu::Port::Port10 => (),
				mcu::Port::PortJ => (),
				mcu::Port::PortDisabled => (),
			}
		}
	);

	if port == mcu::Port::Port1 || port == mcu::Port::Port2 || port == mcu::Port::Port3 
		|| port == mcu::Port::Port4 || port == mcu::Port::Port5 || port == mcu::Port::Port6 {
			nvic::clear_pending(port as u8 + 35);
			nvic::enable(port as u8 + 35);
		}
}

#[allow(dead_code)]
pub fn pin_disable(_config: &PinConfig) {
	unsafe { if !INITIALIZED {
		return;
	}}
}

#[allow(dead_code)]
pub fn pin_setup(config: &PinConfig){
	unsafe { if !INITIALIZED {
		return;
	}}	
	
	let mut state: PinState = config.state;
	
	free(|cs| {
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if let PinDirection::Input = config.direction {
				// Set pin direction
				match config.port {
					mcu::Port::Port1 =>  dio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port2 =>  dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port3 =>  dio.pbdir.modify(|r, w| unsafe { w.p3dir().bits(r.p3dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port4 =>  dio.pbdir.modify(|r, w| unsafe { w.p4dir().bits(r.p4dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port5 =>  dio.pcdir.modify(|r, w| unsafe { w.p5dir().bits(r.p5dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port6 =>  dio.pcdir.modify(|r, w| unsafe { w.p6dir().bits(r.p6dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port7 =>  dio.pddir.modify(|r, w| unsafe { w.p7dir().bits(r.p7dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port8 =>  dio.pddir.modify(|r, w| unsafe { w.p8dir().bits(r.p8dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port9 =>  dio.pedir.modify(|r, w| unsafe { w.p9dir().bits(r.p9dir().bits()   & !(1 << config.pin)) }),
					mcu::Port::Port10 => dio.pedir.modify(|r, w| unsafe { w.p10dir().bits(r.p10dir().bits() & !(1 << config.pin)) }),
					mcu::Port::PortJ =>  dio.pjdir.modify(|r, w| unsafe { w.pjdir().bits(r.pjdir().bits()   & !(1 << config.pin)) }),
					mcu::Port::PortDisabled => (),
				}

				// Set pin pull as needed
				let pull = config.pull as u8;
				match config.port {
					mcu::Port::Port1 =>  dio.paren.modify(|r, w| unsafe { w.p1ren().bits(r.p1ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port2 =>  dio.paren.modify(|r, w| unsafe { w.p2ren().bits(r.p2ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port3 =>  dio.pbren.modify(|r, w| unsafe { w.p3ren().bits(r.p3ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port4 =>  dio.pbren.modify(|r, w| unsafe { w.p4ren().bits(r.p4ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port5 =>  dio.pcren.modify(|r, w| unsafe { w.p5ren().bits(r.p5ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port6 =>  dio.pcren.modify(|r, w| unsafe { w.p6ren().bits(r.p6ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port7 =>  dio.pdren.modify(|r, w| unsafe { w.p7ren().bits(r.p7ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port8 =>  dio.pdren.modify(|r, w| unsafe { w.p8ren().bits(r.p8ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port9 =>  dio.peren.modify(|r, w| unsafe { w.p9ren().bits(r.p9ren().bits()   & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::Port10 => dio.peren.modify(|r, w| unsafe { w.p10ren().bits(r.p10ren().bits() & !(1 << config.pin) | (pull << config.pin)) }),
					mcu::Port::PortJ =>  dio.pjren.modify(|r, w| unsafe { w.pjren().bits(r.pjren().bits()   & !(1 << config.pin) | ((pull as u16) << config.pin)) }),
					mcu::Port::PortDisabled => (),
				}

				// Set the pull state based on the otuput register value
				state = if let PinPull::PullUp = config.pull { PinState::PinHigh } else { PinState::PinLow };
			}
			else {
				// Set pin direction
				match config.port {
					mcu::Port::Port1 =>  dio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port2 =>  dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port3 =>  dio.pbdir.modify(|r, w| unsafe { w.p3dir().bits(r.p3dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port4 =>  dio.pbdir.modify(|r, w| unsafe { w.p4dir().bits(r.p4dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port5 =>  dio.pcdir.modify(|r, w| unsafe { w.p5dir().bits(r.p5dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port6 =>  dio.pcdir.modify(|r, w| unsafe { w.p6dir().bits(r.p6dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port7 =>  dio.pddir.modify(|r, w| unsafe { w.p7dir().bits(r.p7dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port8 =>  dio.pddir.modify(|r, w| unsafe { w.p8dir().bits(r.p8dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port9 =>  dio.pedir.modify(|r, w| unsafe { w.p9dir().bits(r.p9dir().bits()   | (1 << config.pin)) }),
					mcu::Port::Port10 => dio.pedir.modify(|r, w| unsafe { w.p10dir().bits(r.p10dir().bits() | (1 << config.pin)) }),
					mcu::Port::PortJ =>  dio.pjdir.modify(|r, w| unsafe { w.pjdir().bits(r.pjdir().bits()   | (1 << config.pin)) }),
					mcu::Port::PortDisabled => (),
				}
			}
		}
	});
	
	// Set the pin state after this critical section is left
	set_pin_state(config.port, config.pin, state);
	
	set_pin_function_select(config, 0);
}

#[allow(dead_code)]
pub fn set_pin_state(port: mcu::Port, pin: u8, state: PinState){
	unsafe { if !INITIALIZED {
		return;
	}}
	
	free(|cs| {
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			let out = state as u8;
			match port {
				mcu::Port::Port1 =>  dio.paout.modify(|r, w| unsafe { w.p1out().bits(r.p1out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port2 =>  dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port3 =>  dio.pbout.modify(|r, w| unsafe { w.p3out().bits(r.p3out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port4 =>  dio.pbout.modify(|r, w| unsafe { w.p4out().bits(r.p4out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port5 =>  dio.pcout.modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port6 =>  dio.pcout.modify(|r, w| unsafe { w.p6out().bits(r.p6out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port7 =>  dio.pdout.modify(|r, w| unsafe { w.p7out().bits(r.p7out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port8 =>  dio.pdout.modify(|r, w| unsafe { w.p8out().bits(r.p8out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port9 =>  dio.peout.modify(|r, w| unsafe { w.p9out().bits(r.p9out().bits()   & !(1 << pin) | (out << pin)) }),
				mcu::Port::Port10 => dio.peout.modify(|r, w| unsafe { w.p10out().bits(r.p10out().bits() & !(1 << pin) | (out << pin)) }),
				mcu::Port::PortJ =>  dio.pjout.modify(|r, w| unsafe { w.pjout().bits(r.pjout().bits()   & !(1 << pin) | ((out as u16) << pin)) }),
				mcu::Port::PortDisabled => (),
			}
		}
	});
}

#[allow(dead_code)]
pub fn set_pin_function_select(config: &PinConfig, function: u8){
	unsafe { if !INITIALIZED {
		return;
	}}

	free(|cs| {
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			let sel0 = if (function & 0x1) > 0 { 1 } else { 0 };
			let sel1 = if (function & 0x2) > 0 { 1 } else { 0 };

			match config.port {
				mcu::Port::Port1 => {
					dio.pasel0.modify(|r, w| unsafe { w.p1sel0().bits(r.p1sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pasel1.modify(|r, w| unsafe { w.p1sel1().bits(r.p1sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port2 => {
					dio.pasel0.modify(|r, w| unsafe { w.p2sel0().bits(r.p2sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pasel1.modify(|r, w| unsafe { w.p2sel1().bits(r.p2sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port3 => {
					dio.pbsel0.modify(|r, w| unsafe { w.p3sel0().bits(r.p3sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pbsel1.modify(|r, w| unsafe { w.p3sel1().bits(r.p3sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port4 => {
					dio.pbsel0.modify(|r, w| unsafe { w.p4sel0().bits(r.p4sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pbsel1.modify(|r, w| unsafe { w.p4sel1().bits(r.p4sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port5 => {
					dio.pcsel0.modify(|r, w| unsafe { w.p5sel0().bits(r.p5sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pcsel1.modify(|r, w| unsafe { w.p5sel1().bits(r.p5sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port6 => {
					dio.pcsel0.modify(|r, w| unsafe { w.p6sel0().bits(r.p6sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pcsel1.modify(|r, w| unsafe { w.p6sel1().bits(r.p6sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port7 => {
					dio.pdsel0.modify(|r, w| unsafe { w.p7sel0().bits(r.p7sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pdsel1.modify(|r, w| unsafe { w.p7sel1().bits(r.p7sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port8 => {
					dio.pdsel0.modify(|r, w| unsafe { w.p8sel0().bits(r.p8sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pdsel1.modify(|r, w| unsafe { w.p8sel1().bits(r.p8sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port9 => {
					dio.pesel0.modify(|r, w| unsafe { w.p9sel0().bits(r.p9sel0().bits()   & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pesel1.modify(|r, w| unsafe { w.p9sel1().bits(r.p9sel1().bits()   & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::Port10 => {
					dio.pesel0.modify(|r, w| unsafe { w.p10sel0().bits(r.p10sel0().bits() & !(1 << config.pin) | (sel0 << config.pin)) });
					dio.pesel1.modify(|r, w| unsafe { w.p10sel1().bits(r.p10sel1().bits() & !(1 << config.pin) | (sel1 << config.pin)) });
				},
				mcu::Port::PortJ => {
					dio.pjsel0.modify(|r, w| unsafe { w.pjsel0().bits(r.pjsel0().bits()   & !(1 << config.pin) | ((sel0 as u16 ) << config.pin)) });
					dio.pjsel1.modify(|r, w| unsafe { w.pjsel1().bits(r.pjsel1().bits()   & !(1 << config.pin) | ((sel1 as u16 ) << config.pin)) });
				},
				mcu::Port::PortDisabled => (),
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
pub fn interrupt_handler(port: mcu::Port) -> u8 {
	let mut flags: u8 = 0;

	free(|cs|
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			match port {
				mcu::Port::Port1  => {
					flags = dio.paifg.read().p1ifg().bits();
					dio.paifg.write(|w| unsafe { w.p1ifg().bits(0x00) });
				},
				mcu::Port::Port2  => {
					flags = dio.paifg.read().p2ifg().bits();
					dio.paifg.write(|w| unsafe { w.p2ifg().bits(0x00) });
				},
				mcu::Port::Port3  => {
					flags = dio.pbifg.read().p3ifg().bits();
					dio.pbifg.write(|w| unsafe { w.p3ifg().bits(0x00) });
				},
				mcu::Port::Port4  => {
					flags = dio.pbifg.read().p4ifg().bits();
					dio.pbifg.write(|w| unsafe { w.p4ifg().bits(0x00) });
				},
				mcu::Port::Port5  => {
					flags = dio.pcifg.read().p5ifg().bits();
					dio.pcifg.write(|w| unsafe { w.p5ifg().bits(0x00) });
				},
				mcu::Port::Port6  => {
					flags = dio.pcifg.read().p6ifg().bits();
					dio.pcifg.write(|w| unsafe { w.p6ifg().bits(0x00) });
				},
				mcu::Port::Port7  => (),
				mcu::Port::Port8  => (),
				mcu::Port::Port9  => (),
				mcu::Port::Port10 => (),
				mcu::Port::PortJ => (),
				mcu::Port::PortDisabled => (),
			}
		}
	);
	let mut pin: u8 = 0;
	while flags & 0x1 == 0 {
		flags >>= 1;
		pin += 1;
	}
	
	pin
}

//==============================================================================
// Task Handler
//==============================================================================
