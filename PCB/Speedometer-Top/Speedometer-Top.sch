EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
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
L Connector_Generic:Conn_02x10_Odd_Even J1
U 1 1 6111457C
P 1450 1450
F 0 "J1" H 1500 1975 50  0000 C CNN
F 1 "Conn_02x10_Odd_Even" H 1500 1976 50  0001 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_2x10_P2.54mm_Vertical" H 1450 1450 50  0001 C CNN
F 3 "~" H 1450 1450 50  0001 C CNN
	1    1450 1450
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_02x10_Odd_Even J2
U 1 1 61116495
P 3450 1450
F 0 "J2" H 3500 1975 50  0000 C CNN
F 1 "Conn_02x10_Odd_Even" H 3500 1976 50  0001 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_2x10_P2.54mm_Vertical" H 3450 1450 50  0001 C CNN
F 3 "~" H 3450 1450 50  0001 C CNN
	1    3450 1450
	1    0    0    -1  
$EndComp
Text Label 700  1050 0    50   ~ 0
V3P3
Text Label 700  1150 0    50   ~ 0
P6.0
Text Label 700  1250 0    50   ~ 0
P3.2
Text Label 700  1350 0    50   ~ 0
P3.3
Text Label 700  1450 0    50   ~ 0
P4.1
Text Label 700  1550 0    50   ~ 0
P4.3
Text Label 700  1650 0    50   ~ 0
P1.5
Text Label 700  1750 0    50   ~ 0
P4.6
Text Label 700  1850 0    50   ~ 0
P6.5
Text Label 700  1950 0    50   ~ 0
P6.4
Text Label 2700 1050 0    50   ~ 0
P2.7
Text Label 2700 1150 0    50   ~ 0
P2.6
Text Label 2700 1250 0    50   ~ 0
P2.4
Text Label 2700 1350 0    50   ~ 0
MCU_RW
Text Label 2700 1450 0    50   ~ 0
MCU_D0
Text Label 2700 1550 0    50   ~ 0
MCU_D1
Text Label 2700 1650 0    50   ~ 0
MCU_D2
Text Label 2700 1750 0    50   ~ 0
MCU_D3
Text Label 2700 1850 0    50   ~ 0
MCU_D5
Text Label 2700 1950 0    50   ~ 0
MCU_D7
Text Label 2300 1050 2    50   ~ 0
V5P0
Text Label 2300 1150 2    50   ~ 0
GND
Text Label 2300 1250 2    50   ~ 0
FUEL_ADC
Text Label 2300 1350 2    50   ~ 0
P4.0
Text Label 2300 1450 2    50   ~ 0
SPEED_A
Text Label 2300 1550 2    50   ~ 0
P4.4
Text Label 2300 1650 2    50   ~ 0
P4.5
Text Label 2300 1750 2    50   ~ 0
P4.7
Text Label 2300 1850 2    50   ~ 0
P5.4
Text Label 2300 1950 2    50   ~ 0
P5.5
Wire Wire Line
	700  1050 1250 1050
Wire Wire Line
	1250 1150 700  1150
Wire Wire Line
	700  1250 1250 1250
Wire Wire Line
	1250 1350 700  1350
Wire Wire Line
	700  1450 1250 1450
Wire Wire Line
	1250 1550 700  1550
Wire Wire Line
	700  1650 1250 1650
Wire Wire Line
	1250 1750 700  1750
Wire Wire Line
	700  1850 1250 1850
Wire Wire Line
	1250 1950 700  1950
Wire Wire Line
	2300 1950 1750 1950
Wire Wire Line
	1750 1850 2300 1850
Wire Wire Line
	2300 1750 1750 1750
Wire Wire Line
	1750 1650 2300 1650
Wire Wire Line
	2300 1550 1750 1550
Wire Wire Line
	1750 1450 2300 1450
Wire Wire Line
	2300 1350 1750 1350
Wire Wire Line
	1750 1250 2300 1250
Wire Wire Line
	2300 1150 1750 1150
Wire Wire Line
	1750 1050 2300 1050
Wire Wire Line
	2700 1050 3250 1050
Wire Wire Line
	3250 1150 2700 1150
Wire Wire Line
	2700 1250 3250 1250
Wire Wire Line
	3250 1350 2700 1350
Wire Wire Line
	2700 1450 3250 1450
Wire Wire Line
	3250 1550 2700 1550
Wire Wire Line
	2700 1650 3250 1650
Wire Wire Line
	3250 1750 2700 1750
Wire Wire Line
	2700 1850 3250 1850
Wire Wire Line
	3250 1950 2700 1950
