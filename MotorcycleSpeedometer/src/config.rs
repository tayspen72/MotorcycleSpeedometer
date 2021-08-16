//==============================================================================
// Notes
//==============================================================================
// config.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::mcu;
use crate::mcu::{adc, counter, eusci};

//==============================================================================
// ADC
//==============================================================================


//==============================================================================
// Button
//==============================================================================


//==============================================================================
// Clock
//==============================================================================
#[allow(dead_code)] pub const LFXCLK_IN_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const LFXCLK_IN_PIN: u8 = 0;
#[allow(dead_code)] pub const LFXCLK_OUT_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const LFXCLK_OUT_PIN: u8 = 1;
#[allow(dead_code)] pub const HFXCLK_IN_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const HFXCLK_IN_PIN: u8 = 2;
#[allow(dead_code)] pub const HFXCLK_OUT_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const HFXCLK_OUT_PIN: u8 = 3;

//==============================================================================
// Counter
//==============================================================================
#[allow(dead_code)] pub const COUNTER_TACLK_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const COUNTER_TACLK_PIN: u8 = 2;
#[allow(dead_code)] pub const COUNTER_TACLK: counter::TaClk = counter::TaClk::A2;
#[allow(dead_code)] pub const COUNTER_FUNCTION_SELECT: u8 = 0b10;

//==============================================================================
// Debug
//==============================================================================


//==============================================================================
// Flash
//==============================================================================


//==============================================================================
// Fuel ADC
//==============================================================================
#[allow(dead_code)] pub const FUEL_ADC_PORT: mcu::Port = mcu::Port::Port6;
#[allow(dead_code)] pub const FUEL_ADC_PIN: u8 = 1;
#[allow(dead_code)] pub const FUEL_ADC_CHANNEL: adc::Channel = adc::Channel::A14;
#[allow(dead_code)] pub const FUEL_ADC_FUNCTION_SELECT: u8 = 0b11;

//==============================================================================
// I2C
//==============================================================================
#[allow(dead_code)] pub const I2C_SDA_PORT: mcu::Port = mcu::Port::Port1;
#[allow(dead_code)] pub const I2C_SDA_PIN: u8 = 6;
#[allow(dead_code)] pub const I2C_SCL_PORT: mcu::Port = mcu::Port::Port1;
#[allow(dead_code)] pub const I2C_SCL_PIN: u8 = 7;
#[allow(dead_code)] pub const I2C_EUSCI: eusci::EUSCI = eusci::EUSCI::B0;
#[allow(dead_code)] pub const I2C_ADDRESS: u8 = 0x70;
#[allow(dead_code)] pub const I2C_SPEED: u32 = 400_000;
#[allow(dead_code)] pub const I2C_FUNCTION_SELECT: u8 = 0b01;

//==============================================================================
// Input
//==============================================================================
#[allow(dead_code)] pub const INPUT_QUEUE_LENGTH: u8 = 16;

//==============================================================================
// LCD
//==============================================================================0
#[allow(dead_code)] pub const LCD_RS_PORT: mcu::Port = mcu::Port::Port3;
#[allow(dead_code)] pub const LCD_RS_PIN: u8 = 0;
#[allow(dead_code)] pub const LCD_RW_PORT: mcu::Port = mcu::Port::Port5;
#[allow(dead_code)] pub const LCD_RW_PIN: u8 = 6;
#[allow(dead_code)] pub const LCD_EN_PORT: mcu::Port = mcu::Port::Port5;
#[allow(dead_code)] pub const LCD_EN_PIN: u8 = 7;
#[allow(dead_code)] pub const LCD_D0_PORT: mcu::Port = mcu::Port::Port6;
#[allow(dead_code)] pub const LCD_D0_PIN: u8 = 6;
#[allow(dead_code)] pub const LCD_D1_PORT: mcu::Port = mcu::Port::Port6;
#[allow(dead_code)] pub const LCD_D1_PIN: u8 = 7;
#[allow(dead_code)] pub const LCD_D2_PORT: mcu::Port = mcu::Port::Port2;
#[allow(dead_code)] pub const LCD_D2_PIN: u8 = 3;
#[allow(dead_code)] pub const LCD_D3_PORT: mcu::Port = mcu::Port::Port5;
#[allow(dead_code)] pub const LCD_D3_PIN: u8 = 1;
#[allow(dead_code)] pub const LCD_D4_PORT: mcu::Port = mcu::Port::Port5;
#[allow(dead_code)] pub const LCD_D4_PIN: u8 = 0;
#[allow(dead_code)] pub const LCD_D5_PORT: mcu::Port = mcu::Port::Port3;
#[allow(dead_code)] pub const LCD_D5_PIN: u8 = 5;
#[allow(dead_code)] pub const LCD_D6_PORT: mcu::Port = mcu::Port::Port5;
#[allow(dead_code)] pub const LCD_D6_PIN: u8 = 2;
#[allow(dead_code)] pub const LCD_D7_PORT: mcu::Port = mcu::Port::Port3;
#[allow(dead_code)] pub const LCD_D7_PIN: u8 = 7;
#[allow(dead_code)] pub const LCD_BLA_PORT: mcu::Port = mcu::Port::Port3;
#[allow(dead_code)] pub const LCD_BLA_PIN: u8 = 6;

//==============================================================================
// LED
//==============================================================================


//==============================================================================
// QUADALPHA
//==============================================================================


//==============================================================================
// SPI
//==============================================================================


//==============================================================================
// Temperature Sensor
//==============================================================================
#[allow(dead_code)] pub const TEMPERATURE_ADC_PORT: mcu::Port = mcu::Port::PortDisabled;
#[allow(dead_code)] pub const TEMPERATURE_ADC_PIN: u8 = 0;
#[allow(dead_code)] pub const TEMPERATURE_ADC_CHANNEL: adc::Channel = adc::Channel::Temperature;
#[allow(dead_code)] pub const TEMPERATURE_ADC_SIGNAL: u8 = 0;
#[allow(dead_code)] pub const TEMPERATURE_ADC_FUNCTION_SELECT: u8 = 0b11;

//==============================================================================
// Uart
//==============================================================================
