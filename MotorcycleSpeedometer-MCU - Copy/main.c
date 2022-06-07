/*
 * Camp Chef
 * Copyright (C) 2022, All rights reserved.
 *
 * main.c
 */

//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Includes
//==============================================================================
#include "display.h"
#include "led.h"

// FreeRTOS Includes
#include "FreeRTOS.h"
#include "queue.h"
#include "task.h"
#include "controls/led/led.h"

//==============================================================================
// Definitions
//==============================================================================


//==============================================================================
// Enumerations and Structures
//==============================================================================


//==============================================================================
// Function Prototypes
//==============================================================================
static void initTask(void* p);

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Main
//==============================================================================
int main() {
	xTaskCreate(initTask, "initTask", configMINIMAL_STACK_SIZE, NULL, 4, NULL);

	vTaskStartScheduler();

	while(1);
}

//==============================================================================
// Private Functions
//==============================================================================
static void initTask(void* p) {
//	static QueueHandle_t infoChangeQueue;

	xTaskCreate(Display_TaskHandler, "Display", configMINIMAL_STACK_SIZE, NULL, 4, NULL);
	xTaskCreate(Led_TaskHandler, "LED", configMINIMAL_STACK_SIZE, NULL, 4, NULL);

	// Delete this task when finished
	vTaskDelete(NULL);
}

//==============================================================================
// Task Handler
//==============================================================================


//==============================================================================
// Interrupt
//==============================================================================

