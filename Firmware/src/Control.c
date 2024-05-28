/*
 * Camp Chef
 * Copyright (C) 2024, All rights reserved.
 *
 * Control.c
 */

//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Includes
//==============================================================================
#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>
#include <zephyr/devicetree.h>
#include <zephyr/drivers/gpio.h>

#include "Control.h"

//==============================================================================
// Definitions
//==============================================================================
LOG_MODULE_REGISTER(Control, LOG_LEVEL_INF);

#define BEAM_NODE				DT_NODELABEL(button0)
#define NEUTRAL_NODE			DT_NODELABEL(button1)
#define OIL_NODE				DT_NODELABEL(button2)
#define TURN_NODE				DT_NODELABEL(button3)


//==============================================================================
// Enumerations and Structures
//==============================================================================


//==============================================================================
// Private Function Prototypes
//==============================================================================
static void control_handler(void* args);

static void control_beamHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins);
static void control_neutralHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins);
static void control_oilHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins);
static void control_turnHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins);

//==============================================================================
// Variables
//==============================================================================
K_THREAD_DEFINE(_controlThread, 1024, control_handler, NULL, NULL, NULL, 4, 0, -1);

static const struct gpio_dt_spec beam = GPIO_DT_SPEC_GET(BEAM_NODE, gpios);
static const struct gpio_dt_spec neutral = GPIO_DT_SPEC_GET(NEUTRAL_NODE, gpios);
static const struct gpio_dt_spec oil = GPIO_DT_SPEC_GET(OIL_NODE, gpios);
static const struct gpio_dt_spec turn = GPIO_DT_SPEC_GET(TURN_NODE, gpios);

//==============================================================================
// Public Functions
//==============================================================================
int Control_Init() {
	LOG_INF("Initializing");

	static struct gpio_callback beamCBData;
	static struct gpio_callback neutralCBData;
	static struct gpio_callback oilCBData;
	static struct gpio_callback turnCBData;

	if (!device_is_ready(beam.port))
		return -1;
	else if (gpio_pin_configure_dt(&beam, GPIO_INPUT) < 0)
		return -1;
	else if (gpio_pin_interrupt_configure_dt(&beam, GPIO_INT_EDGE_TO_ACTIVE) < 0)
		return -1;
	gpio_init_callback(&beamCBData, control_beamHandler, BIT(beam.pin));
	gpio_add_callback(beam.port, &beamCBData);

	if (!device_is_ready(neutral.port))
		return -1;
	else if (gpio_pin_configure_dt(&neutral, GPIO_INPUT) < 0)
		return -1;
	else if (gpio_pin_interrupt_configure_dt(&neutral, GPIO_INT_EDGE_TO_ACTIVE) < 0)
		return -1;
	gpio_init_callback(&neutralCBData, control_neutralHandler, BIT(neutral.pin));
	gpio_add_callback(neutral.port, &neutralCBData);
	
	if (!device_is_ready(oil.port))
		return -1;
	else if (gpio_pin_configure_dt(&oil, GPIO_INPUT) < 0)
		return -1;
	else if (gpio_pin_interrupt_configure_dt(&oil, GPIO_INT_EDGE_TO_ACTIVE) < 0)
		return -1;
	gpio_init_callback(&oilCBData, control_oilHandler, BIT(oil.pin));
	gpio_add_callback(oil.port, &oilCBData);

	if (!device_is_ready(turn.port))
		return -1;
	else if (gpio_pin_configure_dt(&turn, GPIO_INPUT) < 0)
		return -1;
	else if (gpio_pin_interrupt_configure_dt(&turn, GPIO_INT_EDGE_TO_ACTIVE) < 0)
		return -1;
	gpio_init_callback(&turnCBData, control_turnHandler, BIT(turn.pin));
	gpio_add_callback(turn.port, &turnCBData);

	return 0;
}

int Control_Start() {
	LOG_INF("Starting");
	k_thread_start(_controlThread);
	return 0;
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
static void control_handler(void* args) {

	while(1) {
		k_msleep(1000);
	}
}

//==============================================================================
// Interrupt
//==============================================================================
static void control_beamHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {

}

static void control_neutralHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {

}

static void control_oilHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {

}

static void control_turnHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {

}
