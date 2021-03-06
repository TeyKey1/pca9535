EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title "PCA9535 Testbench"
Date "2021-12-22"
Rev "0"
Comp ""
Comment1 "Thierry Kühni (TeyKey1)"
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L custom:PCA9535CPW,118 U?
U 1 1 61728630
P 8400 2500
F 0 "U?" H 8400 3670 50  0000 C CNN
F 1 "PCA9535CPW,118" H 8400 3579 50  0000 C CNN
F 2 "SOP65P640X110-24N" H 8400 2500 50  0001 L BNN
F 3 "" H 8400 2500 50  0001 L BNN
F 4 "70R6541" H 8400 2500 50  0001 L BNN "OC_NEWARK"
F 5 "TSSOP-24" H 8400 2500 50  0001 L BNN "PACKAGE"
F 6 "PCA9535CPW,118" H 8400 2500 50  0001 L BNN "MPN"
F 7 "NXP" H 8400 2500 50  0001 L BNN "SUPPLIER"
F 8 "-" H 8400 2500 50  0001 L BNN "OC_FARNELL"
	1    8400 2500
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 6172A36B
P 7600 3650
F 0 "#PWR?" H 7600 3400 50  0001 C CNN
F 1 "GND" H 7605 3477 50  0000 C CNN
F 2 "" H 7600 3650 50  0001 C CNN
F 3 "" H 7600 3650 50  0001 C CNN
	1    7600 3650
	1    0    0    -1  
$EndComp
Wire Wire Line
	7600 3650 7600 3500
$Comp
L power:+3V3 #PWR?
U 1 1 6172AB8A
P 7450 1600
F 0 "#PWR?" H 7450 1450 50  0001 C CNN
F 1 "+3V3" H 7465 1773 50  0000 C CNN
F 2 "" H 7450 1600 50  0001 C CNN
F 3 "" H 7450 1600 50  0001 C CNN
	1    7450 1600
	1    0    0    -1  
$EndComp
Wire Wire Line
	7700 1700 7450 1700
Wire Wire Line
	7450 1700 7450 1600
$Comp
L Device:R_Small R?
U 1 1 6172B921
P 7300 3700
F 0 "R?" V 7250 3800 50  0000 L CNN
F 1 "100k" V 7250 3400 50  0000 L CNN
F 2 "" H 7300 3700 50  0001 C CNN
F 3 "~" H 7300 3700 50  0001 C CNN
	1    7300 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 6172C64B
P 7150 3700
F 0 "R?" V 7100 3800 50  0000 L CNN
F 1 "100k" V 7100 3400 50  0000 L CNN
F 2 "" H 7150 3700 50  0001 C CNN
F 3 "~" H 7150 3700 50  0001 C CNN
	1    7150 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 6172C9FF
P 7000 3700
F 0 "R?" V 6950 3800 50  0000 L CNN
F 1 "100k" V 6950 3400 50  0000 L CNN
F 2 "" H 7000 3700 50  0001 C CNN
F 3 "~" H 7000 3700 50  0001 C CNN
	1    7000 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 6172CBD8
P 6850 3700
F 0 "R?" V 6800 3800 50  0000 L CNN
F 1 "100k" V 6800 3400 50  0000 L CNN
F 2 "" H 6850 3700 50  0001 C CNN
F 3 "~" H 6850 3700 50  0001 C CNN
	1    6850 3700
	1    0    0    -1  
$EndComp
Wire Wire Line
	7700 2600 6850 2600
Wire Wire Line
	7700 2700 7000 2700
Wire Wire Line
	7700 2800 7150 2800
Wire Wire Line
	7700 2900 7300 2900
Wire Wire Line
	7300 3600 7300 2900
Wire Wire Line
	7150 3600 7150 2800
Wire Wire Line
	7000 3600 7000 2700
Wire Wire Line
	6850 3600 6850 2600
$Comp
L power:+3V3 #PWR?
U 1 1 6172D9FE
P 6850 4200
F 0 "#PWR?" H 6850 4050 50  0001 C CNN
F 1 "+3V3" H 6865 4373 50  0000 C CNN
F 2 "" H 6850 4200 50  0001 C CNN
F 3 "" H 6850 4200 50  0001 C CNN
	1    6850 4200
	-1   0    0    1   
$EndComp
Wire Wire Line
	6850 3800 6850 4100
Wire Wire Line
	7000 3800 7000 4100
