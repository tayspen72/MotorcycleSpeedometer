//==============================================================================
// Notes
//==============================================================================
// drivers::lcd::st7789.rs
// Register definitions for the ST7789 LCD driver 

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
/** Commands **/
//From page 156 of datasheet
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum COMMAND { 
	SW_RESET							= 0x01,
	SLEEP_IN							= 0x10,	/* Enter minimum power consumption mode */
	SLEEP_OUT							= 0x11,	/* Exit minimum power consumption mode */
	PARTIAL_MODE						= 0x12,
	NORMAL_MODE							= 0x13,
	DISPLAY_INVERSION_OFF				= 0x20,
	DISPLAY_INVERSION_ON				= 0x21,	/* Color inversion mode */
	GAMMA								= 0x26,
	DISPLAY_OFF							= 0x28,
	DISPLAY_ON							= 0x29,
	COLUMN_ADDRESS						= 0x2A,
	ROW_ADDRESS							= 0x2B,
	MEMORY_WRITE						= 0x2C,
	PARTIAL_AREA						= 0x30,
	VERTICAL_SCROLLING_DEFINITION		= 0x33,
	TEARING_EFFECT_LINE_ON				= 0x35,
	MEMORY_DATA_ACCESS_CONTROL			= 0x36,
	VERTICAL_SCROLLING_START_ADDRESS	= 0x37,
	IDLE_MODE_OFF						= 0x38,
	IDLE_MODE_ON						= 0x39,
	INTERFACE_PIXEL_FORMAT				= 0x3A,
	MEMORY_WRITE_CONTINUE				= 0x3C,
	TEAR_SCANLINE						= 0x44,
	DISPLAY_BRIGHTNESS					= 0x51,
	CTRL_DISPLAY						= 0x53,
	ADAPTIVE_BRIGHTNESS_CTRL			= 0x55,
	CABC_MINIMUM_BRIGHTNESS				= 0x5E,
	RAM_CONTROL							= 0xB0,
	RGB_INTERFACE_CONTROL				= 0xB1,
	PORCH_SETTING						= 0xB2,
	FRAME_RATE_CONTROL					= 0xB3,
	PARTIAL_CONTROL						= 0xB5,
	GATE_CONTROL						= 0xB7,
	GATE_ON_TIMING_ADJUSTMENT			= 0xB8,
	DIGITAL_GAMMA_ENABLE				= 0xBA,
	VCOM_SETTING						= 0xB9,
	POWER_SAVING_MODE					= 0xBC,
	DISPLAY_OFF_POWER_SAVE				= 0xBD,
	LCM_CONTROL							= 0xC0,
	ID_SETTING							= 0xC1,
	VDV_VRH_CMD_ENABLE					= 0xC2,
	VRH_SET								= 0xC3,
	VDV_SET								= 0xC4,
	VCM_OFFSET_SET						= 0xC5,
	FRAME_RATE_CONTROL_2				= 0xC6,
	CABC_CONTROL						= 0xC7,
	REGISTER_VALUE_SELECTION_1			= 0xC8,
	REGISTER_VALUE_SELECTION_2			= 0xCA,
	PWM_FREQUENCY_SELECTION				= 0xCC,
	POWER_CONTROL_1						= 0xD0,
	VAP_VAN_SIGNAL_OUT					= 0xD2,
	COMMAND_2_ENABLE					= 0xDF,
	POSITIVE_VOLTAGE_GAMMA_CONTROL		= 0xE0,
	NEGATIVE_VOLTAGE_GAMMA_CONTROL		= 0xE1,
	DIGITAL_GAMMA_LOOKUP_RED			= 0xE2,
	DIGITAL_GAMMA_LOOKUP_BLUE			= 0xE3,
	GATE_CONTROL_2						= 0xE4,
	SPI2_ENABLE							= 0xE7,
	POWER_CONTROL_2						= 0xE8,
	EQUALIZE_TIME_CONTROL				= 0xE9,
	PROGRAM_MODE_CONTROL				= 0xEC,
	PROGRAM_MODE_ENABLE					= 0xFA,
	NVM_SETTING							= 0xFC,
	PROGRAM_ACTION						= 0xFE
}
