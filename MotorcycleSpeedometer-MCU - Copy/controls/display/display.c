/*
 * Camp Chef
 * Copyright (C) 2022, All rights reserved.
 *
 * controls
 *   display.c
 */

//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Includes
//==============================================================================
#include "config.h"
#include "display.h"

// FreeRTOS
#include "FreeRTOS.h"
#include "task.h"

// LVGL: Lightweight Versatile Graphics Library
//#include "lvgl.h"

#include "stm32f1xx_hal_dma.h"
#include "stm32f1xx_hal_gpio.h"
#include "stm32f1xx_hal_gpio_ex.h"
#include "stm32f1xx_hal_nor.h"
#include "stm32f1xx_hal_rcc.h"
#include "stm32f1xx_hal_sram.h"

//==============================================================================
// Definitions
//==============================================================================
#define DISPLAY_ID						0x00009488
#define LCD_WRITE_COMMAND(command)		*((uint16_t*) 0x60000000) = command
#define LCD_WRITE_DATA(data)			*((uint16_t*) 0x60000002) = data

//==============================================================================
// Enumerations and Structures
//==============================================================================


//==============================================================================
// Function Prototypes
//==============================================================================
// Base
static void display_init(void);
// Hal
static void display_halConfigure(void);
static void display_halInitDma(void);
static void display_halInitGpio(void);
static void display_halInitExternalBus(void);
// static uint32_t display_halGetId(void);
// static uint16_t display_halReadData(void);
static void display_halWriteBlock(uint16_t xStart, uint16_t xEnd, uint16_t yStart, uint16_t yEnd);
static inline void display_halWriteCommand(uint16_t command);
static inline void display_halWriteData(uint16_t data);
// LVGL
static void display_lvglInit(void);
static void display_lvglFlushCallback(struct _lv_disp_drv_t* displayDriver, const lv_area_t* area, lv_color_t* color);
static void display_lvglFlushCompleteCallback(DMA_HandleTypeDef *dmaHandle);
static void display_lvglWaitCallback(struct _lv_disp_drv_t* displayDriver);

//==============================================================================
// Variables
//==============================================================================
static DMA_HandleTypeDef _dmaHandle;

static lv_disp_drv_t _displayDriver;

//==============================================================================
// Public Functions
//==============================================================================
void Display_HalReset(void) {
	#if defined(GD32F10X_HD) || defined(GD32F10X_XD)
		gpio_bit_reset(LCD_RESET_PORT, 1 << LCD_RESET_PIN);
	#elif defined(STM32F10X_HD) || defined(STM32F10X_XL)
		GPIO_PinWrite(LCD_RESET_PORT, LCD_RESET_PIN, 0);
	#endif

	vTaskDelay(100);

	#if defined(GD32F10X_HD) || defined(GD32F10X_XD)
		gpio_bit_set(LCD_RESET_PORT, 1 << LCD_RESET_PIN);
	#elif defined(STM32F10X_HD) || defined(STM32F10X_XL)
		GPIO_PinWrite(LCD_RESET_PORT, LCD_RESET_PIN, 1);
	#endif

	vTaskDelay(100);
}

void Display_HalSetBacklight(uint8_t isOn){
	#if defined(GD32F10X_HD) || defined(GD32F10X_XD)
		gpio_bit_write(LCD_BL_PORT, 1 << LCD_BL_PIN, isOn ? SET : RESET);
	#elif defined(STM32F10X_HD) || defined(STM32F10X_XL)
		GPIO_PinWrite(LCD_BL_PORT, LCD_BL_PIN, isOn ? 1: 0);
	#endif
}

//==============================================================================
// Private Functions (Base)
//==============================================================================
void display_init() {
	// Setup HAL and hardware as needed
	display_halInitGpio();
	Display_HalSetBacklight(0);
	display_halInitExternalBus();
	display_halInitDma();
	Display_HalReset();
	display_halConfigure();
}

