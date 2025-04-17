//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Includes
//==============================================================================
#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>

#include "Subject.h"

//==============================================================================
// Definitions
//==============================================================================
LOG_MODULE_REGISTER(Subject, LOG_LEVEL_INF);

#define CHANGE_QUEUE_LENGTH				16
#define MSQ_QUEUE_PUT_TIMEOUT			K_MSEC(10)
//==============================================================================
// Enumerations and Structures
//==============================================================================


//==============================================================================
// Private Function Prototypes
//==============================================================================

static void subject_observerAdd(Observer_t* observer);
static int subject_observerExists(const Observer_t* this);
static void subject_observerNotify(ChangeFlag_t changeFlag);

static void subject_handler(void* args);

//==============================================================================
// Variables
//==============================================================================
K_THREAD_DEFINE(_subjectThread, 1024, subject_handler, NULL, NULL, NULL, 4, 0, -1);
K_MSGQ_DEFINE(_subjectChangeQueue, sizeof(ChangeFlag_t), CHANGE_QUEUE_LENGTH, sizeof(uint8_t));

static Observer_t* _first = NULL;

//==============================================================================
// Public Functions
//==============================================================================
int Subject_Init() {
	LOG_INF("Initializing");
	return 0;
}

int Subject_Start() {
	LOG_INF("Starting");
	k_thread_start(_subjectThread);
	return 0;
}

void Subject_ObserverSubscribe(Observer_t* observer) {
	if (subject_observerExists(observer))
		return;

	subject_observerAdd(observer);
}

//==============================================================================
// Private Functions
//==============================================================================
static void subject_observerAdd(Observer_t* newObserver) {
	if (subject_observerExists(newObserver))
		return;

	if (!_first) {
		_first = newObserver;
		newObserver->Next = NULL;
		return;
	}

	Observer_t* observer = _first;
	while(observer->Next) {
		observer = observer->Next;
	}

	observer->Next = newObserver;
	newObserver->Next = NULL;
	return;
}

static int subject_observerExists(const Observer_t* this) {
	for (Observer_t* observer = _first; observer; observer = observer->Next) {
		if (observer == this) {
			return 1;
		}
	}
	return 0;
}

static void subject_observerNotify(ChangeFlag_t changeFlag) {
	for (Observer_t* observer = _first; observer; observer = observer->Next) {
		if ((observer->Subscriptions & changeFlag) && (observer->Notify)) {
			observer->Notify(changeFlag);
		}
	}
}

//==============================================================================
// Task Handler
//==============================================================================
static void subject_handler(void* args) {
	ChangeFlag_t changeFlag;

	while(1) {
		// Wait for a new event to be pushed to the QUEUE
		if (k_msgq_get(&_subjectChangeQueue, &changeFlag, K_FOREVER) != 0)
			continue;

		subject_observerNotify(changeFlag);
	}
}

//==============================================================================
// Interrupt
//==============================================================================

