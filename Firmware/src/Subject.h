#ifndef SUBJECT_H_
#define SUBJECT_H_

//==============================================================================
// Notes
//==============================================================================


//==============================================================================
// Definitions
//==============================================================================


//==============================================================================
// Includes
//==============================================================================


//==============================================================================
// Enumerations and Structures
//==============================================================================
typedef enum ChangeFlag_t {
	ChangeFlag_BatteryVoltage =			(1 << 0),
	ChangeFlag_Beam =					(1 << 1),
	ChangeFlag_Clock =					(1 << 2),
	ChangeFlag_CurrentSpeed =			(1 << 3),
	ChangeFlag_Distance =				(1 << 4),
	ChangeFlag_FuelLevel =				(1 << 5),
	ChangeFlag_Neutral =				(1 << 6),
	ChangeFlag_Oil =					(1 << 7),
	ChangeFlag_TimeElapsed =			(1 << 8),
	ChangeFlag_Turn_Left =				(1 << 9),
	ChangeFlag_Turn_Right =				(1 << 10),
	ChangeFlag_AllChanges =				0x3FF,
} ChangeFlag_t;

typedef void (*Notify_f)(ChangeFlag_t);

typedef struct Observer_t Observer_t;
struct Observer_t {
	uint16_t Subscriptions;
	Notify_f Notify;
	Observer_t* Next;
};

//==============================================================================
// Function Prototypes
//==============================================================================
int Subject_Init();
int Subject_Start();

// Macro helper functions
void Subject_ObserverSubscribe(Observer_t* observer);

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Macro Functions
//==============================================================================
#define OBSERVER_CREATE_NEW(observer, subscriptions, notify) \
static Observer_t observer = Observer_t { \
	.Next = NULL, \
	.Subscriptions = subscriptions, \
	.Notify = notify, \
};

#define OBSERVER_SUBSCRIBE(observer, subscriptions, notify) \
OBSERVER_CREATE_NEW(observer, subscriptions, notify); \
Subject_ObserverSubscribe(&observer);

#endif /* Subject.h */