//==============================================================================
// Private Functions (HAL)
//==============================================================================
static void display_halConfigure(void) {
	display_halWriteCommand(0xE0);
	display_halWriteData(0x00);
	display_halWriteData(0x13);
	display_halWriteData(0x18);
	display_halWriteData(0x04);
	display_halWriteData(0x0F);
	display_halWriteData(0x06);
	display_halWriteData(0x3A);
	display_halWriteData(0x56);
	display_halWriteData(0x4D);
	display_halWriteData(0x03);
	display_halWriteData(0x0A);
	display_halWriteData(0x06);
	display_halWriteData(0x30);
	display_halWriteData(0x3E);
	display_halWriteData(0x0F);

	display_halWriteCommand(0xE1);
	display_halWriteData(0x00);
	display_halWriteData(0x13);
	display_halWriteData(0x18);
	display_halWriteData(0x01);
	display_halWriteData(0x11);
	display_halWriteData(0x06);
	display_halWriteData(0x38);
	display_halWriteData(0x34);
	display_halWriteData(0x4D);
	display_halWriteData(0x06);
	display_halWriteData(0x0D);
	display_halWriteData(0x0B);
	display_halWriteData(0x31);
	display_halWriteData(0x37);
	display_halWriteData(0x0f);

	display_halWriteCommand(0xC0);
	display_halWriteData(0x13);
	display_halWriteData(0x13);

	display_halWriteCommand(0xC1);
	display_halWriteData(0x41);

	display_halWriteCommand(0xC5);
	display_halWriteData(0x00);
	display_halWriteData(0x2c);	// VCOM
	display_halWriteData(0x80);

	display_halWriteCommand(0x36);
	display_halWriteData(0x08);	// BGR

	display_halWriteCommand(0x3A);	// Interface Mode Control
	display_halWriteData(0x55);	// 0x55:5-6-5 / 0x66:6-6-6

	display_halWriteCommand(0XB0);	// Interface Mode Control
	display_halWriteData(0x00);

	display_halWriteCommand(0xB1);	// Frame rate 70HZ
	display_halWriteData(0xB0);

	display_halWriteCommand(0xB4);
	display_halWriteData(0x02);

	display_halWriteCommand(0xB6);	// Interface Control
	display_halWriteData(0x02);	// RGB:72 /MCU:02
	display_halWriteData(0x22);

	display_halWriteCommand(0xE9);
	display_halWriteData(0x00);

	display_halWriteCommand(0XF7);
	display_halWriteData(0xA9);
	display_halWriteData(0x51);
	display_halWriteData(0x2C);
	display_halWriteData(0x82);

	display_halWriteCommand(0x21);

	display_halWriteCommand(0x11);	//Exit Sleep
	vTaskDelay(100);

	display_halWriteCommand(0x29);	//Display on
	vTaskDelay(100);

	display_halWriteCommand(0x2c);
	vTaskDelay(100);

	display_halWriteCommand(0x36);	// Set Display Mode
	display_halWriteData(0x68);		// Top to bottom, right to left
	//display_halWriteData(0x48);		// right to left, Top to bottom
}

static void display_halInitDma(void) {
	// The DMA will be used to transfer from some buffer address into the external bus for the LCD
	_dmaHandle = (DMA_HandleTypeDef) {
		.Instance = DMA1_Channel1,
		.Init = (DMA_InitTypeDef) {
			.Direction = DMA_MEMORY_TO_MEMORY,
			.PeriphInc = DMA_PINC_ENABLE,
			.MemInc = DMA_MINC_DISABLE,
			.PeriphDataAlignment = DMA_PDATAALIGN_HALFWORD,
			.MemDataAlignment = DMA_MDATAALIGN_HALFWORD,
			.Mode = DMA_NORMAL,
			.Priority = DMA_PRIORITY_LOW,
		},
		.Parent = NULL,
		.XferCpltCallback = &display_lvglFlushCompleteCallback,
		.XferHalfCpltCallback = NULL,
		.XferErrorCallback = NULL,
		.XferAbortCallback = NULL,
		.DmaBaseAddress = DMA1
	};

	__HAL_RCC_DMA1_CLK_ENABLE();
	HAL_DMA_Init(&_dmaHandle);

	NVIC_ClearPendingIRQ(DMA1_Channel1_IRQn);
	NVIC_EnableIRQ(DMA1_Channel1_IRQn);
}