Wire Wire Line
	7000 4100 6850 4100
Connection ~ 6850 4100
Wire Wire Line
	6850 4100 6850 4200
Wire Wire Line
	7150 3800 7150 4100
Wire Wire Line
	7150 4100 7000 4100
Connection ~ 7000 4100
Wire Wire Line
	7300 3800 7300 4100
Wire Wire Line
	7300 4100 7150 4100
Connection ~ 7150 4100
Wire Wire Line
	9100 2300 9350 2300
Wire Wire Line
	9100 2400 9500 2400
Wire Wire Line
	9100 2500 9650 2500
Wire Wire Line
	9100 2600 9800 2600
$Comp
L Device:R_Small R?
U 1 1 6173189F
P 9800 3700
F 0 "R?" V 9750 3800 50  0000 L CNN
F 1 "100k" V 9750 3400 50  0000 L CNN
F 2 "" H 9800 3700 50  0001 C CNN
F 3 "~" H 9800 3700 50  0001 C CNN
	1    9800 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 617318A5
P 9650 3700
F 0 "R?" V 9600 3800 50  0000 L CNN
F 1 "100k" V 9600 3400 50  0000 L CNN
F 2 "" H 9650 3700 50  0001 C CNN
F 3 "~" H 9650 3700 50  0001 C CNN
	1    9650 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 617318AB
P 9500 3700
F 0 "R?" V 9450 3800 50  0000 L CNN
F 1 "100k" V 9450 3400 50  0000 L CNN
F 2 "" H 9500 3700 50  0001 C CNN
F 3 "~" H 9500 3700 50  0001 C CNN
	1    9500 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 617318B1
P 9350 3700
F 0 "R?" V 9300 3800 50  0000 L CNN
F 1 "100k" V 9300 3400 50  0000 L CNN
F 2 "" H 9350 3700 50  0001 C CNN
F 3 "~" H 9350 3700 50  0001 C CNN
	1    9350 3700
	1    0    0    -1  
$EndComp
$Comp
L power:+3V3 #PWR?
U 1 1 617318B7
P 9350 4200
F 0 "#PWR?" H 9350 4050 50  0001 C CNN
F 1 "+3V3" H 9365 4373 50  0000 C CNN
F 2 "" H 9350 4200 50  0001 C CNN
F 3 "" H 9350 4200 50  0001 C CNN
	1    9350 4200
	-1   0    0    1   
$EndComp
Wire Wire Line
	9350 3800 9350 4100
Wire Wire Line
	9500 3800 9500 4100
Wire Wire Line
	9500 4100 9350 4100
Connection ~ 9350 4100
Wire Wire Line
	9350 4100 9350 4200
Wire Wire Line
	9650 3800 9650 4100
Wire Wire Line
	9650 4100 9500 4100
Connection ~ 9500 4100
Wire Wire Line
	9800 3800 9800 4100
Wire Wire Line
	9800 4100 9650 4100
Connection ~ 9650 4100
Wire Wire Line
	9350 3600 9350 2300
Connection ~ 9350 2300
Wire Wire Line
	9500 3600 9500 2400
Connection ~ 9500 2400
Wire Wire Line
	9650 3600 9650 2500
Connection ~ 9650 2500
Wire Wire Line
	9800 3600 9800 2600
Connection ~ 9800 2600
Wire Wire Line
	9100 2200 10550 2200
Wire Wire Line
	9100 2100 10400 2100
Wire Wire Line
	9100 2000 10250 2000
Wire Wire Line
	9100 1900 10100 1900
$Comp
L Device:R_Small R?
U 1 1 61736F55
P 10550 3700
F 0 "R?" V 10500 3800 50  0000 L CNN
F 1 "100k" V 10500 3400 50  0000 L CNN
F 2 "" H 10550 3700 50  0001 C CNN
F 3 "~" H 10550 3700 50  0001 C CNN
	1    10550 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 61736F5B
P 10400 3700
F 0 "R?" V 10350 3800 50  0000 L CNN
F 1 "100k" V 10350 3400 50  0000 L CNN
F 2 "" H 10400 3700 50  0001 C CNN
F 3 "~" H 10400 3700 50  0001 C CNN
	1    10400 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 61736F61
P 10250 3700
F 0 "R?" V 10200 3800 50  0000 L CNN
F 1 "100k" V 10200 3400 50  0000 L CNN
F 2 "" H 10250 3700 50  0001 C CNN
F 3 "~" H 10250 3700 50  0001 C CNN
	1    10250 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 61736F67
