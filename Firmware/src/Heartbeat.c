/*
 * Camp Chef
 * Copyright (C) 2024, All rights reserved.
 *
 * Heartbeat.c
 */

//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Includes
//==============================================================================
#include <zephyr/kernel.h>
#include <zephyr/device.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/logging/log.h>

#include "Heartbeat.h"

//==============================================================================
// Definitions
//==============================================================================
LOG_MODULE_REGISTER(Heartbeat, LOG_LEVEL_INF);

#define LED0_NODE DT_ALIAS(led0)

#define HEARTBEAT_BPM			70

//==============================================================================
// Enumerations and Structures
//==============================================================================
typedef struct HeartbeatState_t {
	int8_t State;
	int16_t Duration;
} HeartbeatState_t;

//==============================================================================
// Private Function Prototypes
//==============================================================================
static void heartbeat_handler(void* args);

//==============================================================================
// Variables
//==============================================================================
K_THREAD_DEFINE(_heartbeatThread, 1024, heartbeat_handler, NULL, NULL, NULL, 14, 0, -1);

static const struct gpio_dt_spec led0 = GPIO_DT_SPEC_GET_OR(LED0_NODE, gpios, {0});

static HeartbeatState_t _states[] = {
	{ .State = 0, .Duration = (HEARTBEAT_BPM * 1000 / 60) - 400 },
	{ .State = 1, .Duration = 75 },
	{ .State = 0, .Duration = 250 },
	{ .State = 1, .Duration = 75 },
};

//==============================================================================
// Public Functions
//==============================================================================
int Heartbeat_Init() {
	if (!gpio_is_ready_dt(&led0)) {
		LOG_ERR("GPIO is not ready!");
		return -1;
	}

	int ret = gpio_pin_configure_dt(&led0, GPIO_OUTPUT_ACTIVE);
	if (ret < 0) {
		LOG_ERR("Failed to configure GPIO pin!");
		return ret;
	}

	return 0;
}

int Heartbeat_Start() {
	k_thread_start(_heartbeatThread);
	return 0;
}

//==============================================================================
// Private Functions
//==============================================================================
static void heartbeat_handler(void* args) {
	static int8_t step = 0;

	while(1) {
		gpio_pin_set_dt(&led0, _states[step].State);
		k_msleep(_states[step].Duration);

		step++;
		if (step >= 4)
			step = 0;
	}
}

//==============================================================================
// Task Handler
//==============================================================================


//==============================================================================
// Interrupt
//==============================================================================