static void display_halInitExternalBus(void) {
	__HAL_RCC_SRAM_CLK_ENABLE();

	// Init external SRAM bank
	SRAM_HandleTypeDef sramHandle = (SRAM_HandleTypeDef) {
		.Instance = FSMC_Bank1,
		.Extended= FSMC_Bank1E,
		.Init = (FSMC_NORSRAM_InitTypeDef) {
			.NSBank = FSMC_NORSRAM_BANK3,
			.DataAddressMux = FSMC_DATA_ADDRESS_MUX_DISABLE,
			.MemoryType = FSMC_MEMORY_TYPE_SRAM,
			.MemoryDataWidth = FSMC_NORSRAM_MEM_BUS_WIDTH_16,
			.BurstAccessMode = FSMC_BURST_ACCESS_MODE_DISABLE,
			.WaitSignalPolarity = FSMC_WAIT_SIGNAL_POLARITY_LOW,
			.WrapMode = FSMC_WRAP_MODE_DISABLE,
			.WaitSignalActive = FSMC_WAIT_SIGNAL_DISABLE,
			.WriteOperation = FSMC_WRITE_OPERATION_ENABLE,
			.ExtendedMode = FSMC_EXTENDED_MODE_DISABLE,
			.AsynchronousWait = FSMC_ASYNCHRONOUS_WAIT_DISABLE,
			.WriteBurst = FSMC_WRITE_BURST_DISABLE,
			.ContinuousClock = FSMC_CONTINUOUS_CLOCK_SYNC_ONLY,
		},
		.hdma = &_dmaHandle,
	};
	FSMC_NORSRAM_TimingTypeDef timing = (FSMC_NORSRAM_TimingTypeDef) {
		.AddressSetupTime = 0,
		.AddressHoldTime = 0,
		.DataSetupTime = 3,
		.BusTurnAroundDuration = 0,
		.CLKDivision = 0,
		.DataLatency = 0,
		.AccessMode = FSMC_ACCESS_MODE_A,
	};

	HAL_SRAM_Init(&sramHandle, &timing, &timing);
	HAL_SRAM_WriteOperation_Enable(&sramHandle);

	// Init the external bus for the lcd
	NOR_HandleTypeDef norHandle = (NOR_HandleTypeDef) {
		.Instance = FSMC_Bank1,
		.Extended = FSMC_Bank1E,
		.Init = {
			.NSBank = FSMC_NORSRAM_BANK1,
			.DataAddressMux = FSMC_DATA_ADDRESS_MUX_DISABLE,
			.MemoryType = FSMC_MEMORY_TYPE_NOR,
			.MemoryDataWidth = FSMC_NORSRAM_MEM_BUS_WIDTH_16,
			.BurstAccessMode = FSMC_BURST_ACCESS_MODE_DISABLE,
			.WaitSignalPolarity = FSMC_WAIT_SIGNAL_POLARITY_LOW,
			.WrapMode = FSMC_WRAP_MODE_DISABLE,
			.WaitSignalActive = FSMC_WAIT_TIMING_BEFORE_WS,
			.WriteOperation = FSMC_WRITE_OPERATION_ENABLE,
			.WaitSignal = FSMC_WAIT_SIGNAL_DISABLE,
			.ExtendedMode = FSMC_EXTENDED_MODE_DISABLE,
			.AsynchronousWait = FSMC_ASYNCHRONOUS_WAIT_DISABLE,
			.WriteBurst = FSMC_WRITE_BURST_DISABLE,
			.ContinuousClock = FSMC_CONTINUOUS_CLOCK_SYNC_ONLY,
		}
	};
	timing = (FSMC_NORSRAM_TimingTypeDef) {
		.AddressSetupTime = 2,
		.AddressHoldTime = 0,
		.DataSetupTime = 5,
		.BusTurnAroundDuration = 0,
		.CLKDivision = 0,
		.DataLatency = 0,
		.AccessMode = FSMC_ACCESS_MODE_B
	};

	HAL_NOR_Init(&norHandle, &timing, &timing);
	HAL_NOR_WriteOperation_Enable(&norHandle);
}