P 10100 3700
F 0 "R?" V 10050 3800 50  0000 L CNN
F 1 "100k" V 10050 3400 50  0000 L CNN
F 2 "" H 10100 3700 50  0001 C CNN
F 3 "~" H 10100 3700 50  0001 C CNN
	1    10100 3700
	1    0    0    -1  
$EndComp
Wire Wire Line
	10100 3800 10100 4100
Wire Wire Line
	10250 3800 10250 4100
Wire Wire Line
	10250 4100 10100 4100
Connection ~ 10100 4100
Wire Wire Line
	10100 4100 10100 4200
Wire Wire Line
	10400 3800 10400 4100
Wire Wire Line
	10400 4100 10250 4100
Connection ~ 10250 4100
Wire Wire Line
	10550 3800 10550 4100
Wire Wire Line
	10550 4100 10400 4100
Connection ~ 10400 4100
$Comp
L power:GND #PWR?
U 1 1 61739384
P 10100 4200
F 0 "#PWR?" H 10100 3950 50  0001 C CNN
F 1 "GND" H 10105 4027 50  0000 C CNN
F 2 "" H 10100 4200 50  0001 C CNN
F 3 "" H 10100 4200 50  0001 C CNN
	1    10100 4200
	1    0    0    -1  
$EndComp
Wire Wire Line
	10100 3600 10100 1900
Wire Wire Line
	10250 3600 10250 2000
Wire Wire Line
	10400 3600 10400 2100
Connection ~ 10100 1900
Connection ~ 10250 2000
Connection ~ 10400 2100
Wire Wire Line
	10550 3600 10550 2200
Connection ~ 10550 2200
$Comp
L Device:R_Small R?
U 1 1 617404F9
P 6600 3700
F 0 "R?" V 6550 3800 50  0000 L CNN
F 1 "100k" V 6550 3400 50  0000 L CNN
F 2 "" H 6600 3700 50  0001 C CNN
F 3 "~" H 6600 3700 50  0001 C CNN
	1    6600 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 617404FF
P 6450 3700
F 0 "R?" V 6400 3800 50  0000 L CNN
F 1 "100k" V 6400 3400 50  0000 L CNN
F 2 "" H 6450 3700 50  0001 C CNN
F 3 "~" H 6450 3700 50  0001 C CNN
	1    6450 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 61740505
P 6300 3700
F 0 "R?" V 6250 3800 50  0000 L CNN
F 1 "100k" V 6250 3400 50  0000 L CNN
F 2 "" H 6300 3700 50  0001 C CNN
F 3 "~" H 6300 3700 50  0001 C CNN
	1    6300 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 6174050B
P 6150 3700
F 0 "R?" V 6100 3800 50  0000 L CNN
F 1 "100k" V 6100 3400 50  0000 L CNN
F 2 "" H 6150 3700 50  0001 C CNN
F 3 "~" H 6150 3700 50  0001 C CNN
	1    6150 3700
	1    0    0    -1  
$EndComp
Wire Wire Line
	6150 3800 6150 4100
Wire Wire Line
	6300 3800 6300 4100
Wire Wire Line
	6300 4100 6150 4100
Connection ~ 6150 4100
Wire Wire Line
	6150 4100 6150 4200
Wire Wire Line
	6450 3800 6450 4100
Wire Wire Line
	6450 4100 6300 4100
Connection ~ 6300 4100
Wire Wire Line
	6600 3800 6600 4100
Wire Wire Line
	6600 4100 6450 4100
Connection ~ 6450 4100
$Comp
L power:GND #PWR?
U 1 1 6174051C
P 6150 4200
F 0 "#PWR?" H 6150 3950 50  0001 C CNN
F 1 "GND" H 6155 4027 50  0000 C CNN
F 2 "" H 6150 4200 50  0001 C CNN
F 3 "" H 6150 4200 50  0001 C CNN
	1    6150 4200
	1    0    0    -1  
$EndComp
Connection ~ 6850 2600
Connection ~ 7000 2700
Connection ~ 7150 2800
Connection ~ 7300 2900
Wire Wire Line
	7600 3500 7700 3500
Wire Wire Line
	7700 3300 6600 3300
Wire Wire Line
	7700 3200 6450 3200
Wire Wire Line
	7700 3100 6300 3100
Wire Wire Line
	7700 3000 6150 3000
