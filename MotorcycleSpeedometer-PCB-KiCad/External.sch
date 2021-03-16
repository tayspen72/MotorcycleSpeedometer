EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 4 4
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Connector_Generic:Conn_02x08_Odd_Even J?
U 1 1 602C088A
P 2500 4850
AR Path="/602C088A" Ref="J?"  Part="1" 
AR Path="/602379AF/602C088A" Ref="J?"  Part="1" 
AR Path="/602B901C/602C088A" Ref="J?"  Part="1" 
F 0 "J?" H 2550 5367 50  0000 C CNN
F 1 "Wire Harness Connector" H 2550 5276 50  0000 C CNN
F 2 "" H 2500 4850 50  0001 C CNN
F 3 "~" H 2500 4850 50  0001 C CNN
	1    2500 4850
	1    0    0    -1  
$EndComp
Text Label 1550 4550 0    50   ~ 0
VDD_CONST_EXT
Text Label 1550 4650 0    50   ~ 0
TURN_L_EXT
Text Label 1550 4750 0    50   ~ 0
TURN_R_EXT
Text Label 1550 4850 0    50   ~ 0
HIGH_BEAM_EXT
Text Label 1550 4950 0    50   ~ 0
BACKLIGHT_EXT
Text Label 1550 5050 0    50   ~ 0
OIL_EXT
Text Label 1550 5150 0    50   ~ 0
FUEL_EXT
Text Label 1550 5250 0    50   ~ 0
GND_EXT
Text Label 3550 4550 2    50   ~ 0
SPEED_A_EXT
Text Label 3550 5250 2    50   ~ 0
SPEED_B_EXT
Text Label 3550 4750 2    50   ~ 0
NC
Text Label 3550 4950 2    50   ~ 0
NEUTRAL_EXT
Text Label 3550 5050 2    50   ~ 0
VDD_FUSE_EXT
NoConn ~ 2800 4650
NoConn ~ 2800 4850
NoConn ~ 2800 5150
Wire Wire Line
	1550 4550 2300 4550
Wire Wire Line
	1550 4650 2300 4650
Wire Wire Line
	1550 4750 2300 4750
Wire Wire Line
	1550 4850 2300 4850
Wire Wire Line
	1550 4950 2300 4950
Wire Wire Line
	1550 5050 2300 5050
Wire Wire Line
	1550 5150 2300 5150
Wire Wire Line
	1550 5250 2300 5250
Wire Wire Line
	3550 5250 2800 5250
Wire Wire Line
	3550 5050 2800 5050
Wire Wire Line
	3550 4950 2800 4950
Wire Wire Line
	3550 4750 2800 4750
Wire Wire Line
	3550 4550 2800 4550
$Comp
L Device:R_US R?
U 1 1 602C08AD
P 1500 1150
AR Path="/602C08AD" Ref="R?"  Part="1" 
AR Path="/602379AF/602C08AD" Ref="R?"  Part="1" 
AR Path="/602B901C/602C08AD" Ref="R?"  Part="1" 
F 0 "R?" V 1450 1000 50  0000 L BNN
F 1 "150k" V 1450 1200 50  0000 L BNN
F 2 "" V 1540 1140 50  0001 C CNN
F 3 "~" H 1500 1150 50  0001 C CNN
	1    1500 1150
	1    0    0    -1  
$EndComp
$Comp
L Device:R_US R?
U 1 1 602C08B3
P 1500 1850
AR Path="/602C08B3" Ref="R?"  Part="1" 
AR Path="/602379AF/602C08B3" Ref="R?"  Part="1" 
AR Path="/602B901C/602C08B3" Ref="R?"  Part="1" 
F 0 "R?" V 1450 1700 50  0000 L BNN
F 1 "470k" V 1450 1900 50  0000 L BNN
F 2 "" V 1540 1840 50  0001 C CNN
F 3 "~" H 1500 1850 50  0001 C CNN
	1    1500 1850
	1    0    0    -1  
$EndComp
Text Label 900  750  0    50   ~ 0
VDD_FUSED_EXT
Wire Wire Line
	900  750  1500 750 
Wire Wire Line
	1500 750  1500 1000
Wire Wire Line
	1500 1300 1500 1500
$Comp
L power:GND #PWR?
U 1 1 602C08BD
P 1500 2300
AR Path="/602C08BD" Ref="#PWR?"  Part="1" 
AR Path="/602379AF/602C08BD" Ref="#PWR?"  Part="1" 
AR Path="/602B901C/602C08BD" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 1500 2050 50  0001 C CNN
F 1 "GND" H 1505 2127 50  0000 C CNN
F 2 "" H 1500 2300 50  0001 C CNN
F 3 "" H 1500 2300 50  0001 C CNN
	1    1500 2300
	1    0    0    -1  
$EndComp
Wire Wire Line
	1500 2300 1500 2000
Text Label 1650 1500 0    50   ~ 0
VBAT_ADC_MCU
Wire Wire Line
	1650 1500 1500 1500
Connection ~ 1500 1500
Wire Wire Line
	1500 1500 1500 1700
