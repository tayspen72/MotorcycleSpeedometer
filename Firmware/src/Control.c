<<<<<<< HEAD
/*
 * Camp Chef
 * Copyright (C) 2024, All rights reserved.
 *
 * Control.c
 */

=======
>>>>>>> 4bad30e (Updated framework for nrf project)
//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Includes
//==============================================================================
#include <zephyr/kernel.h>
<<<<<<< HEAD
#include <zephyr/logging/log.h>
#include <zephyr/devicetree.h>
#include <zephyr/drivers/gpio.h>
=======
#include <zephyr/drivers/gpio.h>
#include <zephyr/logging/log.h>

#include <helpers/nrfx_gppi.h>
#include <nrfx_timer.h>
#include <nrfx_gpiote.h>
>>>>>>> 4bad30e (Updated framework for nrf project)

#include "Control.h"

//==============================================================================
// Definitions
//==============================================================================
LOG_MODULE_REGISTER(Control, LOG_LEVEL_INF);
<<<<<<< HEAD

#define BEAM_NODE				DT_NODELABEL(button0)
#define NEUTRAL_NODE			DT_NODELABEL(button1)
#define OIL_NODE				DT_NODELABEL(button2)
#define TURN_NODE				DT_NODELABEL(button3)

=======
#define BUTTON_NODE		DT_ALIAS(sw0)
>>>>>>> 4bad30e (Updated framework for nrf project)

//==============================================================================
// Enumerations and Structures
//==============================================================================


//==============================================================================
// Private Function Prototypes
//==============================================================================
static void control_handler(void* args);

<<<<<<< HEAD
static void control_beamHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins);
static void control_neutralHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins);
static void control_oilHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins);
static void control_turnHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins);
=======
static void pulse_count_init(uint32_t pin);
static void pulse_count_reset(void);
static uint32_t pulse_count_sample(void);

void control_gpioteInterruptHandler(nrfx_gpiote_pin_t pin, nrfx_gpiote_trigger_t trigger, void* p_context);
>>>>>>> 4bad30e (Updated framework for nrf project)

//==============================================================================
// Variables
//==============================================================================
K_THREAD_DEFINE(_controlThread, 1024, control_handler, NULL, NULL, NULL, 4, 0, -1);
<<<<<<< HEAD

static const struct gpio_dt_spec beam = GPIO_DT_SPEC_GET(BEAM_NODE, gpios);
static const struct gpio_dt_spec neutral = GPIO_DT_SPEC_GET(NEUTRAL_NODE, gpios);
static const struct gpio_dt_spec oil = GPIO_DT_SPEC_GET(OIL_NODE, gpios);
static const struct gpio_dt_spec turn = GPIO_DT_SPEC_GET(TURN_NODE, gpios);
=======
static nrfx_timer_t _timer = NRFX_TIMER_INSTANCE(0);
static const struct gpio_dt_spec button = GPIO_DT_SPEC_GET(BUTTON_NODE, gpios);
>>>>>>> 4bad30e (Updated framework for nrf project)

//==============================================================================
// Public Functions
//==============================================================================
int Control_Init() {
	LOG_INF("Initializing");
<<<<<<< HEAD

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

=======
	pulse_count_init(button.pin);
>>>>>>> 4bad30e (Updated framework for nrf project)
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
<<<<<<< HEAD

=======
static void pulse_count_init(uint32_t pin) {
	nrfx_gpiote_t gpiote = NRFX_GPIOTE_INSTANCE(0);

	uint8_t gpioteChannel;


	uint8_t ppiChannel;

	uint32_t err;

	// Configure GPIOTE as event generator by low-to-high edge transition
	err = nrfx_gpiote_init(&gpiote, NRFX_GPIOTE_DEFAULT_CONFIG_IRQ_PRIORITY);
	NRFX_ASSERT(err == NRFX_SUCCESS);

	err = nrfx_gpiote_channel_alloc(&gpiote, &gpioteChannel);
	NRFX_ASSERT(err == NRFX_SUCCESS);

	nrf_gpio_pin_pull_t pinPullConfig = NRF_GPIO_PIN_PULLUP;
	nrfx_gpiote_trigger_config_t triggerConfig = {
		.trigger = NRFX_GPIOTE_TRIGGER_LOTOHI,
		.p_in_channel = &gpioteChannel,
	};
	nrfx_gpiote_handler_config_t handlerConfig = {
		.handler = &control_gpioteInterruptHandler,
		.p_context = NULL,
	};
	nrfx_gpiote_input_pin_config_t inputConfig = {
		.p_pull_config = &pinPullConfig,
		.p_trigger_config = &triggerConfig,
		.p_handler_config = &handlerConfig,
	};
	err = nrfx_gpiote_input_configure(&gpiote, pin, &inputConfig);
	NRFX_ASSERT(err == NRFX_SUCCESS);

	nrfx_gpiote_trigger_enable(&gpiote, pin, false);

	// Initialize the timer in counter mode
	nrfx_timer_config_t timerConfig = {
		.mode = NRF_TIMER_MODE_COUNTER,
		.bit_width = NRF_TIMER_BIT_WIDTH_32,
		.frequency = 1000000,
	};

	err = nrfx_timer_init(&_timer, &timerConfig, NULL);
	NRFX_ASSERT(err == NRFX_SUCCESS);

	// Configure the PPO channel linking the input event and timer count
	err = nrfx_gppi_channel_alloc(&ppiChannel);
	NRFX_ASSERT(err == NRFX_SUCCESS);
	uint32_t eventAddress = nrfx_gpiote_in_event_address_get(&gpiote , pin);
	uint32_t taskAddress = nrfx_timer_task_address_get(&_timer, NRF_TIMER_TASK_COUNT);

	nrfx_gppi_channel_endpoints_setup(ppiChannel, eventAddress, taskAddress);

	// Enable the PPI channel
	nrfx_gppi_channels_enable(BIT(ppiChannel));

	// Enable the timer
	nrfx_timer_enable(&_timer);
}

static void pulse_count_reset(void){
	nrfx_timer_clear(&_timer);
}

static uint32_t pulse_count_sample(void){
	return nrfx_timer_capture_get(&_timer, NRF_TIMER_CC_CHANNEL0);
}
>>>>>>> 4bad30e (Updated framework for nrf project)

//==============================================================================
// Task Handler
//==============================================================================
static void control_handler(void* args) {
<<<<<<< HEAD

	while(1) {
		k_msleep(1000);
=======
	(void) args;

	while(1) {
		k_msleep(1000);
		LOG_INF("Pulse count: %u", pulse_count_sample());
		pulse_count_reset();
>>>>>>> 4bad30e (Updated framework for nrf project)
	}
}

//==============================================================================
// Interrupt
//==============================================================================
<<<<<<< HEAD
static void control_beamHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {

}

static void control_neutralHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {

}

static void control_oilHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {

}

static void control_turnHandler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {

=======
void control_gpioteInterruptHandler(nrfx_gpiote_pin_t pin, nrfx_gpiote_trigger_t trigger, void* p_context) {
	LOG_INF("gpiote interrupt handler for pin %u", pin);
>>>>>>> 4bad30e (Updated framework for nrf project)
}