Wire Wire Line
	6150 3600 6150 3000
Connection ~ 6150 3000
Wire Wire Line
	6300 3600 6300 3100
Connection ~ 6300 3100
Wire Wire Line
	6450 3600 6450 3200
Connection ~ 6450 3200
Wire Wire Line
	6600 3600 6600 3300
Connection ~ 6600 3300
Wire Wire Line
	5600 3300 6600 3300
Wire Wire Line
	5600 3200 6450 3200
Wire Wire Line
	5600 3100 6300 3100
Wire Wire Line
	5600 3000 6150 3000
Wire Wire Line
	5600 2900 7300 2900
Wire Wire Line
	5600 2800 7150 2800
Wire Wire Line
	5600 2700 7000 2700
Wire Wire Line
	5600 2600 6850 2600
Wire Wire Line
	9800 2600 11100 2600
Wire Wire Line
	9650 2500 11100 2500
Wire Wire Line
	9500 2400 11100 2400
Wire Wire Line
	9350 2300 11100 2300
Wire Wire Line
	10550 2200 11100 2200
Wire Wire Line
	10400 2100 11100 2100
Wire Wire Line
	10250 2000 11100 2000
Wire Wire Line
	10100 1900 11100 1900
Wire Wire Line
	7700 2400 5600 2400
Wire Wire Line
	7700 2300 5600 2300
$Comp
L Device:R_Small R?
U 1 1 6177DCE4
P 9350 1300
F 0 "R?" V 9300 1400 50  0000 L CNN
F 1 "100k" V 9300 1000 50  0000 L CNN
F 2 "" H 9350 1300 50  0001 C CNN
F 3 "~" H 9350 1300 50  0001 C CNN
	1    9350 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	9100 1700 9350 1700
Wire Wire Line
	9350 1700 9350 1400
$Comp
L power:+3V3 #PWR?
U 1 1 61780609
P 9350 1050
F 0 "#PWR?" H 9350 900 50  0001 C CNN
F 1 "+3V3" H 9365 1223 50  0000 C CNN
F 2 "" H 9350 1050 50  0001 C CNN
F 3 "" H 9350 1050 50  0001 C CNN
	1    9350 1050
	1    0    0    -1  
$EndComp
Wire Wire Line
	9350 1200 9350 1050
$Comp
L Device:Jumper JP?
U 1 1 61783055
P 5650 1400
F 0 "JP?" H 5650 1664 50  0000 C CNN
F 1 "Jumper" H 5650 1573 50  0000 C CNN
F 2 "" H 5650 1400 50  0001 C CNN
F 3 "~" H 5650 1400 50  0001 C CNN
	1    5650 1400
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 6178465A
P 6700 1400
F 0 "R?" V 6650 1500 50  0000 L CNN
F 1 "100k" V 6650 1100 50  0000 L CNN
F 2 "" H 6700 1400 50  0001 C CNN
F 3 "~" H 6700 1400 50  0001 C CNN
	1    6700 1400
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 61784660
P 6550 1400
F 0 "R?" V 6500 1500 50  0000 L CNN
F 1 "100k" V 6500 1100 50  0000 L CNN
F 2 "" H 6550 1400 50  0001 C CNN
F 3 "~" H 6550 1400 50  0001 C CNN
	1    6550 1400
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R?
U 1 1 61784666
P 6400 1400
F 0 "R?" V 6350 1500 50  0000 L CNN
F 1 "100k" V 6350 1100 50  0000 L CNN
F 2 "" H 6400 1400 50  0001 C CNN
F 3 "~" H 6400 1400 50  0001 C CNN
	1    6400 1400
	1    0    0    -1  
$EndComp
Wire Wire Line
	7700 1900 6400 1900
Wire Wire Line
	7700 2000 6550 2000
Wire Wire Line
	7700 2100 6700 2100
Wire Wire Line
	6700 1500 6700 2100
Connection ~ 6700 2100
Wire Wire Line
	6550 1500 6550 2000
Connection ~ 6550 2000
Wire Wire Line
	6400 1500 6400 1900
Connection ~ 6400 1900
Wire Wire Line
	6400 1900 6050 1900
$Comp
L power:GND #PWR?
U 1 1 617990EF
P 6400 1050
F 0 "#PWR?" H 6400 800 50  0001 C CNN
F 1 "GND" H 6405 877 50  0000 C CNN
F 2 "" H 6400 1050 50  0001 C CNN
F 3 "" H 6400 1050 50  0001 C CNN
	1    6400 1050
	-1   0    0    1   