static void display_halInitGpio(void) {
	// Init GPIO pins
	GPIO_TypeDef* ports[] = {
		EXT_A0_PORT, EXT_A1_PORT, EXT_A2_PORT, EXT_A3_PORT, EXT_A4_PORT, EXT_A5_PORT, EXT_A6_PORT, EXT_A7_PORT,
		EXT_A8_PORT, EXT_A9_PORT, EXT_A10_PORT, EXT_A11_PORT, EXT_A12_PORT, EXT_A13_PORT, EXT_A14_PORT, EXT_A15_PORT,
		EXT_A16_PORT, EXT_A17_PORT, EXT_A18_PORT,
		EXT_D0_PORT, EXT_D1_PORT, EXT_D2_PORT, EXT_D3_PORT, EXT_D4_PORT, EXT_D5_PORT, EXT_D6_PORT, EXT_D7_PORT,
		EXT_D8_PORT, EXT_D9_PORT, EXT_D10_PORT, EXT_D11_PORT, EXT_D12_PORT, EXT_D13_PORT, EXT_D14_PORT, EXT_D15_PORT, 
		EXT_NBL0_PORT, EXT_NBL1_PORT, EXT_NE1_PORT, EXT_NE2_PORT, EXT_NOE_PORT, EXT_NWE_PORT, 
	};
	uint8_t pins[] = {
		EXT_A0_PIN, EXT_A1_PIN, EXT_A2_PIN, EXT_A3_PIN, EXT_A4_PIN, EXT_A5_PIN, EXT_A6_PIN, EXT_A7_PIN,
		EXT_A8_PIN, EXT_A9_PIN, EXT_A10_PIN, EXT_A11_PIN, EXT_A12_PIN, EXT_A13_PIN, EXT_A14_PIN, EXT_A15_PIN,
		EXT_A16_PIN, EXT_A17_PIN, EXT_A18_PIN,
		EXT_D0_PIN, EXT_D1_PIN, EXT_D2_PIN, EXT_D3_PIN, EXT_D4_PIN, EXT_D5_PIN, EXT_D6_PIN, EXT_D7_PIN,
		EXT_D8_PIN, EXT_D9_PIN, EXT_D10_PIN, EXT_D11_PIN, EXT_D12_PIN, EXT_D13_PIN, EXT_D14_PIN, EXT_D15_PIN,
		EXT_NBL0_PIN, EXT_NBL1_PIN, EXT_NE1_PIN, EXT_NE2_PIN, EXT_NOE_PIN, EXT_NWE_PIN,
	};

	GPIO_InitTypeDef gpio = {
		.Mode = GPIO_MODE_AF_PP,
		.Speed = GPIO_SPEED_FREQ_HIGH,
		.Alternate = GPIO_AF12_FSMC,
	};

	__HAL_RCC_GPIOA_CLK_ENABLE();
	__HAL_RCC_GPIOB_CLK_ENABLE();
	__HAL_RCC_GPIOC_CLK_ENABLE();
	__HAL_RCC_GPIOD_CLK_ENABLE();
	__HAL_RCC_GPIOE_CLK_ENABLE();
	__HAL_RCC_GPIOF_CLK_ENABLE();
	__HAL_RCC_GPIOG_CLK_ENABLE();

	for (uint8_t p = 0; p < 41; p++) {
		gpio.Pin = 1 << pins[p];
		HAL_GPIO_Init(ports[p], &gpio);
	}

	HAL_GPIO_Init(LCD_BL_PORT, &((GPIO_InitTypeDef) { .Pin = 1 << LCD_BL_PIN, .Mode = GPIO_MODE_OUTPUT_PP, .Speed = GPIO_SPEED_HIGH }) );						// LCD Backlight
	HAL_GPIO_Init(LCD_BUSY_PORT, &((GPIO_InitTypeDef) { .Pin = 1 << LCD_BUSY_PIN, .Mode = GPIO_MODE_INPUT, .Pull = GPIO_PULLUP, .Speed = GPIO_SPEED_HIGH }) );	// LCD Busy Indicator
	HAL_GPIO_Init(LCD_RESET_PORT, &((GPIO_InitTypeDef) { .Pin = 1 << LCD_RESET_PIN, .Mode = GPIO_MODE_OUTPUT_PP, .Speed = GPIO_SPEED_HIGH }) );				// LCD Reset
}

