EESchema Schematic File Version 4
LIBS:reference-project-cache
EELAYER 26 0
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
L Switch:SW_Push SW1
U 1 1 59E49B79
P 9150 5300
F 0 "SW1" H 8950 5400 50  0000 L CNN
F 1 "RESET" H 9150 5240 50  0000 C CNN
F 2 "Buttons_Switches_SMD:SW_SPST_TL3342" H 9150 5500 50  0001 C CNN
F 3 "" H 9150 5500 50  0001 C CNN
	1    9150 5300
	1    0    0    -1  
$EndComp
$Comp
L Switch:SW_Push SW2
U 1 1 59E49BEA
P 9800 5300
F 0 "SW2" H 9850 5400 50  0000 L CNN
F 1 "INPUT_5" H 9800 5240 50  0000 C CNN
F 2 "Buttons_Switches_SMD:SW_SPST_TL3342" H 9800 5500 50  0001 C CNN
F 3 "" H 9800 5500 50  0001 C CNN
	1    9800 5300
	1    0    0    -1  
$EndComp
$Comp
L reference-project-rescue:R R2
U 1 1 59E53A35
P 8850 4650
F 0 "R2" V 8930 4650 50  0000 C CNN
F 1 "10K" V 8850 4650 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 8780 4650 50  0001 C CNN
F 3 "" H 8850 4650 50  0001 C CNN
	1    8850 4650
	1    0    0    -1  
$EndComp
$Comp
L reference-project-rescue:R R3
U 1 1 59E56D49
P 9800 4000
F 0 "R3" V 9880 4000 50  0000 C CNN
F 1 "220" V 9800 4000 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 9730 4000 50  0001 C CNN
F 3 "" H 9800 4000 50  0001 C CNN
	1    9800 4000
	0    1    1    0   
$EndComp
$Comp
L reference-project-rescue:R R1
U 1 1 59E587BF
P 8850 3600
F 0 "R1" V 8930 3600 50  0000 C CNN
F 1 "10K" V 8850 3600 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 8780 3600 50  0001 C CNN
F 3 "" H 8850 3600 50  0001 C CNN
	1    8850 3600
	1    0    0    -1  
$EndComp
$Comp
L power:PWR_FLAG #FLG01
U 1 1 59E6375F
P 7850 2250
F 0 "#FLG01" H 7850 2325 50  0001 C CNN
F 1 "PWR_FLAG" H 7850 2400 50  0000 C CNN
F 2 "" H 7850 2250 50  0001 C CNN
F 3 "" H 7850 2250 50  0001 C CNN
	1    7850 2250
	-1   0    0    1   
$EndComp
$Comp
L reference-project-rescue:ATTINY85-20SU U1
U 1 1 59FA3237
P 7400 4150
F 0 "U1" H 7400 4667 50  0000 C CNN
F 1 "ATTINY85-20SU" H 7400 4576 50  0000 C CNN
F 2 "Housings_SOIC:SOIC-8_3.9x4.9mm_Pitch1.27mm" H 8350 4150 50  0001 C CIN
F 3 "http://www.atmel.com/images/atmel-2586-avr-8-bit-microcontroller-attiny25-attiny45-attiny85_datasheet.pdf" H 7400 4150 50  0001 C CNN
	1    7400 4150
	-1   0    0    -1  
$EndComp
$Comp
L reference-project-rescue:Conn_01x08_Female J1
U 1 1 59FCBF1A
P 9100 1850
F 0 "J1" V 9265 1780 50  0000 C CNN
F 1 "Conn_01x08_Female" V 9174 1780 50  0000 C CNN
F 2 "conn-100mil:CONN-100MIL-F-1x8" H 9100 1850 50  0001 C CNN
F 3 "~" H 9100 1850 50  0001 C CNN
	1    9100 1850
	0    -1   -1   0   
$EndComp
Wire Wire Line
	9950 4000 10350 4000
Wire Wire Line
	8750 4000 9250 4000
Wire Wire Line
	8750 4200 8850 4200
Wire Wire Line
	8850 4200 8850 3750
Wire Wire Line
	8850 3000 8850 3450
Wire Wire Line
	6050 3000 6050 3900
Wire Wire Line
	8750 4300 8850 4300
Wire Wire Line
	8850 4300 8850 4500
