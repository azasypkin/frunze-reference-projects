EESchema Schematic File Version 2
LIBS:power
LIBS:device
LIBS:linear
LIBS:regul
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:adafruit
LIBS:User-Submitted
LIBS:Teensy_3_and_LC_Series_Boards_v1.1
LIBS:SparkFun-Sensors
LIBS:SparkFun-Retired
LIBS:SparkFun-Resistors
LIBS:SparkFun-RF
LIBS:SparkFun-PowerIC
LIBS:SparkFun-Passives
LIBS:SparkFun-LED
LIBS:SparkFun-FreqCtrl
LIBS:SparkFun-Electromechanical
LIBS:SparkFun-Displays
LIBS:SparkFun-DiscreteSemi
LIBS:SparkFun-DigitalIC
LIBS:SparkFun-Connectors
LIBS:SparkFun-Capacitors
LIBS:SparkFun-Boards
LIBS:SparkFun-AnalogIC
LIBS:SparkFun-Aesthetics
LIBS:LilyPad-Wearables
LIBS:GeekAmmo
LIBS:Switch
LIBS:Relay
LIBS:Motor
LIBS:Connector
LIBS:Transistor
LIBS:Display
LIBS:Valve
LIBS:Logic_74xgxx
LIBS:Logic_74xx
LIBS:Logic_CMOS_4000
LIBS:Logic_CMOS_IEEE
LIBS:reference-project
LIBS:Abracon
LIBS:ActiveSemi
LIBS:Altera
LIBS:AMS
LIBS:AnalogDevices
LIBS:AOS
LIBS:Atmel
LIBS:Bosch
LIBS:conn-2mm
LIBS:conn-100mil
LIBS:conn-amphenol
LIBS:conn-assmann
LIBS:conn-cui
LIBS:conn-fci
LIBS:conn-jae
LIBS:conn-linx
LIBS:conn-molex
LIBS:conn-special-headers
LIBS:conn-tagconnect
LIBS:conn-te
LIBS:conn-test
LIBS:DiodesInc
LIBS:display
LIBS:electomech-misc
LIBS:_electromech
LIBS:Fairchild
LIBS:FTDI
LIBS:Infineon
LIBS:Intersil
LIBS:Lattice
LIBS:_linear
LIBS:LinearTech
LIBS:Littelfuse
LIBS:_logic
LIBS:logic-4000
LIBS:logic-7400
LIBS:logic-7400-new
LIBS:MACOM
LIBS:Macrofab
LIBS:maxim
LIBS:mechanical
LIBS:Microchip
LIBS:Micron
LIBS:Murata
LIBS:NXP
LIBS:OceanOptics
LIBS:onsemi
LIBS:_passive
LIBS:pasv-BelFuse
LIBS:pasv-BiTech
LIBS:pasv-Bourns
LIBS:pasv-cap
LIBS:pasv-ind
LIBS:pasv-Murata
LIBS:pasv-res
LIBS:pasv-TDK
LIBS:pasv-xtal
LIBS:pcb
LIBS:Recom
LIBS:Richtek
LIBS:_semi
LIBS:semi-diode-DiodesInc
LIBS:semi-diode-generic
LIBS:semi-diode-MCC
LIBS:semi-diode-NXP
LIBS:semi-diode-OnSemi
LIBS:semi-diode-Semtech
LIBS:semi-diode-ST
LIBS:semi-diode-Toshiba
LIBS:semi-opto-generic
LIBS:semi-opto-liteon
LIBS:semi-thyristor-generic
LIBS:semi-trans-AOS
LIBS:semi-trans-DiodesInc
LIBS:semi-trans-EPC
LIBS:semi-trans-Fairchild
LIBS:semi-trans-generic
LIBS:semi-trans-Infineon
LIBS:semi-trans-IRF
LIBS:semi-trans-IXYS
LIBS:semi-trans-NXP
LIBS:semi-trans-OnSemi
LIBS:semi-trans-Panasonic
LIBS:semi-trans-ST
LIBS:semi-trans-TI
LIBS:semi-trans-Toshiba
LIBS:semi-trans-Vishay
LIBS:silabs
LIBS:skyworks
LIBS:ST
LIBS:st_ic
LIBS:supertex
LIBS:symbol
LIBS:TexasInstruments
LIBS:u-blox
LIBS:Vishay
LIBS:Winbond
LIBS:Xilinx
LIBS:reference-project-cache
EELAYER 25 0
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
L SW_Push SW1
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
L SW_Push SW2
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
L ATTINY85-20PU U1
U 1 1 59E50D10
P 7400 4150
F 0 "U1" H 6250 4550 50  0000 C CNN
F 1 "ATTINY85-20PU" H 8400 3750 50  0000 C CNN
F 2 "Housings_DIP:DIP-8_W7.62mm" H 8400 4150 50  0001 C CIN
F 3 "" H 7400 4150 50  0001 C CNN
	1    7400 4150
	-1   0    0    -1  
$EndComp
$Comp
L Speaker LS1
U 1 1 59E51513
P 10300 4000
F 0 "LS1" H 10350 4225 50  0000 R CNN
F 1 "Buzzer" H 10350 4150 50  0000 R CNN
F 2 "Buzzers_Beepers:Buzzer_12x9.5RM7.6" H 10300 3800 50  0001 C CNN
F 3 "" H 10290 3950 50  0001 C CNN
	1    10300 4000
	1    0    0    -1  