$EndComp
Wire Wire Line
	6400 1300 6400 1150
Wire Wire Line
	6700 1300 6700 1150
Wire Wire Line
	6700 1150 6550 1150
Connection ~ 6400 1150
Wire Wire Line
	6400 1150 6400 1050
Wire Wire Line
	6550 1300 6550 1150
Connection ~ 6550 1150
Wire Wire Line
	6550 1150 6400 1150
$Comp
L Device:Jumper JP?
U 1 1 617A37D7
P 5650 2100
F 0 "JP?" H 5650 2364 50  0000 C CNN
F 1 "Jumper" H 5650 2273 50  0000 C CNN
F 2 "" H 5650 2100 50  0001 C CNN
F 3 "~" H 5650 2100 50  0001 C CNN
	1    5650 2100
	1    0    0    -1  
$EndComp
$Comp
L Device:Jumper JP?
U 1 1 617A3B9E
P 5650 1750
F 0 "JP?" H 5650 2014 50  0000 C CNN
F 1 "Jumper" H 5650 1923 50  0000 C CNN
F 2 "" H 5650 1750 50  0001 C CNN
F 3 "~" H 5650 1750 50  0001 C CNN
	1    5650 1750
	1    0    0    -1  
$EndComp
Wire Wire Line
	5950 2100 6700 2100
Wire Wire Line
	6050 1900 6050 1400
Wire Wire Line
	6050 1400 5950 1400
Wire Wire Line
	6000 2000 6000 1750
Wire Wire Line
	6000 1750 5950 1750
Wire Wire Line
	6000 2000 6550 2000
$Comp
L power:+3V3 #PWR?
U 1 1 617AE784
P 5200 1050
F 0 "#PWR?" H 5200 900 50  0001 C CNN
F 1 "+3V3" H 5215 1223 50  0000 C CNN
F 2 "" H 5200 1050 50  0001 C CNN
F 3 "" H 5200 1050 50  0001 C CNN
	1    5200 1050
	1    0    0    -1  
$EndComp
Wire Wire Line
	5200 1050 5200 1400
Wire Wire Line
	5200 2100 5350 2100
Wire Wire Line
	5350 1750 5200 1750
Connection ~ 5200 1750
Wire Wire Line
	5200 1750 5200 2100
Wire Wire Line
	5350 1400 5200 1400
Connection ~ 5200 1400
Wire Wire Line
	5200 1400 5200 1750
Text Label 11100 1900 2    50   ~ 0
IN_Bank2_0
Text Label 11100 2000 2    50   ~ 0
IN_Bank2_1
Text Label 11100 2100 2    50   ~ 0
IN_Bank2_2
Text Label 11100 2200 2    50   ~ 0
IN_Bank2_3
Text Label 11100 2300 2    50   ~ 0
OUT_Bank2_4
Text Label 11100 2400 2    50   ~ 0
OUT_Bank2_5
Text Label 11100 2500 2    50   ~ 0
OUT_Bank2_6
Text Label 11100 2600 2    50   ~ 0
OUT_Bank2_7
Text Label 5600 2600 0    50   ~ 0
OUT_Bank1_0
Text Label 5600 2700 0    50   ~ 0
OUT_Bank1_1
Text Label 5600 2800 0    50   ~ 0
OUT_Bank1_2
Text Label 5600 2900 0    50   ~ 0
OUT_Bank1_3
Text Label 5600 3000 0    50   ~ 0
IN_Bank1_4
Text Label 5600 3100 0    50   ~ 0
IN_Bank1_5
Text Label 5600 3200 0    50   ~ 0
IN_Bank1_6
Text Label 5600 3300 0    50   ~ 0
IN_Bank1_7
Text Label 5600 2300 0    50   ~ 0
I2C_SDA
Text Label 5600 2400 0    50   ~ 0
I2C_SCL
$Comp
L Connector:Raspberry_Pi_2_3 J?
U 1 1 617CCE67
P 2800 5250
F 0 "J?" H 2800 6731 50  0000 C CNN
F 1 "Raspberry_Pi_2_3" H 2050 6550 50  0000 C CNN
F 2 "" H 2800 5250 50  0001 C CNN
F 3 "https://www.raspberrypi.org/documentation/hardware/raspberrypi/schematics/rpi_SCH_3bplus_1p0_reduced.pdf" H 2800 5250 50  0001 C CNN
	1    2800 5250
	1    0    0    -1  