//static uint32_t display_halGetId(void) {
//	uint16_t buf[4];
//
//	display_halWriteCommand(0xD3);
//
//	buf[0] = displayHal_readData() & 0x00FF;
//	buf[1] = displayHal_readData() & 0x00FF;
//	buf[2] = displayHal_readData() & 0x00FF;
//	buf[3] = displayHal_readData() & 0x00FF;
//
//	return (uint32_t) ((buf[1] << 16) | (buf[2] << 8) | buf[3]);
//}
//
//static uint16_t display_halReadData(void) {
//	return *((uint16_t*) 0x60000002);
//}

static void display_halWriteBlock(uint16_t xStart, uint16_t xEnd, uint16_t yStart, uint16_t yEnd) {
	display_halWriteCommand(0x2A);
	display_halWriteData(xStart >> 8);
	display_halWriteData(xStart & 0xFF);
	display_halWriteData(xEnd >> 8);
	display_halWriteData(xEnd & 0xFF);

	display_halWriteCommand(0x2B);
	display_halWriteData(yStart >> 8);
	display_halWriteData(yStart & 0xFF);
	display_halWriteData(yEnd >> 8);
	display_halWriteData(yEnd & 0xFF);

	display_halWriteCommand(0x2C);
}

static inline void display_halWriteCommand(uint16_t command) {
	LCD_WRITE_COMMAND(command);
}

static inline void display_halWriteData(uint16_t data) {
	LCD_WRITE_DATA(data);
}

//==============================================================================
// Private Functions (LVGL)
//==============================================================================
static void display_lvglInit(void) {
	// Initialize LVGL library
	lv_init();

	// Initialize draw buffer
	static lv_disp_draw_buf_t drawBuffer;
	static const lv_color_t* buf1 = (lv_color_t*) 0x68000000;
	static const lv_color_t* buf2 = (lv_color_t*) 0x68040000;
	// SRAM is 512KB, Each buffer should be 262,144 bytes, lv_color-t is 8 bytes -> buffer size is 32768
	lv_disp_draw_buf_init(&drawBuffer, (void*) buf1, (void*) buf2, 32768);

	// Initialize display driver
	lv_disp_drv_init(&_displayDriver);
	_displayDriver.hor_res = 480;
	_displayDriver.ver_res = 320;
	_displayDriver.draw_buf = &drawBuffer;
	_displayDriver.flush_cb = display_lvglFlushCallback;
	_displayDriver.wait_cb = display_lvglWaitCallback;
	lv_disp_drv_register(&_displayDriver);
}

static void display_lvglFlushCallback(lv_disp_drv_t* displayDriver, const lv_area_t* area, lv_color_t* color) {
	(void) displayDriver;

	display_halWriteBlock(area->x1, area->x2, area->y1, area->y2);

	uint32_t numWrites = (area->x2 - area->x1 + 1) * (area->y2 - area->y1 + 1);

	HAL_DMA_Start_IT(&_dmaHandle, (uint32_t) color, (uint32_t) 0x60000002, numWrites);
}

static void display_lvglFlushCompleteCallback(DMA_HandleTypeDef *dmaHandle) {
	(void) dmaHandle;

	lv_disp_flush_ready(&_displayDriver);
}

static void display_lvglWaitCallback(struct _lv_disp_drv_t* displayDriver) {
	vTaskDelay(1);
}

//==============================================================================
// Task Handler
//==============================================================================
void Display_TaskHandler(void* p) {
	display_init();
	display_lvglInit();

	lv_obj_t* main = lv_obj_create(lv_scr_act());
	lv_obj_set_size(main, 480, 320);
	lv_obj_center(main);
	lv_obj_t* label = lv_label_create(main);
	lv_label_set_text(label,  "Hello world!");
	lv_obj_center(label);

	TickType_t lastWakeTime = xTaskGetTickCount();

	while(1) {
		vTaskDelayUntil(&lastWakeTime, pdMS_TO_TICKS(15));

		lv_timer_handler();
	}
}

//==============================================================================
// Interrupt
//==============================================================================
void DMA1_Channel1_IRQHandler(void) {
	HAL_DMA_IRQHandler(&_dmaHandle);
}
