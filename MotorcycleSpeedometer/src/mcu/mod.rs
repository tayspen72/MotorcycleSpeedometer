//==============================================================================
// Notes
//==============================================================================
// mcu::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use cortex_m;
use msp432p401r_pac;

use crate::config;

pub mod adc;
pub mod counter;
pub mod eusci;
pub mod gpio;
pub mod input;
pub mod nvic;
pub mod rtc;
pub mod systick;
pub mod timer;
pub mod wdt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub enum McuState {
	Idle
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Port{
	Port1 = 0,
	Port2 = 1,
	Port3 = 2,
	Port4 = 3,
	Port5 = 4,
	Port6 = 5,
	Port7 = 6,
	Port8 = 7,
	Port9 = 8,
	Port10 =9,
	PortJ = 10,
	PortDisabled
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub struct SystemClock{
	pub m_clk: u32,
	pub hsm_clk: u32,
	pub sm_clk: u32,
	pub a_clk: u32,
	pub b_clk: u32
}

//==============================================================================
// Variables
//==============================================================================
const HFXT_CLK_IN: gpio::PinConfig = gpio::PinConfig {
	port: config::HFXCLK_IN_PORT,
	pin: config::HFXCLK_IN_PIN,
	direction: gpio::PinDirection::Input,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const HFXT_CLK_OUT: gpio::PinConfig = gpio::PinConfig {
	port: config::HFXCLK_OUT_PORT,
	pin: config::HFXCLK_OUT_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinHigh,
};
const LFXT_CLK_IN: gpio::PinConfig = gpio::PinConfig {
	port: config::LFXCLK_IN_PORT,
	pin: config::LFXCLK_IN_PIN,
	direction: gpio::PinDirection::Input,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const LFXT_CLK_OUT: gpio::PinConfig = gpio::PinConfig {
	port: config::LFXCLK_OUT_PORT,
	pin: config::LFXCLK_OUT_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinHigh,
};

static mut SYSTEM_CLOCK: SystemClock = SystemClock {
	m_clk: 0,
	hsm_clk: 0,
	sm_clk: 0,
	a_clk: 0,
	b_clk: 0,
};

const TEMPERATURE_ADC: adc::Adc = adc::Adc {
	port: Port::PortDisabled,
	pin: 0,
	channel: adc::Channel::Temperature,
	function_select: 0,
	resolution: adc::Resolution::B14
};

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	let peripherals = msp432p401r_pac::Peripherals::take().unwrap();
	let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
	
	cortex_m::interrupt::disable();

	nvic::init(cortex_peripherals.NVIC);
	wdt::init(peripherals.WDT_A);

	// Enable all banks of SRAM and wait for SRAM_RDY to be set
	peripherals.SYSCTL.sys_sram_banken.write(|w| w.bnk7_en().set_bit());
	while peripherals.SYSCTL.sys_sram_banken.read().sram_rdy().is_sram_rdy_0() {};

	// Enable temperature sensor in reference module
	peripherals.REF_A.refctl0.modify(|_, w| w
		.reftcoff().clear_bit()
		.refvsel().refvsel_3()
		.refon().set_bit()
	);
	
	eusci::init(
		peripherals.EUSCI_A0,
		peripherals.EUSCI_A1,
		peripherals.EUSCI_A2,
		peripherals.EUSCI_A3,
		peripherals.EUSCI_B0,
		peripherals.EUSCI_B1,
		peripherals.EUSCI_B2,
		peripherals.EUSCI_B3
	);
	counter::init(
		peripherals.TIMER_A0,
		peripherals.TIMER_A1,
		peripherals.TIMER_A2,
		peripherals.TIMER_A3
	);
	gpio::init(peripherals.DIO);
	
	// These peripherals use GPIO pins
	adc::init(peripherals.ADC14);
	init_clock(peripherals.CS);

	// These peripherals rely on the core clock being stable
	systick::init(cortex_peripherals.SYST);
	timer::init(peripherals.TIMER32);
	rtc::init(peripherals.RTC_C, rtc::WakeInterval::Wake250Ms);

	init_temp_sensor();
	
	unsafe { cortex_m::interrupt::enable() };
}

#[allow(dead_code)]
pub fn get_busy() -> McuState {
	McuState::Idle
}

#[allow(dead_code)]
pub fn get_system_clock() -> SystemClock {
	unsafe { SYSTEM_CLOCK } 
}

#[allow(dead_code)]
pub fn get_temperature() -> i8 {
	// Temperature graph seems to be appx:
	//	y = 0.00104x + 0.65166V
	//	-> 
	//	temp(C) = { ADC(mV) - 650mV } / 0.00104
	let read = adc::read_ref(&TEMPERATURE_ADC, 2.5);

	((read - 0.65166) / 0.00104) as i8
}



#[allow(dead_code)]
pub fn restart() {
	cortex_m::peripheral::SCB::sys_reset();
}

//==============================================================================
// Private Functions
//==============================================================================
fn init_clock(clock: msp432p401r_pac::CS) {
	gpio::pin_setup(&HFXT_CLK_IN);
	gpio::set_pin_function_select(&HFXT_CLK_IN, 0b01);
	gpio::pin_setup(&HFXT_CLK_OUT);
	gpio::set_pin_function_select(&HFXT_CLK_OUT, 0b01);
	gpio::pin_setup(&LFXT_CLK_IN);
	gpio::set_pin_function_select(&LFXT_CLK_IN, 0b01);
	gpio::pin_setup(&LFXT_CLK_OUT);
	gpio::set_pin_function_select(&LFXT_CLK_OUT, 0b01);
	
	clock.cskey.write(|w| unsafe { w.cskey().bits(0x695A) });

	// Configure clock speeds
	clock.csctl1.write(|w| w
		// MCLK: Master Clock 48MHz
		.selm().selm_5()
		.divm().divm_0()
		// HSMCLK: Sub-Master Clock 24MHz
		.sels().sels_5()
		.divhs().divhs_1()
		// SMCLK: Low-Speed Master Clock 6MHz
		.divs().divs_4()
		// ACLK: Aux Clock 32.768 kHz
		.sela().sela_0()
		.diva().diva_0()
		// BCLK: Backup Clock 32.768 kHz
		.selb().selb_0()
	);
	
	// Enable HFXT and LFXT as external crystals
	clock.csctl2.write(|w| w
		.hfxtbypass().clear_bit()
		.hfxt_en().clear_bit()
		.hfxtfreq().hfxtfreq_6()
		.hfxtdrive().set_bit()
		
		.lfxtbypass().clear_bit()
		.lfxt_en().clear_bit()
		.lfxtdrive().lfxtdrive_0()
	);
	
	let mut status = clock.csstat.read().bits();
	
	while status & 0x1F000044 != 0x1F000044 {
		status = clock.csstat.read().bits();
	}
	
	// Lock the clock registers when finished
	clock.cskey.write(|w| unsafe { w.cskey().bits(0xFFFF) });

	unsafe {
		SYSTEM_CLOCK = SystemClock {
			m_clk: 48_000_000,
			hsm_clk: 24_000_000,
			sm_clk: 6_000_000,
			a_clk: 32_768,
			b_clk: 32_768,
		};
	}
}

fn init_temp_sensor() {
	adc::configure(&TEMPERATURE_ADC);
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {
	input::task_handler();
}