Text Label 4300 1050 2    50   ~ 0
GND
Text Label 4300 1150 2    50   ~ 0
P2.5
Text Label 4300 1250 2    50   ~ 0
MCU_RS
Text Label 4300 1350 2    50   ~ 0
MCU_EN
Text Label 4300 1450 2    50   ~ 0
RST
Text Label 4300 1550 2    50   ~ 0
I2C_SDA
Text Label 4300 1650 2    50   ~ 0
I2C_SCL
Text Label 4300 1750 2    50   ~ 0
MCU_D4
Text Label 4300 1850 2    50   ~ 0
MCU_D6
Text Label 4300 1950 2    50   ~ 0
MCU_BLA
Wire Wire Line
	4300 1950 3750 1950
Wire Wire Line
	3750 1850 4300 1850
Wire Wire Line
	4300 1750 3750 1750
Wire Wire Line
	3750 1650 4300 1650
Wire Wire Line
	4300 1550 3750 1550
Wire Wire Line
	3750 1450 4300 1450
Wire Wire Line
	4300 1350 3750 1350
Wire Wire Line
	3750 1250 4300 1250
Wire Wire Line
	4300 1150 3750 1150
Wire Wire Line
	3750 1050 4300 1050
Wire Notes Line width 10 style solid
	550  2500 4500 2500
$Comp
L MySymbols:1602A U1
U 1 1 61157068
P 1250 3800
F 0 "U1" H 1200 4700 50  0000 L CNN
F 1 "1602A" H 1050 3800 50  0000 L CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x16_P2.54mm_Vertical" H 1300 4300 50  0001 C CNN
F 3 "" H 1300 4300 50  0001 C CNN
	1    1250 3800
	1    0    0    -1  
$EndComp
Text Label 2000 3050 2    50   ~ 0
GND
Text Label 2000 3150 2    50   ~ 0
V5P0
Text Label 2000 3350 2    50   ~ 0
MCU_RS
Text Label 2000 3450 2    50   ~ 0
MCU_RW
Text Label 2000 3550 2    50   ~ 0
MCU_EN
Text Label 2000 3650 2    50   ~ 0
MCU_D0
Text Label 2000 3750 2    50   ~ 0
MCU_D1
Text Label 2000 3850 2    50   ~ 0
MCU_D2
Text Label 2000 4050 2    50   ~ 0
MCU_D4
Text Label 2000 3950 2    50   ~ 0
MCU_D3
Text Label 2000 4150 2    50   ~ 0
MCU_D5
Text Label 2000 4250 2    50   ~ 0
MCU_D6
Text Label 2000 4350 2    50   ~ 0
MCU_D7
Text Label 2000 4550 2    50   ~ 0
GND
Wire Wire Line
	2000 4550 1600 4550
Wire Wire Line
	2000 4350 1600 4350
Wire Wire Line
	1600 4250 2000 4250
Wire Wire Line
	2000 4150 1600 4150
Wire Wire Line
	1600 4050 2000 4050
Wire Wire Line
	2000 3950 1600 3950
Wire Wire Line
	1600 3850 2000 3850
Wire Wire Line
	2000 3750 1600 3750
Wire Wire Line
	1600 3650 2000 3650
Wire Wire Line
	2000 3550 1600 3550
Wire Wire Line
	1600 3450 2000 3450
Wire Wire Line
	2000 3350 1600 3350
Wire Wire Line
	1600 3150 2000 3150
Wire Wire Line
	2000 3050 1600 3050
$Comp
L Device:R_US R1
U 1 1 6117250C
P 2250 3000
F 0 "R1" H 2182 2954 50  0000 R CNN
F 1 "10k" H 2182 3045 50  0000 R CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 2290 2990 50  0001 C CNN
F 3 "~" H 2250 3000 50  0001 C CNN
	1    2250 3000
	-1   0    0    1   
$EndComp
$Comp
L Device:R_US R2
U 1 1 61173396
P 2250 3500
F 0 "R2" H 2182 3454 50  0000 R CNN
F 1 "10k" H 2182 3545 50  0000 R CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 2290 3490 50  0001 C CNN
F 3 "~" H 2250 3500 50  0001 C CNN
	1    2250 3500
	-1   0    0    1   
$EndComp
Wire Wire Line
	2250 3350 2250 3250
Wire Wire Line
	2250 3250 1600 3250
Connection ~ 2250 3250
Wire Wire Line
	2250 3250 2250 3150
Text Label 2500 3750 2    50   ~ 0
GND
Text Label 2500 2750 2    50   ~ 0
V5P0
Wire Wire Line
	2500 2750 2250 2750
