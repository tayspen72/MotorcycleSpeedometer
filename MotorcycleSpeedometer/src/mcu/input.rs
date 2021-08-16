//==============================================================================
// Notes
//==============================================================================
// mcu::input.rs
// Input Interrupt Monitor

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::Cell;
use cortex_m::interrupt::{free, Mutex};
use crate::{config, mcu};
use crate::mcu::gpio;
use msp432p401r_pac::interrupt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum EdgeTrigger {
	Falling,
	Rising,
}

#[allow(dead_code)]
pub struct Input {
	pub port: mcu::Port,
	pub pin: u8,
	pub pull: gpio::PinPull,
	pub edge: EdgeTrigger,
	pub callback: fn()
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub struct QueueEntry {
	pub port: mcu::Port,
	pub pin: u8,
}

//==============================================================================
// Variables
//==============================================================================
static HEAD: Mutex<Cell<u8>> =Mutex::new(Cell::new(0));
static TAIL: Mutex<Cell<u8>> =Mutex::new(Cell::new(0));
const QUEUE_LENGTH: u8 = config::INPUT_QUEUE_LENGTH;

static mut QUEUE: [QueueEntry; QUEUE_LENGTH as usize] = {
	[ QueueEntry {
		port: mcu::Port::PortDisabled,
		pin: 0,
	}; QUEUE_LENGTH as usize ]
};

static mut CALLBACK_QUEUE: [[fn(); 8]; 6] = [
	[dummy_handler; 8]; 6
];

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn configure(input: &Input){
	gpio::pin_setup(&gpio::PinConfig {
		port: input.port,
		pin: input.pin,
		direction: gpio::PinDirection::Input,
		pull: input.pull,
		state: gpio::PinState::PinHigh	// will be overridden by pin pull
	});

	gpio::interrupt_edge(input.port, input.pin, input.edge);
	gpio::interrupt_enable(input.port, input.pin);

	unsafe { 
		CALLBACK_QUEUE[input.pin as usize][input.port as usize] = input.callback;
	}
}

//==============================================================================
// Private Functions
//==============================================================================
fn dummy_handler() {
	// Empty function for the callback queue
}

//==============================================================================
// Interrupt Handler
//==============================================================================
fn interrupt_handler(port: mcu::Port) {
	// Call the GPIO interrupt handler to service the IFG register and get flags
	let pin = gpio::interrupt_handler(port);
	let tail = free(|cs| TAIL.borrow(cs).get());
	
	unsafe {
		free(|cs| TAIL.borrow(cs).set( if tail + 1 == QUEUE_LENGTH { 0 } else { tail + 1 } ));

		QUEUE[tail as usize] = QueueEntry {
			port: port,
			pin: pin,
		}
	}
}

#[interrupt]
fn PORT1_IRQ() {
	interrupt_handler(mcu::Port::Port1);
}

#[interrupt]
fn PORT2_IRQ() {
	interrupt_handler(mcu::Port::Port2);
}

#[interrupt]
fn PORT3_IRQ() {
	interrupt_handler(mcu::Port::Port3);
}

#[interrupt]
fn PORT4_IRQ() {
	interrupt_handler(mcu::Port::Port4);
}

#[interrupt]
fn PORT5_IRQ() {
	interrupt_handler(mcu::Port::Port5);
}

#[interrupt]
fn PORT6_IRQ() {
	interrupt_handler(mcu::Port::Port6);
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {
	let mut head = free(|cs| HEAD.borrow(cs).get() as usize);
	let tail = free(|cs| TAIL.borrow(cs).get() as usize);

	while head != tail {
		// For this input event, load and use callback from queue
		unsafe { 
			let callback = CALLBACK_QUEUE[QUEUE[head].pin as usize][QUEUE[head].port as usize];
			callback();

			// Increment head index to traverse queue
			head += 1;
			if head == QUEUE_LENGTH as usize {
				head = 0;
			}
		}
	}
	
	free(|cs| HEAD.borrow(cs).set(tail as u8));
}