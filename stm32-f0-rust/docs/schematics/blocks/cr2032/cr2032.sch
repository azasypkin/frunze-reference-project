EESchema Schematic File Version 4
LIBS:cr2032-cache
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
L Device:Battery_Cell BT1
U 1 1 5B0C5305
P 4200 1600
F 0 "BT1" H 4318 1696 50  0000 L CNN
F 1 "Battery_Cell" H 4318 1605 50  0000 L CNN
F 2 "" V 4200 1660 50  0001 C CNN
F 3 "~" V 4200 1660 50  0001 C CNN
	1    4200 1600
	1    0    0    -1  
$EndComp
Wire Wire Line
	4200 1400 5250 1400
Wire Wire Line
	5250 1400 5250 1500
Wire Wire Line
	4200 1700 5250 1700
Wire Wire Line
	5250 1700 5250 1600
$Comp
L Connector_Generic:Conn_01x03_Female J1
U 1 1 5B1574DD
P 6400 1300
F 0 "J1" H 6427 1326 50  0000 L CNN
F 1 "Conn_01x03_Female" H 6427 1235 50  0000 L CNN
F 2 "Connector_PinSocket_2.54mm:PinSocket_1x03_P2.54mm_Vertical" H 6400 1300 50  0001 C CNN
F 3 "~" H 6400 1300 50  0001 C CNN
	1    6400 1300
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_01x03_Female J2
U 1 1 5B157552
P 6400 1750
F 0 "J2" H 6427 1776 50  0000 L CNN
F 1 "Conn_01x03_Female" H 6427 1685 50  0000 L CNN
F 2 "Connector_PinSocket_2.54mm:PinSocket_1x03_P2.54mm_Vertical" H 6400 1750 50  0001 C CNN
F 3 "~" H 6400 1750 50  0001 C CNN
	1    6400 1750
	1    0    0    -1  
$EndComp
Wire Wire Line
	5250 1500 6200 1500
Wire Wire Line
	6200 1500 6200 1400
Wire Wire Line
	6200 1200 6200 1300
Connection ~ 6200 1400
Connection ~ 6200 1300
Wire Wire Line
	6200 1300 6200 1400
Wire Wire Line
	5250 1600 6200 1600
Wire Wire Line
	6200 1600 6200 1650
Wire Wire Line
	6200 1650 6200 1750
Connection ~ 6200 1650
Connection ~ 6200 1750
Wire Wire Line
	6200 1750 6200 1850
$EndSCHEMATC
