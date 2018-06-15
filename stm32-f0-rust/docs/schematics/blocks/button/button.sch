EESchema Schematic File Version 4
LIBS:button-cache
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
U 1 1 5B1C43FE
P 4150 2350
F 0 "SW1" H 4150 2635 50  0000 C CNN
F 1 "SW_Push" H 4150 2544 50  0000 C CNN
F 2 "Button_Switch_SMD:SW_SPST_TL3342" H 4150 2550 50  0001 C CNN
F 3 "" H 4150 2550 50  0001 C CNN
	1    4150 2350
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_01x02_Female J1
U 1 1 5B1C451E
P 4200 2950
F 0 "J1" V 4047 2998 50  0000 L CNN
F 1 "Conn_01x02_Female" V 4138 2998 50  0000 L CNN
F 2 "Connector_PinSocket_2.54mm:PinSocket_1x02_P2.54mm_Vertical" H 4200 2950 50  0001 C CNN
F 3 "~" H 4200 2950 50  0001 C CNN
	1    4200 2950
	0    1    1    0   
$EndComp
Wire Wire Line
	3950 2350 3950 2750
Wire Wire Line
	3950 2750 4100 2750
Wire Wire Line
	4350 2350 4350 2750
Wire Wire Line
	4350 2750 4200 2750
$EndSCHEMATC