Wire Wire Line
	8850 4800 8850 5300
Connection ~ 8850 4200
Wire Wire Line
	9350 4200 9350 5300
Wire Wire Line
	9600 4300 9600 5300
Connection ~ 8850 4300
Wire Wire Line
	10000 3000 10000 5300
Wire Wire Line
	6050 3000 8850 3000
Wire Wire Line
	8750 4400 9050 4400
Wire Wire Line
	8750 3900 9150 3900
Wire Wire Line
	8750 4100 9350 4100
Wire Wire Line
	9400 2050 9400 2400
Wire Wire Line
	9300 2050 9300 2400
Text GLabel 9300 2400 3    60   Input ~ 0
SPKR-
Wire Wire Line
	9200 2050 9200 2400
Text GLabel 9200 2400 3    60   Input ~ 0
SPKR+
Text GLabel 10350 3750 1    60   Input ~ 0
SPKR+
Text GLabel 9400 2400 3    60   Input ~ 0
VCC
Text GLabel 9500 2400 3    60   Input ~ 0
GND
Wire Wire Line
	9500 2400 9500 2050
Text GLabel 8800 2400 3    60   Input ~ 0
PIN_1
Text GLabel 9000 2400 3    60   Input ~ 0
PIN_5
Text GLabel 9100 2400 3    60   Input ~ 0
PIN_6
Text GLabel 8900 2400 3    60   Input ~ 0
PIN_7
Wire Wire Line
	9100 2050 9100 2400
Wire Wire Line
	9000 2050 9000 2400
Wire Wire Line
	8900 2050 8900 2400
Wire Wire Line
	8800 2050 8800 2400
Wire Wire Line
	9050 4400 9050 3750
Text GLabel 9050 3750 1    60   Input ~ 0
PIN_1
Text GLabel 9150 3750 1    60   Input ~ 0
PIN_5
Wire Wire Line
	9150 3900 9150 3750
Text GLabel 9250 3750 1    60   Input ~ 0
PIN_6
Wire Wire Line
	9250 4000 9250 3750
Connection ~ 9250 4000
Text GLabel 9350 3750 1    60   Input ~ 0
PIN_7
Wire Wire Line
	9350 4100 9350 3750
Connection ~ 8850 3000
Wire Wire Line
	6050 5300 8850 5300
Wire Wire Line
	6050 4400 6050 5300
Connection ~ 8850 5300
Text GLabel 6050 5300 0    60   Input ~ 0
GND
Text GLabel 6050 3000 0    60   Input ~ 0
VCC
$Comp
L power:GND #PWR01
U 1 1 59FCE2B5
P 10600 3750
F 0 "#PWR01" H 10600 3500 50  0001 C CNN
F 1 "GND" H 10605 3577 50  0000 C CNN
F 2 "" H 10600 3750 50  0001 C CNN
F 3 "" H 10600 3750 50  0001 C CNN
	1    10600 3750
	1    0    0    -1  
$EndComp
Text GLabel 10600 3750 1    60   Input ~ 0
SPKR-
Text GLabel 7850 2250 1    60   Input ~ 0
VCC
$Comp
L power:GND #PWR06
U 1 1 59E63687
P 8300 2250
F 0 "#PWR06" H 8300 2000 50  0001 C CNN
F 1 "GND" H 8300 2100 50  0000 C CNN
F 2 "" H 8300 2250 50  0001 C CNN
F 3 "" H 8300 2250 50  0001 C CNN
	1    8300 2250
	1    0    0    -1  
$EndComp
$Comp
L power:PWR_FLAG #FLG02
U 1 1 59FCE95A
P 8300 2250
F 0 "#FLG02" H 8300 2325 50  0001 C CNN
F 1 "PWR_FLAG" H 8300 2424 50  0000 C CNN
F 2 "" H 8300 2250 50  0001 C CNN
F 3 "" H 8300 2250 50  0001 C CNN
	1    8300 2250
	1    0    0    -1  
$EndComp
Wire Wire Line
	10350 4000 10350 3750
Wire Wire Line
	8850 4200 9350 4200
Wire Wire Line
	8850 4300 9600 4300
Wire Wire Line
	9250 4000 9650 4000
Wire Wire Line
	8850 3000 10000 3000
Wire Wire Line
	8850 5300 8950 5300
$EndSCHEMATC