Text Notes 2050 2450 0    50   ~ 0
Battery ADC:\nR1: 150k\nR2: 470k\nV_in = V_adc * ((R1 + R2) / R1))\nV_in = V_adc * 4.133
$Comp
L Connector_Generic:Conn_01x02 J?
U 1 1 602C08C9
P 2950 3150
AR Path="/602C08C9" Ref="J?"  Part="1" 
AR Path="/602379AF/602C08C9" Ref="J?"  Part="1" 
AR Path="/602B901C/602C08C9" Ref="J?"  Part="1" 
F 0 "J?" H 3030 3142 50  0000 L CNN
F 1 "Conn_01x02" H 3030 3051 50  0000 L CNN
F 2 "" H 2950 3150 50  0001 C CNN
F 3 "~" H 2950 3150 50  0001 C CNN
	1    2950 3150
	1    0    0    -1  
$EndComp
Text Label 1450 3150 0    50   ~ 0
PUSH_BUTTON
$Comp
L Device:R_US R?
U 1 1 602C08D0
P 2300 3150
AR Path="/602C08D0" Ref="R?"  Part="1" 
AR Path="/602379AF/602C08D0" Ref="R?"  Part="1" 
AR Path="/602B901C/602C08D0" Ref="R?"  Part="1" 
F 0 "R?" V 2250 3000 50  0000 L BNN
F 1 "R_US" V 2250 3200 50  0000 L BNN
F 2 "" V 2340 3140 50  0001 C CNN
F 3 "~" H 2300 3150 50  0001 C CNN
	1    2300 3150
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR?
U 1 1 602C08D6
P 2650 3350
AR Path="/602C08D6" Ref="#PWR?"  Part="1" 
AR Path="/602379AF/602C08D6" Ref="#PWR?"  Part="1" 
AR Path="/602B901C/602C08D6" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 2650 3100 50  0001 C CNN
F 1 "GND" H 2655 3177 50  0000 C CNN
F 2 "" H 2650 3350 50  0001 C CNN
F 3 "" H 2650 3350 50  0001 C CNN
	1    2650 3350
	1    0    0    -1  
$EndComp
Wire Wire Line
	2650 3350 2650 3250
Wire Wire Line
	2650 3250 2750 3250
Wire Wire Line
	2750 3150 2450 3150
Wire Wire Line
	2150 3150 1450 3150
Wire Notes Line
	500  2750 4000 2750
Wire Notes Line
	500  3750 4000 3750
Text Notes 550  2700 0    50   ~ 0
Battery Voltage ADC
Text Notes 550  3700 0    50   ~ 0
Push Button
Wire Notes Line
	500  5750 4000 5750
Text Notes 550  5700 0    50   ~ 0
Wire Harness Connector
Wire Wire Line
	2250 6850 3000 6850
Wire Wire Line
	3000 6750 2250 6750
Wire Wire Line
	2250 6650 3000 6650
Wire Wire Line
	3000 6550 2250 6550
NoConn ~ 1750 6850
Connection ~ 1500 6950
Wire Wire Line
	1500 7050 1500 6950
$Comp
L power:GND #PWR?
U 1 1 6048C46B
P 1500 7050
F 0 "#PWR?" H 1500 6800 50  0001 C CNN
F 1 "GND" H 1505 6877 50  0000 C CNN
F 2 "" H 1500 7050 50  0001 C CNN
F 3 "" H 1500 7050 50  0001 C CNN
	1    1500 7050
	1    0    0    -1  
$EndComp
Connection ~ 1500 6750
Wire Wire Line
	1500 6950 1750 6950
Wire Wire Line
	1500 6750 1500 6950
Wire Wire Line
	1500 6750 1750 6750
Wire Wire Line
	1500 6650 1500 6750
Wire Wire Line
	1750 6650 1500 6650
Wire Wire Line
	1500 6450 1250 6450
Wire Wire Line
	1500 6550 1500 6450
Wire Wire Line
	1750 6550 1500 6550
Text Label 1250 6450 0    50   ~ 0
VDDS
Text Label 3000 6950 2    50   ~ 0
JTAG_RESET_MCU
Text Label 3000 6850 2    50   ~ 0
JTAG_TDI_MCU
Text Label 3000 6750 2    50   ~ 0
JTAG_TDO_MCU
Text Label 3000 6650 2    50   ~ 0
JTAG_TCK_MCU
Text Label 3000 6550 2    50   ~ 0
JTAG_TMS_MCU
$Comp
L Connector_Generic:Conn_02x05_Odd_Even J?
U 1 1 6048C480
P 1950 6750
F 0 "J?" H 2000 7167 50  0000 C CNN
F 1 "JTAG Connector" H 2000 7076 50  0000 C CNN
F 2 "" H 1950 6750 50  0001 C CNN
F 3 "~" H 1950 6750 50  0001 C CNN
	1    1950 6750
	1    0    0    -1  
$EndComp
Text Notes 550  7750 0    50   ~ 0
JTAG Connector
$Comp
L Device:R_US R?
U 1 1 6048C489
P 3250 6700
F 0 "R?" H 3318 6746 50  0000 L CNN
F 1 "100k" H 3318 6655 50  0000 L CNN
F 2 "" V 3290 6690 50  0001 C CNN
F 3 "~" H 3250 6700 50  0001 C CNN
	1    3250 6700
	1    0    0    -1  
$EndComp
Wire Wire Line
	3250 6950 3250 6850
Wire Wire Line
	2250 6950 3250 6950
Text Label 3550 6400 2    50   ~ 0
VDDS
Wire Wire Line
	3550 6400 3250 6400
Wire Wire Line
	3250 6400 3250 6550
Wire Notes Line
	4000 500  4000 7800
$EndSCHEMATC
