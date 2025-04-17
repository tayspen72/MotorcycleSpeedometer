//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Includes
//==============================================================================
#include <zephyr/bluetooth/bluetooth.h>
#include <zephyr/bluetooth/gap.h>
#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>

#include "Bluetooth.h"

//==============================================================================
// Definitions
//==============================================================================
LOG_MODULE_REGISTER(Bluetooth, LOG_LEVEL_INF);

#define DEVICE_NAME CONFIG_BT_DEVICE_NAME
#define DEVICE_NAME_LEN (sizeof(DEVICE_NAME) - 1)

//==============================================================================
// Enumerations and Structures
//==============================================================================


//==============================================================================
// Private Function Prototypes
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static const struct bt_le_adv_param _advertisingParameters[] = BT_LE_ADV_PARAM(
	(BT_LE_ADV_OPT_CONNECTABLE | BT_LE_ADV_OPT_USE_IDENTITY), /* Connectable advertising and use identity address */
	BT_GAP_ADV_FAST_INT_MIN_1, 		/* 0x30 units, 48 units, 30ms */
	BT_GAP_ADV_FAST_INT_MAX_1,		/* 0x60 units, 96 units, 60ms */
	NULL 							/* Set to NULL for undirected advertising */
); 

static const struct bt_data _advertiseData[] = {
	BT_DATA_BYTES(BT_DATA_FLAGS, BT_LE_AD_GENERAL | BT_LE_AD_NO_BREDR),
	BT_DATA(BT_DATA_NAME_COMPLETE, DEVICE_NAME, DEVICE_NAME_LEN),
};

static unsigned char url_data[] = { 0x17, '/', '/', 'w', 'w', 'w', '.', 'c', 'a',
	'm',  'p', 'c', 'h', 'e', 'f', '.', 'c', 'o', 'm' };

static const struct bt_data _scanResponseData[] = {
	BT_DATA(BT_DATA_URI, url_data, sizeof(url_data)),
};

//==============================================================================
// Public Functions
//==============================================================================
int Bluetooth_Init() {
	LOG_INF("Initializing");

	int err = bt_enable(NULL);
	if (err) {
		LOG_ERR("Bluetooth failed to init: %d\n", err);
		return err;
	}

	return 0;
}

int Bluetooth_Start() {
	LOG_INF("Starting");

	int err = bt_le_adv_start(_advertisingParameters, _advertiseData, ARRAY_SIZE(_advertiseData), _scanResponseData, ARRAY_SIZE(_scanResponseData));
	if (err) {
		LOG_ERR("Advertising failed to start (err %d)\n", err);
		return err;
	}

	LOG_INF("Advertising successfully started\n");

	return 0;
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================


//==============================================================================
// Interrupt
//==============================================================================