$EndComp
$Comp
L power:+3V3 #PWR?
U 1 1 617DDF38
P 2900 3650
F 0 "#PWR?" H 2900 3500 50  0001 C CNN
F 1 "+3V3" H 2915 3823 50  0000 C CNN
F 2 "" H 2900 3650 50  0001 C CNN
F 3 "" H 2900 3650 50  0001 C CNN
	1    2900 3650
	1    0    0    -1  
$EndComp
Wire Wire Line
	2900 3950 2900 3700
$Comp
L power:PWR_FLAG #FLG?
U 1 1 617E2E34
P 3000 3700
F 0 "#FLG?" H 3000 3775 50  0001 C CNN
F 1 "PWR_FLAG" V 3000 3828 50  0000 L CNN
F 2 "" H 3000 3700 50  0001 C CNN
F 3 "~" H 3000 3700 50  0001 C CNN
	1    3000 3700
	0    1    1    0   
$EndComp
Wire Wire Line
	3000 3700 2900 3700
Connection ~ 2900 3700
Wire Wire Line
	2900 3700 2900 3650
$Comp
L power:GND #PWR?
U 1 1 617E7A05
P 2400 6700
F 0 "#PWR?" H 2400 6450 50  0001 C CNN
F 1 "GND" H 2405 6527 50  0000 C CNN
F 2 "" H 2400 6700 50  0001 C CNN
F 3 "" H 2400 6700 50  0001 C CNN
	1    2400 6700
	1    0    0    -1  
$EndComp
Wire Wire Line
	2400 6700 2400 6550
Text Label 4350 4650 2    50   ~ 0
I2C_SDA
Text Label 4350 4750 2    50   ~ 0
I2C_SCL
Wire Wire Line
	3600 4750 4350 4750
Wire Wire Line
	3600 4650 4350 4650
Wire Wire Line
	1250 4350 2000 4350
Wire Wire Line
	1250 4450 2000 4450
Wire Wire Line
	1250 4850 2000 4850
Wire Wire Line
	1250 5550 2000 5550
Text Label 1250 4350 0    50   ~ 0
OUT_Bank1_0
Text Label 1250 4450 0    50   ~ 0
OUT_Bank1_1
Text Label 1250 4850 0    50   ~ 0
OUT_Bank1_2
Text Label 1250 5550 0    50   ~ 0
OUT_Bank1_3
Wire Wire Line
	1250 5650 2000 5650
Wire Wire Line
	1250 5750 2000 5750
Wire Wire Line
	3600 5450 4350 5450
Wire Wire Line
	3600 5350 4350 5350
Text Label 1250 5650 0    50   ~ 0
IN_Bank1_4
Text Label 1250 5750 0    50   ~ 0
IN_Bank1_5
Text Label 4350 5450 2    50   ~ 0
IN_Bank1_6
Text Label 4350 5350 2    50   ~ 0
IN_Bank1_7
Wire Wire Line
	3600 4950 4350 4950
Wire Wire Line
	1250 4750 2000 4750
Wire Wire Line
	1250 5950 2000 5950
Wire Wire Line
	1250 5450 2000 5450
Text Label 1250 5450 0    50   ~ 0
OUT_Bank2_4
Text Label 1250 5950 0    50   ~ 0
OUT_Bank2_5
Text Label 1250 4750 0    50   ~ 0
OUT_Bank2_6
Text Label 4350 4950 2    50   ~ 0
OUT_Bank2_7
Wire Wire Line
	3600 5650 4350 5650
Wire Wire Line
	3600 5550 4350 5550
Wire Wire Line
	3600 5750 4350 5750
Wire Wire Line
	3600 5050 4350 5050
Text Label 4350 5050 2    50   ~ 0
IN_Bank2_0
Text Label 4350 5750 2    50   ~ 0
IN_Bank2_1
Text Label 4350 5550 2    50   ~ 0
IN_Bank2_2
Text Label 4350 5650 2    50   ~ 0
IN_Bank2_3
Wire Wire Line
	9350 1700 10000 1700
Connection ~ 9350 1700
Text Label 10000 1700 2    50   ~ 0
INT
Text Label 4350 5150 2    50   ~ 0
INT
Wire Wire Line
	3600 5150 4350 5150
$EndSCHEMATC