Wire Wire Line
	2250 2750 2250 2850
Wire Wire Line
	2500 3750 2250 3750
Wire Wire Line
	2250 3750 2250 3650
$Comp
L Device:R_US R3
U 1 1 6117C699
P 2250 4450
F 0 "R3" V 2455 4450 50  0000 C CNN
F 1 "200R" V 2364 4450 50  0000 C CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 2290 4440 50  0001 C CNN
F 3 "~" H 2250 4450 50  0001 C CNN
	1    2250 4450
	0    -1   -1   0   
$EndComp
Wire Wire Line
	2100 4450 1600 4450
$Comp
L Device:Q_PMOS_SGD Q1
U 1 1 61181A5A
P 2600 4550
F 0 "Q1" V 2851 4550 50  0000 C CNN
F 1 "Q_PMOS_SGD" V 2851 4550 50  0001 C CNN
F 2 "Package_TO_SOT_SMD:SOT-23" H 2800 4650 50  0001 C CNN
F 3 "~" H 2600 4550 50  0001 C CNN
	1    2600 4550
	0    -1   -1   0   
$EndComp
Text Label 3400 4350 2    50   ~ 0
V5P0
Wire Wire Line
	3400 4350 3000 4350
Wire Wire Line
	2800 4350 2800 4450
Text Label 3400 4850 2    50   ~ 0
MCU_BLA
Wire Wire Line
	3400 4850 3000 4850
Wire Wire Line
	2600 4850 2600 4750
$Comp
L Device:R_US R4
U 1 1 611923B3
P 3000 4600
F 0 "R4" H 2932 4554 50  0000 R CNN
F 1 "100k" H 2932 4645 50  0000 R CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 3040 4590 50  0001 C CNN
F 3 "~" H 3000 4600 50  0001 C CNN
	1    3000 4600
	-1   0    0    1   
$EndComp
Wire Wire Line
	3000 4450 3000 4350
Connection ~ 3000 4350
Wire Wire Line
	3000 4350 2800 4350
Wire Wire Line
	3000 4750 3000 4850
Connection ~ 3000 4850
Wire Wire Line
	3000 4850 2600 4850
Wire Notes Line width 10 style solid
	550  5000 4500 5000
Wire Notes Line width 10 style solid
	4500 500  4500 5000
$Comp
L MySymbols:HT16K33-Backpack U2
U 1 1 61165293
P 1100 6000
F 0 "U2" H 1183 6465 50  0000 C CNN
F 1 "HT16K33-Backpack" H 1183 6374 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x05_P2.54mm_Vertical" H 1400 6000 50  0001 C CNN
F 3 "" H 1400 6000 50  0001 C CNN
	1    1100 6000
	1    0    0    -1  
$EndComp
$Comp
L Device:R_US R6
U 1 1 61167DB3
P 2500 5750
F 0 "R6" H 2432 5704 50  0000 R CNN
F 1 "4.7k" H 2432 5795 50  0000 R CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 2540 5740 50  0001 C CNN
F 3 "~" H 2500 5750 50  0001 C CNN
	1    2500 5750
	-1   0    0    1   
$EndComp
$Comp
L Device:R_US R5
U 1 1 6116A753
P 2000 5750
F 0 "R5" H 1932 5704 50  0000 R CNN
F 1 "4.7k" H 1932 5795 50  0000 R CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 2040 5740 50  0001 C CNN
F 3 "~" H 2000 5750 50  0001 C CNN
	1    2000 5750
	-1   0    0    1   
$EndComp
Wire Wire Line
	2000 5900 2000 6100
Wire Wire Line
	2000 6100 1400 6100
Wire Wire Line
	2500 5900 2500 6200
Wire Wire Line
	2500 6200 1400 6200
Text Label 1750 6000 2    50   ~ 0
GND
Wire Wire Line
	1750 6000 1400 6000
Wire Wire Line
	1750 5900 1400 5900
Text Label 1750 5900 2    50   ~ 0
V5P0
Wire Wire Line
	1400 5800 1650 5800
Wire Wire Line
	1650 5800 1650 5500
Wire Wire Line
	1650 5500 2000 5500
Wire Wire Line
	2500 5500 2500 5600
Wire Wire Line
	2000 5600 2000 5500
Connection ~ 2000 5500
Wire Wire Line
	2000 5500 2500 5500
Text Label 1750 5500 0    50   ~ 0
V3P3
Text Label 1750 6100 2    50   ~ 0
I2C_SDA
Text Label 1750 6200 2    50   ~ 0
I2C_SCL
$EndSCHEMATC
