/*
 * Camp Chef
 * Copyright (C) 2022, All rights reserved.
 *
 * config.h
 */

#ifndef CONFIG_H_
#define CONFIG_H_

//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Definitions
//==============================================================================
/* External Memory Bus */
#define EXT_A0_PORT				GPIOF
#define EXT_A0_PIN				0
#define EXT_A1_PORT				GPIOF
#define EXT_A1_PIN				1
#define EXT_A2_PORT				GPIOF
#define EXT_A2_PIN				2
#define EXT_A3_PORT				GPIOF
#define EXT_A3_PIN				3
#define EXT_A4_PORT				GPIOF
#define EXT_A4_PIN				4
#define EXT_A5_PORT				GPIOF
#define EXT_A5_PIN				5
#define EXT_A6_PORT				GPIOF
#define EXT_A6_PIN				12
#define EXT_A7_PORT				GPIOF
#define EXT_A7_PIN				13
#define EXT_A8_PORT				GPIOF
#define EXT_A8_PIN				14
#define EXT_A9_PORT				GPIOF
#define EXT_A9_PIN				15
#define EXT_A10_PORT			GPIOG
#define EXT_A10_PIN				0
#define EXT_A11_PORT			GPIOG
#define EXT_A11_PIN				1
#define EXT_A12_PORT			GPIOG
#define EXT_A12_PIN				2
#define EXT_A13_PORT			GPIOG
#define EXT_A13_PIN				3
#define EXT_A14_PORT			GPIOG
#define EXT_A14_PIN				4
#define EXT_A15_PORT			GPIOG
#define EXT_A15_PIN				5
#define EXT_A16_PORT			GPIOD
#define EXT_A16_PIN				11
#define EXT_A17_PORT			GPIOD
#define EXT_A17_PIN				12
#define EXT_A18_PORT			GPIOD
#define EXT_A18_PIN				13
#define EXT_D0_PORT				GPIOD
#define EXT_D0_PIN				14
#define EXT_D1_PORT				GPIOD
#define EXT_D1_PIN				15
#define EXT_D2_PORT				GPIOD
#define EXT_D2_PIN				0
#define EXT_D3_PORT				GPIOD
#define EXT_D3_PIN				1
#define EXT_D4_PORT				GPIOE
#define EXT_D4_PIN				7
#define EXT_D5_PORT				GPIOE
#define EXT_D5_PIN				8
#define EXT_D6_PORT				GPIOE
#define EXT_D6_PIN				9
#define EXT_D7_PORT				GPIOE
#define EXT_D7_PIN				10
#define EXT_D8_PORT				GPIOE
#define EXT_D8_PIN				11
#define EXT_D9_PORT				GPIOE
#define EXT_D9_PIN				12
#define EXT_D10_PORT			GPIOE
#define EXT_D10_PIN				13
#define EXT_D11_PORT			GPIOE
#define EXT_D11_PIN				14
#define EXT_D12_PORT			GPIOE
#define EXT_D12_PIN				15
#define EXT_D13_PORT			GPIOD
#define EXT_D13_PIN				8
#define EXT_D14_PORT			GPIOD
#define EXT_D14_PIN				9
#define EXT_D15_PORT			GPIOD
#define EXT_D15_PIN				10
#define EXT_NBL0_PORT			GPIOE
#define EXT_NBL0_PIN			0
#define EXT_NBL1_PORT			GPIOE
#define EXT_NBL1_PIN			1
#define EXT_NE1_PORT			GPIOD
#define EXT_NE1_PIN				7
#define EXT_NE2_PORT			GPIOG
#define EXT_NE2_PIN				10
#define EXT_NOE_PORT			GPIOD
#define EXT_NOE_PIN				4
#define EXT_NWE_PORT			GPIOD
#define EXT_NWE_PIN				5

/* Tft Lcd */
#define LCD_RESET_PORT			GPIOD
#define LCD_RESET_PIN			2
#define LCD_BUSY_PORT			GPIOC
#define LCD_BUSY_PIN			5

#define LCD_BL_PORT				GPIOA
#define LCD_BL_PIN				1
#define LCD_BL_PWM_CHANNEL		??

/* Led */
#define LED_PORT				GPIOE
#define LED_PIN					6
#define LED_CLOCK_ENABLE		__HAL_RCC_GPIOE_CLK_ENABLE

//==============================================================================
// Includes
//==============================================================================


//==============================================================================
// Enumerations and Structures
//==============================================================================


//==============================================================================
// Function Prototypes
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Macro Functions
//==============================================================================


#endif /* config.h */
