EESchema Schematic File Version 4
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
L Connector_Generic:Conn_01x05_Female J1
U 1 1 5AE8B0F8
P 10300 1550
F 0 "J1" H 10327 1576 50  0000 L CNN
F 1 "Conn_01x05_Female" H 10327 1485 50  0000 L CNN
F 2 "" H 10300 1550 50  0001 C CNN
F 3 "~" H 10300 1550 50  0001 C CNN
	1    10300 1550
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_01x02_Female J2
U 1 1 5AE8B17F
P 10300 2100
F 0 "J2" H 10327 2076 50  0000 L CNN
F 1 "Conn_01x02_Female" H 10327 1985 50  0000 L CNN
F 2 "" H 10300 2100 50  0001 C CNN
F 3 "~" H 10300 2100 50  0001 C CNN
	1    10300 2100
	1    0    0    -1  
$EndComp
Text GLabel 10100 1450 0    50   Input ~ 0
SWCLK
Text GLabel 10100 1650 0    50   Input ~ 0
SWDIO
Text GLabel 10100 1750 0    50   Input ~ 0
NRST
Text GLabel 10100 2100 0    50   Input ~ 0
Button
Text GLabel 10100 2200 0    50   Input ~ 0
Buzzer
Text GLabel 2800 2550 0    50   Input ~ 0
NRST
NoConn ~ 2800 3250
NoConn ~ 2800 3350
NoConn ~ 2800 3550
NoConn ~ 2800 3650
NoConn ~ 8600 2650
NoConn ~ 8600 2750
NoConn ~ 8600 2850
NoConn ~ 8600 2950
NoConn ~ 8600 3050
NoConn ~ 8600 3150
NoConn ~ 8600 3350
NoConn ~ 8600 3450
Text GLabel 8600 2550 2    50   Input ~ 0
Button
Text GLabel 8600 3250 2    50   Input ~ 0
Buzzer
Text GLabel 8600 3550 2    50   Input ~ 0
SWDIO
Text GLabel 8600 3650 2    50   Input ~ 0
SWCLK
$Comp
L MCU_ST_STM32:STM32F042F4Px U1
U 1 1 5AE8A70D
P 5700 3150
F 0 "U1" H 4650 3400 50  0000 C CNN
F 1 "STM32F042F4Px" H 5700 1900 50  0000 C CNN
F 2 "Package_SO:TSSOP-20_4.4x6.5mm_P0.65mm" H 8500 4025 50  0001 R TNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00105814.pdf" H 5700 3150 50  0001 C CNN
	1    5700 3150
	1    0    0    -1  
$EndComp
Text GLabel 5700 4050 3    50   Input ~ 0
GND
Text GLabel 10000 2700 0    50   Input ~ 0
GND
$Comp
L power:PWR_FLAG #FLG0103
U 1 1 5AE8CCD3
P 10000 2700
F 0 "#FLG0103" H 10000 2775 50  0001 C CNN
F 1 "PWR_FLAG" V 10000 2828 50  0000 L CNN
F 2 "" H 10000 2700 50  0001 C CNN
F 3 "" H 10000 2700 50  0001 C CNN
	1    10000 2700
	0    1    1    0   
$EndComp
Text GLabel 10100 1550 0    50   Input ~ 0
GND
Wire Wire Line
	5600 2150 5650 2150
Wire Wire Line
	5650 2150 5650 2000
Connection ~ 5650 2150
Wire Wire Line
	5650 2150 5700 2150
Text GLabel 5650 2000 1    50   Input ~ 0
VDD
Text GLabel 10000 2850 0    50   Input ~ 0
VDD
$Comp
L power:PWR_FLAG #FLG0101
U 1 1 5AE8CF34
P 10000 2850
F 0 "#FLG0101" H 10000 2925 50  0001 C CNN
F 1 "PWR_FLAG" V 10000 2978 50  0000 L CNN
F 2 "" H 10000 2850 50  0001 C CNN
F 3 "" H 10000 2850 50  0001 C CNN
	1    10000 2850
	0    1    1    0   
$EndComp
Text GLabel 10100 1350 0    50   Input ~ 0
VDD
$EndSCHEMATC