$EndComp
$Comp
L R R2
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
L R R3
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
L R R1
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
L GND #PWR2
U 1 1 59E5A2E5
P 6050 5400
F 0 "#PWR2" H 6050 5150 50  0001 C CNN
F 1 "GND" H 6050 5250 50  0000 C CNN
F 2 "" H 6050 5400 50  0001 C CNN
F 3 "" H 6050 5400 50  0001 C CNN
	1    6050 5400
	1    0    0    -1  
$EndComp
NoConn ~ 8750 4400
$Comp
L +3.3V #PWR1
U 1 1 59E632F2
P 6050 2850
F 0 "#PWR1" H 6050 2700 50  0001 C CNN
F 1 "+3.3V" H 6050 2990 50  0000 C CNN
F 2 "" H 6050 2850 50  0001 C CNN
F 3 "" H 6050 2850 50  0001 C CNN
	1    6050 2850
	1    0    0    -1  
$EndComp
$Comp
L +3.3V #PWR5
U 1 1 59E635D9
P 6750 1600
F 0 "#PWR5" H 6750 1450 50  0001 C CNN
F 1 "+3.3V" H 6750 1740 50  0000 C CNN
F 2 "" H 6750 1600 50  0001 C CNN
F 3 "" H 6750 1600 50  0001 C CNN
	1    6750 1600
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR6
U 1 1 59E63687
P 7200 1600
F 0 "#PWR6" H 7200 1350 50  0001 C CNN
F 1 "GND" H 7200 1450 50  0000 C CNN
F 2 "" H 7200 1600 50  0001 C CNN
F 3 "" H 7200 1600 50  0001 C CNN
	1    7200 1600
	1    0    0    -1  
$EndComp
$Comp
L PWR_FLAG #FLG1
U 1 1 59E6375F
P 6750 1600
F 0 "#FLG1" H 6750 1675 50  0001 C CNN
F 1 "PWR_FLAG" H 6750 1750 50  0000 C CNN
F 2 "" H 6750 1600 50  0001 C CNN
F 3 "" H 6750 1600 50  0001 C CNN
	1    6750 1600
	-1   0    0    1   
$EndComp
$Comp
L PWR_FLAG #FLG2
U 1 1 59E636C7
P 7200 1600
F 0 "#FLG2" H 7200 1675 50  0001 C CNN
F 1 "PWR_FLAG" H 7200 1750 50  0000 C CNN
F 2 "" H 7200 1600 50  0001 C CNN
F 3 "" H 7200 1600 50  0001 C CNN
	1    7200 1600
	1    0    0    -1  
$EndComp
$Comp
L CELL BT1
U 1 1 59E7C376
P 6350 1650
F 0 "BT1" H 6350 1950 50  0000 C CNN
F 1 "CELL" H 6250 1750 50  0000 C CNN
F 2 "Connectors:bornier2" H 6350 1550 60  0000 C CNN
F 3 "" H 6350 1550 60  0000 C CNN
	1    6350 1650
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR4
U 1 1 59E7C403
P 6350 1750
F 0 "#PWR4" H 6350 1500 50  0001 C CNN
F 1 "GND" H 6350 1600 50  0000 C CNN
F 2 "" H 6350 1750 50  0001 C CNN
F 3 "" H 6350 1750 50  0001 C CNN
	1    6350 1750
	1    0    0    -1  
$EndComp
$Comp
L +3.3V #PWR3
U 1 1 59E7C445
P 6350 1550
F 0 "#PWR3" H 6350 1400 50  0001 C CNN
F 1 "+3.3V" H 6350 1690 50  0000 C CNN
F 2 "" H 6350 1550 50  0001 C CNN
F 3 "" H 6350 1550 50  0001 C CNN
	1    6350 1550
	1    0    0    -1  
$EndComp
NoConn ~ 8750 4100
NoConn ~ 8750 3900
Wire Wire Line
	10100 5100 10100 4100
Wire Wire Line
	9950 4000 10100 4000
Wire Wire Line
	6050 3200 8850 3200
Wire Wire Line
	8750 4000 9650 4000
Wire Wire Line
	8850 4950 6050 4950
Wire Wire Line
	6050 4400 6050 5400
Wire Wire Line
	10100 5100 6050 5100
Connection ~ 6050 5100
Wire Wire Line
	8750 4200 9350 4200
Wire Wire Line
	8850 4200 8850 3750
Wire Wire Line
	8850 3200 8850 3450
Wire Wire Line
	6050 2850 6050 3900
Wire Wire Line
	8750 4300 9600 4300
Wire Wire Line
	8850 4300 8850 4500
Wire Wire Line
	8850 4800 8850 4950
Connection ~ 8850 4200
Wire Wire Line
	6050 5300 8950 5300
Connection ~ 6050 5300
Wire Wire Line
	9350 4200 9350 5300
Connection ~ 6050 4950
Connection ~ 6050 3200
Wire Wire Line
	9600 4300 9600 5300
Connection ~ 8850 4300
Wire Wire Line
	10000 5300 10000 3000
Wire Wire Line
	10000 3000 6050 3000
Connection ~ 6050 3000
$EndSCHEMATC
