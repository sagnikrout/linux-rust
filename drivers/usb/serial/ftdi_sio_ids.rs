//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/ftdi_sio_ids.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// vendor/product IDs (VID/PID) of devices using FTDI USB serial converters.
// Please keep numerically sorted within individual areas, thanks!
//
// Philipp Gühring - pg@futureware.at - added the Device ID of the USB relais
// from Rudolf Gugler
//
// devices using FTDI VID
//
pub const FTDI_VID: c_uint = 0x0403	/* Vendor Id */;
// "original" FTDI device PIDs
pub const FTDI_8U232AM_PID: c_uint = 0x6001 /* Similar device to SIO above */;
pub const FTDI_8U232AM_ALT_PID: c_uint = 0x6006 /* FTDI's alternate PID for above */;
pub const FTDI_8U2232C_PID: c_uint = 0x6010 /* Dual channel device */;
pub const FTDI_4232H_PID: c_uint = 0x6011 /* Quad channel hi-speed device */;
pub const FTDI_232H_PID: c_uint = 0x6014 /* Single channel hi-speed device */;
pub const FTDI_FTX_PID: c_uint = 0x6015 /* FT-X series (FT201X, FT230X, FT231X, etc) */;
pub const FTDI_FT2233HP_PID: c_uint = 0x6040 /* Dual channel hi-speed device with PD */;
pub const FTDI_FT4233HP_PID: c_uint = 0x6041 /* Quad channel hi-speed device with PD */;
pub const FTDI_FT2232HP_PID: c_uint = 0x6042 /* Dual channel hi-speed device with PD */;
pub const FTDI_FT4232HP_PID: c_uint = 0x6043 /* Quad channel hi-speed device with PD */;
pub const FTDI_FT233HP_PID: c_uint = 0x6044 /* Dual channel hi-speed device with PD */;
pub const FTDI_FT232HP_PID: c_uint = 0x6045 /* Dual channel hi-speed device with PD */;
pub const FTDI_FT4232HA_PID: c_uint = 0x6048 /* Quad channel automotive grade hi-speed device */;
pub const FTDI_SIO_PID: c_uint = 0x8372	/* Product Id SIO application of 8U100AX */;
pub const FTDI_232RL_PID: c_uint = 0xFBFA  /* Product ID for FT232RL */;
// third-party PIDs (using FTDI_VID)
//
// Certain versions of the official Windows FTDI driver reprogrammed
// counterfeit FTDI devices to PID 0. Support these devices anyway.
//
pub const FTDI_BRICK_PID: c_uint = 0x0000;
pub const FTDI_LUMEL_PD12_PID: c_uint = 0x6002;
//
// Custom USB adapters made by Falconia Partners LLC
// for FreeCalypso project, ID codes allocated to Falconia by FTDI.
//
pub const FTDI_FALCONIA_JTAG_BUF_PID: c_uint = 0x7150;
pub const FTDI_FALCONIA_JTAG_UNBUF_PID: c_uint = 0x7151;
// Sienna Serial Interface by Secyourit GmbH
pub const FTDI_SIENNA_PID: c_uint = 0x8348;
// Cyber Cortex AV by Fabulous Silicon (http://fabuloussilicon.com)
pub const CYBER_CORTEX_AV_PID: c_uint = 0x8698;
//
// Marvell OpenRD Base, Client
// http://www.open-rd.org
// OpenRD Base, Client use VID 0x0403
//
pub const MARVELL_OPENRD_PID: c_uint = 0x9e90;
// www.candapter.com Ewert Energy Systems CANdapter device
pub const FTDI_CANDAPTER_PID: c_uint = 0x9F80 /* Product Id */;
pub const FTDI_BM_ATOM_NANO_PID: c_uint = 0xa559	/* Basic Micro ATOM Nano USB2Serial */;
//
// Texas Instruments XDS100v2 JTAG / BeagleBone A3
// http://processors.wiki.ti.com/index.php/XDS100
// http://beagleboard.org/bone
//
pub const TI_XDS100V2_PID: c_uint = 0xa6d0;
pub const FTDI_NXTCAM_PID: c_uint = 0xABB8 /* NXTCam for Mindstorms NXT */;
pub const FTDI_EV3CON_PID: c_uint = 0xABB9 /* Mindstorms EV3 Console Adapter */;
// US Interface Navigator (http://www.usinterface.com/)
pub const FTDI_USINT_CAT_PID: c_uint = 0xb810	/* Navigator CAT and 2nd PTT lines */;
pub const FTDI_USINT_WKEY_PID: c_uint = 0xb811	/* Navigator WKEY and FSK lines */;
pub const FTDI_USINT_RS232_PID: c_uint = 0xb812	/* Navigator RS232 and CONFIG lines */;
// OOCDlink by Joern Kaipf <joernk@web.de>
// (http://www.joernonline.de/)
pub const FTDI_OOCDLINK_PID: c_uint = 0xbaf8	/* Amontec JTAGkey */;
// Luminary Micro Stellaris Boards, VID = FTDI_VID
// FTDI 2332C Dual channel device, side A=245 FIFO (JTAG), Side B=RS232 UART
pub const LMI_LM3S_DEVEL_BOARD_PID: c_uint = 0xbcd8;
pub const LMI_LM3S_EVAL_BOARD_PID: c_uint = 0xbcd9;
pub const LMI_LM3S_ICDI_BOARD_PID: c_uint = 0xbcda;
pub const FTDI_AXE027_PID: c_uint = 0xBD90 /* PICAXE AXE027 USB download cable */;
pub const FTDI_TURTELIZER_PID: c_uint = 0xBDC8 /* JTAG/RS-232 adapter by egnite GmbH */;
// OpenDCC (www.opendcc.de) product id
pub const FTDI_OPENDCC_PID: c_uint = 0xBFD8;
pub const FTDI_OPENDCC_SNIFFER_PID: c_uint = 0xBFD9;
pub const FTDI_OPENDCC_THROTTLE_PID: c_uint = 0xBFDA;
pub const FTDI_OPENDCC_GATEWAY_PID: c_uint = 0xBFDB;
pub const FTDI_OPENDCC_GBM_PID: c_uint = 0xBFDC;
pub const FTDI_OPENDCC_GBM_BOOST_PID: c_uint = 0xBFDD;
// NZR SEM 16+ USB (http://www.nzr.de)
pub const FTDI_NZR_SEM_USB_PID: c_uint = 0xC1E0	/* NZR SEM-LOG16+ */;
//
// RR-CirKits LocoBuffer USB (http://www.rr-cirkits.com)
//
pub const FTDI_RRCIRKITS_LOCOBUFFER_PID: c_uint = 0xc7d0	/* LocoBuffer USB */;
// DMX4ALL DMX Interfaces
pub const FTDI_DMX4ALL: c_uint = 0xC850;
//
// ASK.fr devices
//
pub const FTDI_ASK_RDR400_PID: c_uint = 0xC991	/* ASK RDR 400 series card reader */;
// www.starting-point-systems.com µChameleon device
pub const FTDI_MICRO_CHAMELEON_PID: c_uint = 0xCAA0	/* Product Id */;
//
// Tactrix OpenPort (ECU) devices.
// OpenPort 1.3M submitted by Donour Sizemore.
// OpenPort 1.3S and 1.3U submitted by Ian Abbott.
//
pub const FTDI_TACTRIX_OPENPORT_13M_PID: c_uint = 0xCC48	/* OpenPort 1.3 Mitsubishi */;
pub const FTDI_TACTRIX_OPENPORT_13S_PID: c_uint = 0xCC49	/* OpenPort 1.3 Subaru */;
pub const FTDI_TACTRIX_OPENPORT_13U_PID: c_uint = 0xCC4A	/* OpenPort 1.3 Universal */;
pub const FTDI_DISTORTEC_JTAG_LOCK_PICK_PID: c_uint = 0xCFF8;
// SCS HF Radio Modems PID's (http://www.scs-ptc.com)
// the VID is the standard ftdi vid (FTDI_VID)
pub const FTDI_SCS_DEVICE_0_PID: c_uint = 0xD010    /* SCS PTC-IIusb */;
pub const FTDI_SCS_DEVICE_1_PID: c_uint = 0xD011    /* SCS Tracker / DSP TNC */;
pub const FTDI_SCS_DEVICE_2_PID: c_uint = 0xD012;
pub const FTDI_SCS_DEVICE_3_PID: c_uint = 0xD013;
pub const FTDI_SCS_DEVICE_4_PID: c_uint = 0xD014;
pub const FTDI_SCS_DEVICE_5_PID: c_uint = 0xD015;
pub const FTDI_SCS_DEVICE_6_PID: c_uint = 0xD016;
pub const FTDI_SCS_DEVICE_7_PID: c_uint = 0xD017;
// iPlus device
pub const FTDI_IPLUS_PID: c_uint = 0xD070 /* Product Id */;
pub const FTDI_IPLUS2_PID: c_uint = 0xD071 /* Product Id */;
//
// Gamma Scout (http://gamma-scout.com/). Submitted by rsc@runtux.com.
//
pub const FTDI_GAMMA_SCOUT_PID: c_uint = 0xD678	/* Gamma Scout online */;
// Propox devices
pub const FTDI_PROPOX_JTAGCABLEII_PID: c_uint = 0xD738;
pub const FTDI_PROPOX_ISPCABLEIII_PID: c_uint = 0xD739;
// Lenz LI-USB Computer Interface.
pub const FTDI_LENZ_LIUSB_PID: c_uint = 0xD780;
// Vardaan Enterprises Serial Interface VEUSB422R3
pub const FTDI_VARDAAN_PID: c_uint = 0xF070;
// Auto-M3 Ltd. - OP-COM USB V2 - OBD interface Adapter
pub const FTDI_AUTO_M3_OP_COM_V2_PID: c_uint = 0x4f50;
//
// Xsens Technologies BV products (http://www.xsens.com).
//
pub const XSENS_VID: c_uint = 0x2639;
pub const XSENS_AWINDA_STATION_PID: c_uint = 0x0101;
pub const XSENS_AWINDA_DONGLE_PID: c_uint = 0x0102;
pub const XSENS_MTW_PID: c_uint = 0x0200	/* Xsens MTw */;
pub const XSENS_MTDEVBOARD_PID: c_uint = 0x0300	/* Motion Tracker Development Board */;
pub const XSENS_MTIUSBCONVERTER_PID: c_uint = 0x0301	/* MTi USB converter */;
pub const XSENS_CONVERTER_PID: c_uint = 0xD00D	/* Xsens USB-serial converter */;
// Xsens devices using FTDI VID
pub const XSENS_CONVERTER_0_PID: c_uint = 0xD388	/* Xsens USB converter */;
pub const XSENS_CONVERTER_1_PID: c_uint = 0xD389	/* Xsens Wireless Receiver */;
pub const XSENS_CONVERTER_2_PID: c_uint = 0xD38A;
pub const XSENS_CONVERTER_3_PID: c_uint = 0xD38B	/* Xsens USB-serial converter */;
pub const XSENS_CONVERTER_4_PID: c_uint = 0xD38C	/* Xsens Wireless Receiver */;
pub const XSENS_CONVERTER_5_PID: c_uint = 0xD38D	/* Xsens Awinda Station */;
pub const XSENS_CONVERTER_6_PID: c_uint = 0xD38E;
pub const XSENS_CONVERTER_7_PID: c_uint = 0xD38F;
//
// Zolix (www.zolix.com.cb) product ids
//
pub const FTDI_OMNI1509: c_uint = 0xD491	/* Omni1509 embedded USB-serial */;
//
// NDI (www.ndigital.com) product ids
//
pub const FTDI_NDI_HUC_PID: c_uint = 0xDA70	/* NDI Host USB Converter */;
pub const FTDI_NDI_SPECTRA_SCU_PID: c_uint = 0xDA71	/* NDI Spectra SCU */;
pub const FTDI_NDI_FUTURE_2_PID: c_uint = 0xDA72	/* NDI future device #2 */;
pub const FTDI_NDI_FUTURE_3_PID: c_uint = 0xDA73	/* NDI future device #3 */;
pub const FTDI_NDI_AURORA_SCU_PID: c_uint = 0xDA74	/* NDI Aurora SCU */;
pub const FTDI_NDI_VID: c_uint = 0x23F2;
pub const FTDI_NDI_EMGUIDE_GEMINI_PID: c_uint = 0x0003	/* NDI Emguide Gemini */;
//
// ChamSys Limited (www.chamsys.co.uk) USB wing/interface product IDs
//
pub const FTDI_CHAMSYS_24_MASTER_WING_PID: c_uint = 0xDAF8;
pub const FTDI_CHAMSYS_PC_WING_PID: c_uint = 0xDAF9;
pub const FTDI_CHAMSYS_USB_DMX_PID: c_uint = 0xDAFA;
pub const FTDI_CHAMSYS_MIDI_TIMECODE_PID: c_uint = 0xDAFB;
pub const FTDI_CHAMSYS_MINI_WING_PID: c_uint = 0xDAFC;
pub const FTDI_CHAMSYS_MAXI_WING_PID: c_uint = 0xDAFD;
pub const FTDI_CHAMSYS_MEDIA_WING_PID: c_uint = 0xDAFE;
pub const FTDI_CHAMSYS_WING_PID: c_uint = 0xDAFF;
//
// Westrex International devices submitted by Cory Lee
//
pub const FTDI_WESTREX_MODEL_777_PID: c_uint = 0xDC00	/* Model 777 */;
pub const FTDI_WESTREX_MODEL_8900F_PID: c_uint = 0xDC01	/* Model 8900F */;
//
// ACG Identification Technologies GmbH products (http://www.acg.de/).
// Submitted by anton -at- goto10 -dot- org.
//
pub const FTDI_ACG_HFDUAL_PID: c_uint = 0xDD20	/* HF Dual ISO Reader (RFID) */;
//
// Definitions for Artemis astronomical USB based cameras
// Check it at http://www.artemisccd.co.uk
//
pub const FTDI_ARTEMIS_PID: c_uint = 0xDF28	/* All Artemis Cameras */;
//
// Definitions for ATIK Instruments astronomical USB based cameras
// Check it at http://www.atik-instruments.com
//
pub const FTDI_ATIK_ATK16_PID: c_uint = 0xDF30	/* ATIK ATK-16 Grayscale Camera */;
pub const FTDI_ATIK_ATK16C_PID: c_uint = 0xDF32	/* ATIK ATK-16C Colour Camera */;
pub const FTDI_ATIK_ATK16HR_PID: c_uint = 0xDF31	/* ATIK ATK-16HR Grayscale Camera */;
pub const FTDI_ATIK_ATK16HRC_PID: c_uint = 0xDF33	/* ATIK ATK-16HRC Colour Camera */;
pub const FTDI_ATIK_ATK16IC_PID: c_uint = 0xDF35  /* ATIK ATK-16IC Grayscale Camera */;
//
// Yost Engineering, Inc. products (www.yostengineering.com).
// PID 0xE050 submitted by Aaron Prose.
//
pub const FTDI_YEI_SERVOCENTER31_PID: c_uint = 0xE050	/* YEI ServoCenter3.1 USB */;
//
// ELV USB devices submitted by Christian Abt of ELV (www.elv.de).
// Almost all of these devices use FTDI's vendor ID (0x0403).
// Further IDs taken from ELV Windows .inf file.
//
// The previously included PID for the UO 100 module was incorrect.
// In fact, that PID was for ELV's UR 100 USB-RS232 converter (0xFB58).
//
// Armin Laeuger originally sent the PID for the UM 100 module.
//
pub const FTDI_ELV_VID: c_uint = 0x1B1F	/* ELV AG */;
pub const FTDI_ELV_WS300_PID: c_uint = 0xC006	/* eQ3 WS 300 PC II */;
pub const FTDI_ELV_USR_PID: c_uint = 0xE000	/* ELV Universal-Sound-Recorder */;
pub const FTDI_ELV_MSM1_PID: c_uint = 0xE001	/* ELV Mini-Sound-Modul */;
pub const FTDI_ELV_KL100_PID: c_uint = 0xE002	/* ELV Kfz-Leistungsmesser KL 100 */;
pub const FTDI_ELV_WS550_PID: c_uint = 0xE004	/* WS 550 */;
pub const FTDI_ELV_EC3000_PID: c_uint = 0xE006	/* ENERGY CONTROL 3000 USB */;
pub const FTDI_ELV_WS888_PID: c_uint = 0xE008	/* WS 888 */;
pub const FTDI_ELV_TWS550_PID: c_uint = 0xE009	/* Technoline WS 550 */;
pub const FTDI_ELV_FEM_PID: c_uint = 0xE00A	/* Funk Energie Monitor */;
pub const FTDI_ELV_FHZ1300PC_PID: c_uint = 0xE0E8	/* FHZ 1300 PC */;
pub const FTDI_ELV_WS500_PID: c_uint = 0xE0E9	/* PC-Wetterstation (WS 500) */;
pub const FTDI_ELV_HS485_PID: c_uint = 0xE0EA	/* USB to RS-485 adapter */;
pub const FTDI_ELV_UMS100_PID: c_uint = 0xE0EB	/* ELV USB Master-Slave Schaltsteckdose UMS 100 */;
pub const FTDI_ELV_TFD128_PID: c_uint = 0xE0EC	/* ELV Temperatur-Feuchte-Datenlogger TFD 128 */;
pub const FTDI_ELV_FM3RX_PID: c_uint = 0xE0ED	/* ELV Messwertuebertragung FM3 RX */;
pub const FTDI_ELV_WS777_PID: c_uint = 0xE0EE	/* Conrad WS 777 */;
pub const FTDI_ELV_EM1010PC_PID: c_uint = 0xE0EF	/* Energy monitor EM 1010 PC */;
pub const FTDI_ELV_CSI8_PID: c_uint = 0xE0F0	/* Computer-Schalt-Interface (CSI 8) */;
pub const FTDI_ELV_EM1000DL_PID: c_uint = 0xE0F1	/* PC-Datenlogger fuer Energiemonitor (EM 1000 DL) */;
pub const FTDI_ELV_PCK100_PID: c_uint = 0xE0F2	/* PC-Kabeltester (PCK 100) */;
pub const FTDI_ELV_RFP500_PID: c_uint = 0xE0F3	/* HF-Leistungsmesser (RFP 500) */;
pub const FTDI_ELV_FS20SIG_PID: c_uint = 0xE0F4	/* Signalgeber (FS 20 SIG) */;
pub const FTDI_ELV_UTP8_PID: c_uint = 0xE0F5	/* ELV UTP 8 */;
pub const FTDI_ELV_WS300PC_PID: c_uint = 0xE0F6	/* PC-Wetterstation (WS 300 PC) */;
pub const FTDI_ELV_WS444PC_PID: c_uint = 0xE0F7	/* Conrad WS 444 PC */;
pub const FTDI_PHI_FISCO_PID: c_uint = 0xE40B  /* PHI Fisco USB to Serial cable */;
pub const FTDI_ELV_UAD8_PID: c_uint = 0xF068	/* USB-AD-Wandler (UAD 8) */;
pub const FTDI_ELV_UDA7_PID: c_uint = 0xF069	/* USB-DA-Wandler (UDA 7) */;
pub const FTDI_ELV_USI2_PID: c_uint = 0xF06A	/* USB-Schrittmotoren-Interface (USI 2) */;
pub const FTDI_ELV_T1100_PID: c_uint = 0xF06B	/* Thermometer (T 1100) */;
pub const FTDI_ELV_PCD200_PID: c_uint = 0xF06C	/* PC-Datenlogger (PCD 200) */;
pub const FTDI_ELV_ULA200_PID: c_uint = 0xF06D	/* USB-LCD-Ansteuerung (ULA 200) */;
pub const FTDI_ELV_ALC8500_PID: c_uint = 0xF06E	/* ALC 8500 Expert */;
pub const FTDI_ELV_FHZ1000PC_PID: c_uint = 0xF06F	/* FHZ 1000 PC */;
pub const FTDI_ELV_UR100_PID: c_uint = 0xFB58	/* USB-RS232-Umsetzer (UR 100) */;
pub const FTDI_ELV_UM100_PID: c_uint = 0xFB5A	/* USB-Modul UM 100 */;
pub const FTDI_ELV_UO100_PID: c_uint = 0xFB5B	/* USB-Modul UO 100 */;
// Additional ELV PIDs that default to using the FTDI D2XX drivers on
// MS Windows, rather than the FTDI Virtual Com Port drivers.
// Maybe these will be easier to use with the libftdi/libusb user-space
// drivers, or possibly the Comedi drivers in some cases.
pub const FTDI_ELV_CLI7000_PID: c_uint = 0xFB59	/* Computer-Light-Interface (CLI 7000) */;
pub const FTDI_ELV_PPS7330_PID: c_uint = 0xFB5C	/* Processor-Power-Supply (PPS 7330) */;
pub const FTDI_ELV_TFM100_PID: c_uint = 0xFB5D	/* Temperatur-Feuchte-Messgeraet (TFM 100) */;
pub const FTDI_ELV_UDF77_PID: c_uint = 0xFB5E	/* USB DCF Funkuhr (UDF 77) */;
pub const FTDI_ELV_UIO88_PID: c_uint = 0xFB5F	/* USB-I/O Interface (UIO 88) */;
//
// Endress+Hauser AG product ids (FTDI_VID)
//
pub const FTDI_EH_FXA291_PID: c_uint = 0xE510;
//
// EVER Eco Pro UPS (http://www.ever.com.pl/)
//
pub const EVER_ECO_PRO_CDS: c_uint = 0xe520	/* RS-232 converter */;
//
// Active Robots product ids.
//
pub const FTDI_ACTIVE_ROBOTS_PID: c_uint = 0xE548	/* USB comms board */;
// Pyramid Computer GmbH
pub const FTDI_PYRAMID_PID: c_uint = 0xE6C8	/* Pyramid Appliance Display */;
// www.elsterelectricity.com Elster Unicom III Optical Probe
pub const FTDI_ELSTER_UNICOM_PID: c_uint = 0xE700 /* Product Id */;
//
// Gude Analog- und Digitalsysteme GmbH
//
pub const FTDI_GUDEADS_E808_PID: c_uint = 0xE808;
pub const FTDI_GUDEADS_E809_PID: c_uint = 0xE809;
pub const FTDI_GUDEADS_E80A_PID: c_uint = 0xE80A;
pub const FTDI_GUDEADS_E80B_PID: c_uint = 0xE80B;
pub const FTDI_GUDEADS_E80C_PID: c_uint = 0xE80C;
pub const FTDI_GUDEADS_E80D_PID: c_uint = 0xE80D;
pub const FTDI_GUDEADS_E80E_PID: c_uint = 0xE80E;
pub const FTDI_GUDEADS_E80F_PID: c_uint = 0xE80F;
pub const FTDI_GUDEADS_E888_PID: c_uint = 0xE888  /* Expert ISDN Control USB */;
pub const FTDI_GUDEADS_E889_PID: c_uint = 0xE889  /* USB RS-232 OptoBridge */;
pub const FTDI_GUDEADS_E88A_PID: c_uint = 0xE88A;
pub const FTDI_GUDEADS_E88B_PID: c_uint = 0xE88B;
pub const FTDI_GUDEADS_E88C_PID: c_uint = 0xE88C;
pub const FTDI_GUDEADS_E88D_PID: c_uint = 0xE88D;
pub const FTDI_GUDEADS_E88E_PID: c_uint = 0xE88E;
pub const FTDI_GUDEADS_E88F_PID: c_uint = 0xE88F;
//
// Eclo (http://www.eclo.pt/) product IDs.
// PID 0xEA90 submitted by Martin Grill.
//
pub const FTDI_ECLO_COM_1WIRE_PID: c_uint = 0xEA90	/* COM to 1-Wire USB adaptor */;
// TNC-X USB-to-packet-radio adapter, versions prior to 3.0 (DLP module)
pub const FTDI_TNC_X_PID: c_uint = 0xEBE0;
//
// Teratronik product ids.
// Submitted by O. Wölfelschneider.
//
pub const FTDI_TERATRONIK_VCP_PID: c_uint = 0xEC88	/* Teratronik device (preferring VCP driver on windows) */;
pub const FTDI_TERATRONIK_D2XX_PID: c_uint = 0xEC89	/* Teratronik device (preferring D2XX driver on windows) */;
// Rig Expert Ukraine devices
pub const FTDI_REU_TINY_PID: c_uint = 0xED22	/* RigExpert Tiny */;
//
// Hameg HO820 and HO870 interface (using VID 0x0403)
//
pub const HAMEG_HO820_PID: c_uint = 0xed74;
pub const HAMEG_HO730_PID: c_uint = 0xed73;
pub const HAMEG_HO720_PID: c_uint = 0xed72;
pub const HAMEG_HO870_PID: c_uint = 0xed71;
//
// MaxStream devices	www.maxstream.net
//
pub const FTDI_MAXSTREAM_PID: c_uint = 0xEE18	/* Xbee PKG-U Module */;
//
// microHAM product IDs (http://www.microham.com).
// Submitted by Justin Burket (KL1RL) <zorton@jtan.com>
// and Mike Studer (K6EEP) <k6eep@hamsoftware.org>.
// Ian Abbott <abbotti@mev.co.uk> added a few more from the driver INF file.
//
pub const FTDI_MHAM_KW_PID: c_uint = 0xEEE8	/* USB-KW interface */;
pub const FTDI_MHAM_YS_PID: c_uint = 0xEEE9	/* USB-YS interface */;
pub const FTDI_MHAM_Y6_PID: c_uint = 0xEEEA	/* USB-Y6 interface */;
pub const FTDI_MHAM_Y8_PID: c_uint = 0xEEEB	/* USB-Y8 interface */;
pub const FTDI_MHAM_IC_PID: c_uint = 0xEEEC	/* USB-IC interface */;
pub const FTDI_MHAM_DB9_PID: c_uint = 0xEEED	/* USB-DB9 interface */;
pub const FTDI_MHAM_RS232_PID: c_uint = 0xEEEE	/* USB-RS232 interface */;
pub const FTDI_MHAM_Y9_PID: c_uint = 0xEEEF	/* USB-Y9 interface */;
// Domintell products  http://www.domintell.com
pub const FTDI_DOMINTELL_DGQG_PID: c_uint = 0xEF50	/* Master */;
pub const FTDI_DOMINTELL_DUSB_PID: c_uint = 0xEF51	/* DUSB01 module */;
//
// The following are the values for the Perle Systems
// UltraPort USB serial converters
//
pub const FTDI_PERLE_ULTRAPORT_PID: c_uint = 0xF0C0	/* Perle UltraPort Product Id */;
// Sprog II (Andrew Crosland's SprogII DCC interface)
pub const FTDI_SPROG_II: c_uint = 0xF0C8;
//
// Two of the Tagsys RFID Readers
//
pub const FTDI_TAGSYS_LP101_PID: c_uint = 0xF0E9	/* Tagsys L-P101 RFID*/;
pub const FTDI_TAGSYS_P200X_PID: c_uint = 0xF0EE	/* Tagsys Medio P200x RFID*/;
// an infrared receiver for user access control with IR tags
pub const FTDI_PIEGROUP_PID: c_uint = 0xF208	/* Product Id */;
// ACT Solutions HomePro ZWave interface
pub const FTDI_ACTZWAVE_PID: c_uint = 0xF2D0;
//
// 4N-GALAXY.DE PIDs for CAN-USB, USB-RS232, USB-RS422, USB-RS485,
// USB-TTY aktiv, USB-TTY passiv.  Some PIDs are used by several devices
// and I'm not entirely sure which are used by which.
//
pub const FTDI_4N_GALAXY_DE_1_PID: c_uint = 0xF3C0;
pub const FTDI_4N_GALAXY_DE_2_PID: c_uint = 0xF3C1;
pub const FTDI_4N_GALAXY_DE_3_PID: c_uint = 0xF3C2;
//
// Ivium Technologies product IDs
//
pub const FTDI_PALMSENS_PID: c_uint = 0xf440;
pub const FTDI_IVIUM_XSTAT_PID: c_uint = 0xf441;
//
// Linx Technologies product ids
//
pub const LINX_SDMUSBQSS_PID: c_uint = 0xF448	/* Linx SDM-USB-QS-S */;
pub const LINX_MASTERDEVEL2_PID: c_uint = 0xF449	/* Linx Master Development 2.0 */;
pub const LINX_FUTURE_0_PID: c_uint = 0xF44A	/* Linx future device */;
pub const LINX_FUTURE_1_PID: c_uint = 0xF44B	/* Linx future device */;
pub const LINX_FUTURE_2_PID: c_uint = 0xF44C	/* Linx future device */;
//
// Abacus Electrics
//
pub const ABACUS_OPTICAL_PROBE_PID: c_uint = 0xf458 /* ABACUS ELECTRICS Optical Probe */;
//
// Oceanic product ids
//
pub const FTDI_OCEANIC_PID: c_uint = 0xF460  /* Oceanic dive instrument */;
//
// SUUNTO product ids
//
pub const FTDI_SUUNTO_SPORTS_PID: c_uint = 0xF680	/* Suunto Sports instrument */;
// USB-UIRT - An infrared receiver and transmitter using the 8U232AM chip
// http://www.usbuirt.com/
pub const FTDI_USB_UIRT_PID: c_uint = 0xF850	/* Product Id */;
// CCS Inc. ICDU/ICDU40 product ID -
// the FT232BM is used in an in-circuit-debugger unit for PIC16's/PIC18's
pub const FTDI_CCSICDU20_0_PID: c_uint = 0xF9D0;
pub const FTDI_CCSICDU40_1_PID: c_uint = 0xF9D1;
pub const FTDI_CCSMACHX_2_PID: c_uint = 0xF9D2;
pub const FTDI_CCSLOAD_N_GO_3_PID: c_uint = 0xF9D3;
pub const FTDI_CCSICDU64_4_PID: c_uint = 0xF9D4;
pub const FTDI_CCSPRIME8_5_PID: c_uint = 0xF9D5;
//
// The following are the values for the Matrix Orbital LCD displays,
// which are the FT232BM ( similar to the 8U232AM )
//
pub const FTDI_MTXORB_0_PID: c_uint = 0xFA00  /* Matrix Orbital Product Id */;
pub const FTDI_MTXORB_1_PID: c_uint = 0xFA01  /* Matrix Orbital Product Id */;
pub const FTDI_MTXORB_2_PID: c_uint = 0xFA02  /* Matrix Orbital Product Id */;
pub const FTDI_MTXORB_3_PID: c_uint = 0xFA03  /* Matrix Orbital Product Id */;
pub const FTDI_MTXORB_4_PID: c_uint = 0xFA04  /* Matrix Orbital Product Id */;
pub const FTDI_MTXORB_5_PID: c_uint = 0xFA05  /* Matrix Orbital Product Id */;
pub const FTDI_MTXORB_6_PID: c_uint = 0xFA06  /* Matrix Orbital Product Id */;
//
// Home Electronics (www.home-electro.com) USB gadgets
//
pub const FTDI_HE_TIRA1_PID: c_uint = 0xFA78	/* Tira-1 IR transceiver */;
// Inside Accesso contactless reader (http://www.insidecontactless.com/)
pub const INSIDE_ACCESSO: c_uint = 0xFAD0;
//
// ThorLabs USB motor drivers
//
pub const FTDI_THORLABS_PID: c_uint = 0xfaf0 /* ThorLabs USB motor drivers */;
//
// Protego product ids
//
pub const PROTEGO_SPECIAL_1: c_uint = 0xFC70	/* special/unknown device */;
pub const PROTEGO_R2X0: c_uint = 0xFC71	/* R200-USB TRNG unit (R210, R220, and R230) */;
pub const PROTEGO_SPECIAL_3: c_uint = 0xFC72	/* special/unknown device */;
pub const PROTEGO_SPECIAL_4: c_uint = 0xFC73	/* special/unknown device */;
//
// Sony Ericsson product ids
//
pub const FTDI_DSS20_PID: c_uint = 0xFC82	/* DSS-20 Sync Station for Sony Ericsson P800 */;
pub const FTDI_URBAN_0_PID: c_uint = 0xFC8A	/* Sony Ericsson Urban, uart #0 */;
pub const FTDI_URBAN_1_PID: c_uint = 0xFC8B	/* Sony Ericsson Urban, uart #1 */;
// www.irtrans.de device
pub const FTDI_IRTRANS_PID: c_uint = 0xFC60 /* Product Id */;
//
// RM Michaelides CANview USB (http://www.rmcan.com) (FTDI_VID)
// CAN fieldbus interface adapter, added by port GmbH www.port.de)
// Ian Abbott changed the macro names for consistency.
//
pub const FTDI_RM_CANVIEW_PID: c_uint = 0xfd60	/* Product Id */;
// www.thoughttechnology.com/ TT-USB provide with procomp use ftdi_sio
pub const FTDI_TTUSB_PID: c_uint = 0xFF20 /* Product Id */;
pub const FTDI_USBX_707_PID: c_uint = 0xF857	/* ADSTech IR Blaster USBX-707 (FTDI_VID) */;
pub const FTDI_RELAIS_PID: c_uint = 0xFA10  /* Relais device from Rudolf Gugler */;
//
// PCDJ use ftdi based dj-controllers. The following PID is
// for their DAC-2 device http://www.pcdjhardware.com/DAC2.asp
// (the VID is the standard ftdi vid (FTDI_VID), PID sent by Wouter Paesen)
//
pub const FTDI_PCDJ_DAC2_PID: c_uint = 0xFA88;
pub const FTDI_R2000KU_TRUE_RNG: c_uint = 0xFB80  /* R2000KU TRUE RNG (FTDI_VID) */;
//
// DIEBOLD BCS SE923 (FTDI_VID)
//
pub const DIEBOLD_BCS_SE923_PID: c_uint = 0xfb99;
// www.crystalfontz.com devices
// - thanx for providing free devices for evaluation !
// they use the ftdi chipset for the USB interface
// and the vendor id is the same
//
pub const FTDI_XF_632_PID: c_uint = 0xFC08	/* 632: 16x2 Character Display */;
pub const FTDI_XF_634_PID: c_uint = 0xFC09	/* 634: 20x4 Character Display */;
pub const FTDI_XF_547_PID: c_uint = 0xFC0A	/* 547: Two line Display */;
pub const FTDI_XF_633_PID: c_uint = 0xFC0B	/* 633: 16x2 Character Display with Keys */;
pub const FTDI_XF_631_PID: c_uint = 0xFC0C	/* 631: 20x2 Character Display */;
pub const FTDI_XF_635_PID: c_uint = 0xFC0D	/* 635: 20x4 Character Display */;
pub const FTDI_XF_640_PID: c_uint = 0xFC0E	/* 640: Two line Display */;
pub const FTDI_XF_642_PID: c_uint = 0xFC0F	/* 642: Two line Display */;
//
// Video Networks Limited / Homechoice in the UK use an ftdi-based device
// for their 1Mb broadband internet service.  The following PID is exhibited
// by the usb device supplied (the VID is the standard ftdi vid (FTDI_VID)
//
pub const FTDI_VNHCPCUSB_D_PID: c_uint = 0xfe38 /* Product Id */;
// AlphaMicro Components AMC-232USB01 device (FTDI_VID)
pub const FTDI_AMC232_PID: c_uint = 0xFF00 /* Product Id */;
//
// IBS elektronik product ids (FTDI_VID)
// Submitted by Thomas Schleusener
//
pub const FTDI_IBS_US485_PID: c_uint = 0xff38  /* IBS US485 (USB<-->RS422/485 interface) */;
pub const FTDI_IBS_PICPRO_PID: c_uint = 0xff39  /* IBS PIC-Programmer */;
pub const FTDI_IBS_PCMCIA_PID: c_uint = 0xff3a  /* IBS Card reader for PCMCIA SRAM-cards */;
pub const FTDI_IBS_PK1_PID: c_uint = 0xff3b  /* IBS PK1 - Particel counter */;
pub const FTDI_IBS_RS232MON_PID: c_uint = 0xff3c  /* IBS RS232 - Monitor */;
pub const FTDI_IBS_APP70_PID: c_uint = 0xff3d  /* APP 70 (dust monitoring system) */;
pub const FTDI_IBS_PEDO_PID: c_uint = 0xff3e  /* IBS PEDO-Modem (RF modem 868.35 MHz) */;
pub const FTDI_IBS_PROD_PID: c_uint = 0xff3f  /* future device */;
// www.canusb.com Lawicel CANUSB device (FTDI_VID)
pub const FTDI_CANUSB_PID: c_uint = 0xFFA8 /* Product Id */;
//
// TavIR AVR product ids (FTDI_VID)
//
pub const FTDI_TAVIR_STK500_PID: c_uint = 0xFA33	/* STK500 AVR programmer */;
//
// TIAO product ids (FTDI_VID)
// http://www.tiaowiki.com/w/Main_Page
//
pub const FTDI_TIAO_UMPA_PID: c_uint = 0x8a98	/* TIAO/DIYGADGET USB Multi-Protocol Adapter */;
//
// NovaTech product ids (FTDI_VID)
//
pub const FTDI_NT_ORIONLXM_PID: c_uint = 0x7c90	/* OrionLXm Substation Automation Platform */;
pub const FTDI_NT_ORIONLX_PLUS_PID: c_uint = 0x7c91	/* OrionLX+ Substation Automation Platform */;
pub const FTDI_NT_ORION_IO_PID: c_uint = 0x7c92	/* Orion I/O */;
pub const FTDI_NT_ORIONMX_PID: c_uint = 0x7c93	/* OrionMX */;
//
// Synapse Wireless product ids (FTDI_VID)
// http://www.synapse-wireless.com
//
pub const FTDI_SYNAPSE_SS200_PID: c_uint = 0x9090 /* SS200 - SNAP Stick 200 */;
//
// CustomWare / ShipModul NMEA multiplexers product ids (FTDI_VID)
//
pub const FTDI_CUSTOMWARE_MINIPLEX_PID: c_uint = 0xfd48	/* MiniPlex first generation NMEA Multiplexer */;
pub const FTDI_CUSTOMWARE_MINIPLEX2_PID: c_uint = 0xfd49	/* MiniPlex-USB and MiniPlex-2 series */;
pub const FTDI_CUSTOMWARE_MINIPLEX2WI_PID: c_uint = 0xfd4a	/* MiniPlex-2Wi */;
pub const FTDI_CUSTOMWARE_MINIPLEX3_PID: c_uint = 0xfd4b	/* MiniPlex-3 series */;
//
// third-party VID/PID combos
//
// Atmel STK541
//
pub const ATMEL_VID: c_uint = 0x03eb /* Vendor ID */;
pub const STK541_PID: c_uint = 0x2109 /* Zigbee Controller */;
//
// Texas Instruments
//
pub const TI_VID: c_uint = 0x0451;
pub const TI_CC3200_LAUNCHPAD_PID: c_uint = 0xC32A /* SimpleLink Wi-Fi CC3200 LaunchPad */;
//
// Blackfin gnICE JTAG
// http://docs.blackfin.uclinux.org/doku.php?id=hw:jtag:gnice
//
pub const ADI_VID: c_uint = 0x0456;
pub const ADI_GNICE_PID: c_uint = 0xF000;
pub const ADI_GNICEPLUS_PID: c_uint = 0xF001;
//
// Cypress WICED USB UART
//
pub const CYPRESS_VID: c_uint = 0x04B4;
pub const CYPRESS_WICED_BT_USB_PID: c_uint = 0x009B;
pub const CYPRESS_WICED_WL_USB_PID: c_uint = 0xF900;
//
// Microchip Technology, Inc.
//
// MICROCHIP_VID (0x04D8) and MICROCHIP_USB_BOARD_PID (0x000A) are
// used by single function CDC ACM class based firmware demo
// applications.  The VID/PID has also been used in firmware
// emulating FTDI serial chips by:
// Hornby Elite - Digital Command Control Console
// http://www.hornby.com/hornby-dcc/controllers
//
pub const MICROCHIP_VID: c_uint = 0x04D8;
pub const MICROCHIP_USB_BOARD_PID: c_uint = 0x000A /* CDC RS-232 Emulation Demo */;
//
// RATOC REX-USB60F
//
pub const RATOC_VENDOR_ID: c_uint = 0x0584;
pub const RATOC_PRODUCT_ID_USB60F: c_uint = 0xb020;
pub const RATOC_PRODUCT_ID_SCU18: c_uint = 0xb03a;
//
// Infineon Technologies
//
pub const INFINEON_VID: c_uint = 0x058b;
pub const INFINEON_TRIBOARD_TC1798_PID: c_uint = 0x0028 /* DAS JTAG TriBoard TC1798 V1.0 */;
pub const INFINEON_TRIBOARD_TC2X7_PID: c_uint = 0x0043 /* DAS JTAG TriBoard TC2X7 V1.0 */;
//
// Omron corporation (https://www.omron.com)
//
pub const OMRON_VID: c_uint = 0x0590;
pub const OMRON_CS1W_CIF31_PID: c_uint = 0x00b2;
//
// Acton Research Corp.
//
pub const ACTON_VID: c_uint = 0x0647	/* Vendor ID */;
pub const ACTON_SPECTRAPRO_PID: c_uint = 0x0100;
//
// Contec products (http://www.contec.com)
// Submitted by Daniel Sangorrin
//
pub const CONTEC_VID: c_uint = 0x06CE	/* Vendor ID */;
pub const CONTEC_COM1USBH_PID: c_uint = 0x8311	/* COM-1(USB)H */;
//
// Mitsubishi Electric Corp. (http://www.meau.com)
// Submitted by Konstantin Holoborodko
//
pub const MITSUBISHI_VID: c_uint = 0x06D3;
pub const MITSUBISHI_FXUSB_PID: c_uint = 0x0284 /* USB/RS422 converters: FX-USB-AW/-BD */;
//
// Definitions for B&B Electronics products.
//
pub const BANDB_VID: c_uint = 0x0856	/* B&B Electronics Vendor ID */;
pub const BANDB_USOTL4_PID: c_uint = 0xAC01	/* USOTL4 Isolated RS-485 Converter */;
pub const BANDB_USTL4_PID: c_uint = 0xAC02	/* USTL4 RS-485 Converter */;
pub const BANDB_USO9ML2_PID: c_uint = 0xAC03	/* USO9ML2 Isolated RS-232 Converter */;
pub const BANDB_USOPTL4_PID: c_uint = 0xAC11;
pub const BANDB_USPTL4_PID: c_uint = 0xAC12;
pub const BANDB_USO9ML2DR_2_PID: c_uint = 0xAC16;
pub const BANDB_USO9ML2DR_PID: c_uint = 0xAC17;
pub const BANDB_USOPTL4DR2_PID: c_uint = 0xAC18	/* USOPTL4R-2 2-port Isolated RS-232 Converter */;
pub const BANDB_USOPTL4DR_PID: c_uint = 0xAC19;
pub const BANDB_485USB9F_2W_PID: c_uint = 0xAC25;
pub const BANDB_485USB9F_4W_PID: c_uint = 0xAC26;
pub const BANDB_232USB9M_PID: c_uint = 0xAC27;
pub const BANDB_485USBTB_2W_PID: c_uint = 0xAC33;
pub const BANDB_485USBTB_4W_PID: c_uint = 0xAC34;
pub const BANDB_TTL5USB9M_PID: c_uint = 0xAC49;
pub const BANDB_TTL3USB9M_PID: c_uint = 0xAC50;
pub const BANDB_ZZ_PROG1_USB_PID: c_uint = 0xBA02;
//
// Echelon USB Serial Interface
//
pub const ECHELON_VID: c_uint = 0x0920;
pub const ECHELON_U20_PID: c_uint = 0x7500;
//
// Intrepid Control Systems (http://www.intrepidcs.com/) ValueCAN and NeoVI
//
pub const INTREPID_VID: c_uint = 0x093C;
pub const INTREPID_VALUECAN_PID: c_uint = 0x0601;
pub const INTREPID_NEOVI_PID: c_uint = 0x0701;
//
// WICED USB UART
//
pub const WICED_VID: c_uint = 0x0A5C;
pub const WICED_USB20706V2_PID: c_uint = 0x6422;
//
// Definitions for ID TECH (www.idt-net.com) devices
//
pub const IDTECH_VID: c_uint = 0x0ACD	/* ID TECH Vendor ID */;
pub const IDTECH_IDT1221U_PID: c_uint = 0x0300	/* IDT1221U USB to RS-232 adapter */;
//
// Definitions for Omnidirectional Control Technology, Inc. devices
//
pub const OCT_VID: c_uint = 0x0B39	/* OCT vendor ID */;
// Note: OCT US101 is also rebadged as Dick Smith Electronics (NZ) XH6381
// Also rebadged as Dick Smith Electronics (Aus) XH6451
// Also rebadged as SIIG Inc. model US2308 hardware version 1
pub const OCT_DK201_PID: c_uint = 0x0103	/* OCT DK201 USB docking station */;
pub const OCT_US101_PID: c_uint = 0x0421	/* OCT US101 USB to RS-232 */;
//
// Definitions for Icom Inc. devices
//
pub const ICOM_VID: c_uint = 0x0C26 /* Icom vendor ID */;
// Note: ID-1 is a communications tranceiver for HAM-radio operators
pub const ICOM_ID_1_PID: c_uint = 0x0004 /* ID-1 USB to RS-232 */;
// Note: OPC is an Optional cable to connect an Icom Tranceiver
pub const ICOM_OPC_U_UC_PID: c_uint = 0x0018 /* OPC-478UC, OPC-1122U cloning cable */;
// Note: ID-RP* devices are Icom Repeater Devices for HAM-radio
pub const ICOM_ID_RP2C1_PID: c_uint = 0x0009 /* ID-RP2C Asset 1 to RS-232 */;
pub const ICOM_ID_RP2C2_PID: c_uint = 0x000A /* ID-RP2C Asset 2 to RS-232 */;
pub const ICOM_ID_RP2D_PID: c_uint = 0x000B /* ID-RP2D configuration port*/;
pub const ICOM_ID_RP2VT_PID: c_uint = 0x000C /* ID-RP2V Transmit config port */;
pub const ICOM_ID_RP2VR_PID: c_uint = 0x000D /* ID-RP2V Receive config port */;
pub const ICOM_ID_RP4KVT_PID: c_uint = 0x0010 /* ID-RP4000V Transmit config port */;
pub const ICOM_ID_RP4KVR_PID: c_uint = 0x0011 /* ID-RP4000V Receive config port */;
pub const ICOM_ID_RP2KVT_PID: c_uint = 0x0012 /* ID-RP2000V Transmit config port */;
pub const ICOM_ID_RP2KVR_PID: c_uint = 0x0013 /* ID-RP2000V Receive config port */;
//
// GN Otometrics (http://www.otometrics.com)
// Submitted by Ville Sundberg.
//
pub const GN_OTOMETRICS_VID: c_uint = 0x0c33	/* Vendor ID */;
pub const AURICAL_USB_PID: c_uint = 0x0010	/* Aurical USB Audiometer */;
//
// The following are the values for the Sealevel SeaLINK+ adapters.
// (Original list sent by Tuan Hoang.  Ian Abbott renamed the macros and
// removed some PIDs that don't seem to match any existing products.)
//
pub const SEALEVEL_VID: c_uint = 0x0c52	/* Sealevel Vendor ID */;
pub const SEALEVEL_2101_PID: c_uint = 0x2101	/* SeaLINK+232 (2101/2105) */;
pub const SEALEVEL_2102_PID: c_uint = 0x2102	/* SeaLINK+485 (2102) */;
pub const SEALEVEL_2103_PID: c_uint = 0x2103	/* SeaLINK+232I (2103) */;
pub const SEALEVEL_2104_PID: c_uint = 0x2104	/* SeaLINK+485I (2104) */;
pub const SEALEVEL_2106_PID: c_uint = 0x9020	/* SeaLINK+422 (2106) */;
pub const SEALEVEL_2201_1_PID: c_uint = 0x2211	/* SeaPORT+2/232 (2201) Port 1 */;
pub const SEALEVEL_2201_2_PID: c_uint = 0x2221	/* SeaPORT+2/232 (2201) Port 2 */;
pub const SEALEVEL_2202_1_PID: c_uint = 0x2212	/* SeaPORT+2/485 (2202) Port 1 */;
pub const SEALEVEL_2202_2_PID: c_uint = 0x2222	/* SeaPORT+2/485 (2202) Port 2 */;
pub const SEALEVEL_2203_1_PID: c_uint = 0x2213	/* SeaPORT+2 (2203) Port 1 */;
pub const SEALEVEL_2203_2_PID: c_uint = 0x2223	/* SeaPORT+2 (2203) Port 2 */;
pub const SEALEVEL_2401_1_PID: c_uint = 0x2411	/* SeaPORT+4/232 (2401) Port 1 */;
pub const SEALEVEL_2401_2_PID: c_uint = 0x2421	/* SeaPORT+4/232 (2401) Port 2 */;
pub const SEALEVEL_2401_3_PID: c_uint = 0x2431	/* SeaPORT+4/232 (2401) Port 3 */;
pub const SEALEVEL_2401_4_PID: c_uint = 0x2441	/* SeaPORT+4/232 (2401) Port 4 */;
pub const SEALEVEL_2402_1_PID: c_uint = 0x2412	/* SeaPORT+4/485 (2402) Port 1 */;
pub const SEALEVEL_2402_2_PID: c_uint = 0x2422	/* SeaPORT+4/485 (2402) Port 2 */;
pub const SEALEVEL_2402_3_PID: c_uint = 0x2432	/* SeaPORT+4/485 (2402) Port 3 */;
pub const SEALEVEL_2402_4_PID: c_uint = 0x2442	/* SeaPORT+4/485 (2402) Port 4 */;
pub const SEALEVEL_2403_1_PID: c_uint = 0x2413	/* SeaPORT+4 (2403) Port 1 */;
pub const SEALEVEL_2403_2_PID: c_uint = 0x2423	/* SeaPORT+4 (2403) Port 2 */;
pub const SEALEVEL_2403_3_PID: c_uint = 0x2433	/* SeaPORT+4 (2403) Port 3 */;
pub const SEALEVEL_2403_4_PID: c_uint = 0x2443	/* SeaPORT+4 (2403) Port 4 */;

//
// JETI SPECTROMETER SPECBOS 1201
// http://www.jeti.com/cms/index.php/instruments/other-instruments/specbos-2101
//
pub const JETI_VID: c_uint = 0x0c6c;
pub const JETI_SPC1201_PID: c_uint = 0x04b2;
//
// FTDI USB UART chips used in construction projects from the
// Elektor Electronics magazine (http://www.elektor.com/)
//
pub const ELEKTOR_VID: c_uint = 0x0C7D;
pub const ELEKTOR_FT323R_PID: c_uint = 0x0005	/* RFID-Reader, issue 09-2006 */;
//
// Posiflex inc retail equipment (http://www.posiflex.com.tw)
//
pub const POSIFLEX_VID: c_uint = 0x0d3a  /* Vendor ID */;
pub const POSIFLEX_PP7000_PID: c_uint = 0x0300  /* PP-7000II thermal printer */;
//
// The following are the values for two KOBIL chipcard terminals.
//
pub const KOBIL_VID: c_uint = 0x0d46	/* KOBIL Vendor ID */;
pub const KOBIL_CONV_B1_PID: c_uint = 0x2020	/* KOBIL Konverter for B1 */;
pub const KOBIL_CONV_KAAN_PID: c_uint = 0x2021	/* KOBIL_Konverter for KAAN */;
pub const FTDI_NF_RIC_VID: c_uint = 0x0DCD	/* Vendor Id */;
pub const FTDI_NF_RIC_PID: c_uint = 0x0001	/* Product Id */;
//
// Falcom Wireless Communications GmbH
//
pub const FALCOM_VID: c_uint = 0x0F94	/* Vendor Id */;
pub const FALCOM_TWIST_PID: c_uint = 0x0001	/* Falcom Twist USB GPRS modem */;
pub const FALCOM_SAMBA_PID: c_uint = 0x0005	/* Falcom Samba USB GPRS modem */;
// Larsen and Brusgaard AltiTrack/USBtrack
pub const LARSENBRUSGAARD_VID: c_uint = 0x0FD8;
pub const LB_ALTITRACK_PID: c_uint = 0x0001;
//
// TTi (Thurlby Thandar Instruments)
//
pub const TTI_VID: c_uint = 0x103E	/* Vendor Id */;
pub const TTI_QL355P_PID: c_uint = 0x03E8	/* TTi QL355P power supply */;
//
// Newport Cooperation (www.newport.com)
//
pub const NEWPORT_VID: c_uint = 0x104D;
pub const NEWPORT_AGILIS_PID: c_uint = 0x3000;
pub const NEWPORT_CONEX_CC_PID: c_uint = 0x3002;
pub const NEWPORT_CONEX_AGP_PID: c_uint = 0x3006;
// Interbiometrics USB I/O Board
// Developed for Interbiometrics by Rudolf Gugler
pub const INTERBIOMETRICS_VID: c_uint = 0x1209;
pub const INTERBIOMETRICS_IOBOARD_PID: c_uint = 0x1002;
pub const INTERBIOMETRICS_MINI_IOBOARD_PID: c_uint = 0x1006;
//
// Testo products (http://www.testo.com/)
// Submitted by Colin Leroy
//
pub const TESTO_VID: c_uint = 0x128D;
pub const TESTO_1_PID: c_uint = 0x0001;
pub const TESTO_3_PID: c_uint = 0x0003;
//
// Mobility Electronics products.
//
pub const MOBILITY_VID: c_uint = 0x1342;
pub const MOBILITY_USB_SERIAL_PID: c_uint = 0x0202	/* EasiDock USB 200 serial */;
//
// FIC / OpenMoko, Inc. http://wiki.openmoko.org/wiki/Neo1973_Debug_Board_v3
// Submitted by Harald Welte <laforge@openmoko.org>
//
pub const FIC_VID: c_uint = 0x1457;
pub const FIC_NEO1973_DEBUG_PID: c_uint = 0x5118;
//
// Actel / Microsemi
//
pub const ACTEL_VID: c_uint = 0x1514;
pub const MICROSEMI_ARROW_SF2PLUS_BOARD_PID: c_uint = 0x2008;
// Olimex
pub const OLIMEX_VID: c_uint = 0x15BA;
pub const OLIMEX_ARM_USB_OCD_PID: c_uint = 0x0003;
pub const OLIMEX_ARM_USB_TINY_PID: c_uint = 0x0004;
pub const OLIMEX_ARM_USB_TINY_H_PID: c_uint = 0x002a;
pub const OLIMEX_ARM_USB_OCD_H_PID: c_uint = 0x002b;
//
// Telldus Technologies
//
pub const TELLDUS_VID: c_uint = 0x1781	/* Vendor ID */;
pub const TELLDUS_TELLSTICK_PID: c_uint = 0x0C30	/* RF control dongle 433 MHz using FT232RL */;
//
// NOVITUS printers
//
pub const NOVITUS_VID: c_uint = 0x1a28;
pub const NOVITUS_BONO_E_PID: c_uint = 0x6010;
//
// ICPDAS I-756*U devices
//
pub const ICPDAS_VID: c_uint = 0x1b5c;
pub const ICPDAS_I7560U_PID: c_uint = 0x0103;
pub const ICPDAS_I7561U_PID: c_uint = 0x0104;
pub const ICPDAS_I7563U_PID: c_uint = 0x0105;
//
// Airbus Defence and Space
//
pub const AIRBUS_DS_VID: c_uint = 0x1e8e  /* Vendor ID */;
pub const AIRBUS_DS_P8GR: c_uint = 0x6001  /* Tetra P8GR */;
//
// RT Systems programming cables for various ham radios
//
// This device uses the VID of FTDI
pub const RTSYSTEMS_USB_VX8_PID: c_uint = 0x9e50  /* USB-VX8 USB to 7 pin modular plug for Yaesu VX-8 radio */;
pub const RTSYSTEMS_VID: c_uint = 0x2100	/* Vendor ID */;
pub const RTSYSTEMS_USB_S03_PID: c_uint = 0x9001	/* RTS-03 USB to Serial Adapter */;
pub const RTSYSTEMS_USB_59_PID: c_uint = 0x9e50	/* USB-59 USB to 8 pin plug */;
pub const RTSYSTEMS_USB_57A_PID: c_uint = 0x9e51	/* USB-57A USB to 4pin 3.5mm plug */;
pub const RTSYSTEMS_USB_57B_PID: c_uint = 0x9e52	/* USB-57B USB to extended 4pin 3.5mm plug */;
pub const RTSYSTEMS_USB_29A_PID: c_uint = 0x9e53	/* USB-29A USB to 3.5mm stereo plug */;
pub const RTSYSTEMS_USB_29B_PID: c_uint = 0x9e54	/* USB-29B USB to 6 pin mini din */;
pub const RTSYSTEMS_USB_29F_PID: c_uint = 0x9e55	/* USB-29F USB to 6 pin modular plug */;
pub const RTSYSTEMS_USB_62B_PID: c_uint = 0x9e56	/* USB-62B USB to 8 pin mini din plug*/;
pub const RTSYSTEMS_USB_S01_PID: c_uint = 0x9e57	/* USB-RTS01 USB to 3.5 mm stereo plug*/;
pub const RTSYSTEMS_USB_63_PID: c_uint = 0x9e58	/* USB-63 USB to 9 pin female*/;
pub const RTSYSTEMS_USB_29C_PID: c_uint = 0x9e59	/* USB-29C USB to 4 pin modular plug*/;
pub const RTSYSTEMS_USB_81B_PID: c_uint = 0x9e5A	/* USB-81 USB to 8 pin mini din plug*/;
pub const RTSYSTEMS_USB_82B_PID: c_uint = 0x9e5B	/* USB-82 USB to 2.5 mm stereo plug*/;
pub const RTSYSTEMS_USB_K5D_PID: c_uint = 0x9e5C	/* USB-K5D USB to 8 pin modular plug*/;
pub const RTSYSTEMS_USB_K4Y_PID: c_uint = 0x9e5D	/* USB-K4Y USB to 2.5/3.5 mm plugs*/;
pub const RTSYSTEMS_USB_K5G_PID: c_uint = 0x9e5E	/* USB-K5G USB to 8 pin modular plug*/;
pub const RTSYSTEMS_USB_S05_PID: c_uint = 0x9e5F	/* USB-RTS05 USB to 2.5 mm stereo plug*/;
pub const RTSYSTEMS_USB_60_PID: c_uint = 0x9e60	/* USB-60 USB to 6 pin din*/;
pub const RTSYSTEMS_USB_61_PID: c_uint = 0x9e61	/* USB-61 USB to 6 pin mini din*/;
pub const RTSYSTEMS_USB_62_PID: c_uint = 0x9e62	/* USB-62 USB to 8 pin mini din*/;
pub const RTSYSTEMS_USB_63B_PID: c_uint = 0x9e63	/* USB-63 USB to 9 pin female*/;
pub const RTSYSTEMS_USB_64_PID: c_uint = 0x9e64	/* USB-64 USB to 9 pin male*/;
pub const RTSYSTEMS_USB_65_PID: c_uint = 0x9e65	/* USB-65 USB to 9 pin female null modem*/;
pub const RTSYSTEMS_USB_92_PID: c_uint = 0x9e66	/* USB-92 USB to 12 pin plug*/;
pub const RTSYSTEMS_USB_92D_PID: c_uint = 0x9e67	/* USB-92D USB to 12 pin plug data*/;
pub const RTSYSTEMS_USB_W5R_PID: c_uint = 0x9e68	/* USB-W5R USB to 8 pin modular plug*/;
pub const RTSYSTEMS_USB_A5R_PID: c_uint = 0x9e69	/* USB-A5R USB to 8 pin modular plug*/;
pub const RTSYSTEMS_USB_PW1_PID: c_uint = 0x9e6A	/* USB-PW1 USB to 8 pin modular plug*/;
//
// Physik Instrumente
// http://www.physikinstrumente.com/en/products
//
// These two devices use the VID of FTDI
pub const PI_C865_PID: c_uint = 0xe0a0  /* PI C-865 Piezomotor Controller */;
pub const PI_C857_PID: c_uint = 0xe0a1  /* PI Encoder Trigger Box */;
pub const PI_VID: c_uint = 0x1a72  /* Vendor ID */;
pub const PI_C866_PID: c_uint = 0x1000  /* PI C-866 Piezomotor Controller */;
pub const PI_C663_PID: c_uint = 0x1001  /* PI C-663 Mercury-Step */;
pub const PI_C725_PID: c_uint = 0x1002  /* PI C-725 Piezomotor Controller */;
pub const PI_E517_PID: c_uint = 0x1005  /* PI E-517 Digital Piezo Controller Operation Module */;
pub const PI_C863_PID: c_uint = 0x1007  /* PI C-863 */;
pub const PI_E861_PID: c_uint = 0x1008  /* PI E-861 Piezomotor Controller */;
pub const PI_C867_PID: c_uint = 0x1009  /* PI C-867 Piezomotor Controller */;
pub const PI_E609_PID: c_uint = 0x100D  /* PI E-609 Digital Piezo Controller */;
pub const PI_E709_PID: c_uint = 0x100E  /* PI E-709 Digital Piezo Controller */;
pub const PI_100F_PID: c_uint = 0x100F  /* PI Digital Piezo Controller */;
pub const PI_1011_PID: c_uint = 0x1011  /* PI Digital Piezo Controller */;
pub const PI_1012_PID: c_uint = 0x1012  /* PI Motion Controller */;
pub const PI_1013_PID: c_uint = 0x1013  /* PI Motion Controller */;
pub const PI_1014_PID: c_uint = 0x1014  /* PI Device */;
pub const PI_1015_PID: c_uint = 0x1015  /* PI Device */;
pub const PI_1016_PID: c_uint = 0x1016  /* PI Digital Servo Module */;
//
// Kondo Kagaku Co.Ltd.
// http://www.kondo-robot.com/EN
//
pub const KONDO_VID: c_uint = 0x165c;
pub const KONDO_USB_SERIAL_PID: c_uint = 0x0002;
//
// Bayer Ascensia Contour blood glucose meter USB-converter cable.
// http://winglucofacts.com/cables
//
pub const BAYER_VID: c_uint = 0x1A79;
pub const BAYER_CONTOUR_CABLE_PID: c_uint = 0x6001;
//
// Matrix Orbital Intelligent USB displays.
// http://www.matrixorbital.com
//
pub const MTXORB_VID: c_uint = 0x1B3D;
pub const MTXORB_FTDI_RANGE_0100_PID: c_uint = 0x0100;
pub const MTXORB_FTDI_RANGE_0101_PID: c_uint = 0x0101;
pub const MTXORB_FTDI_RANGE_0102_PID: c_uint = 0x0102;
pub const MTXORB_FTDI_RANGE_0103_PID: c_uint = 0x0103;
pub const MTXORB_FTDI_RANGE_0104_PID: c_uint = 0x0104;
pub const MTXORB_FTDI_RANGE_0105_PID: c_uint = 0x0105;
pub const MTXORB_FTDI_RANGE_0106_PID: c_uint = 0x0106;
pub const MTXORB_FTDI_RANGE_0107_PID: c_uint = 0x0107;
pub const MTXORB_FTDI_RANGE_0108_PID: c_uint = 0x0108;
pub const MTXORB_FTDI_RANGE_0109_PID: c_uint = 0x0109;
pub const MTXORB_FTDI_RANGE_010A_PID: c_uint = 0x010A;
pub const MTXORB_FTDI_RANGE_010B_PID: c_uint = 0x010B;
pub const MTXORB_FTDI_RANGE_010C_PID: c_uint = 0x010C;
pub const MTXORB_FTDI_RANGE_010D_PID: c_uint = 0x010D;
pub const MTXORB_FTDI_RANGE_010E_PID: c_uint = 0x010E;
pub const MTXORB_FTDI_RANGE_010F_PID: c_uint = 0x010F;
pub const MTXORB_FTDI_RANGE_0110_PID: c_uint = 0x0110;
pub const MTXORB_FTDI_RANGE_0111_PID: c_uint = 0x0111;
pub const MTXORB_FTDI_RANGE_0112_PID: c_uint = 0x0112;
pub const MTXORB_FTDI_RANGE_0113_PID: c_uint = 0x0113;
pub const MTXORB_FTDI_RANGE_0114_PID: c_uint = 0x0114;
pub const MTXORB_FTDI_RANGE_0115_PID: c_uint = 0x0115;
pub const MTXORB_FTDI_RANGE_0116_PID: c_uint = 0x0116;
pub const MTXORB_FTDI_RANGE_0117_PID: c_uint = 0x0117;
pub const MTXORB_FTDI_RANGE_0118_PID: c_uint = 0x0118;
pub const MTXORB_FTDI_RANGE_0119_PID: c_uint = 0x0119;
pub const MTXORB_FTDI_RANGE_011A_PID: c_uint = 0x011A;
pub const MTXORB_FTDI_RANGE_011B_PID: c_uint = 0x011B;
pub const MTXORB_FTDI_RANGE_011C_PID: c_uint = 0x011C;
pub const MTXORB_FTDI_RANGE_011D_PID: c_uint = 0x011D;
pub const MTXORB_FTDI_RANGE_011E_PID: c_uint = 0x011E;
pub const MTXORB_FTDI_RANGE_011F_PID: c_uint = 0x011F;
pub const MTXORB_FTDI_RANGE_0120_PID: c_uint = 0x0120;
pub const MTXORB_FTDI_RANGE_0121_PID: c_uint = 0x0121;
pub const MTXORB_FTDI_RANGE_0122_PID: c_uint = 0x0122;
pub const MTXORB_FTDI_RANGE_0123_PID: c_uint = 0x0123;
pub const MTXORB_FTDI_RANGE_0124_PID: c_uint = 0x0124;
pub const MTXORB_FTDI_RANGE_0125_PID: c_uint = 0x0125;
pub const MTXORB_FTDI_RANGE_0126_PID: c_uint = 0x0126;
pub const MTXORB_FTDI_RANGE_0127_PID: c_uint = 0x0127;
pub const MTXORB_FTDI_RANGE_0128_PID: c_uint = 0x0128;
pub const MTXORB_FTDI_RANGE_0129_PID: c_uint = 0x0129;
pub const MTXORB_FTDI_RANGE_012A_PID: c_uint = 0x012A;
pub const MTXORB_FTDI_RANGE_012B_PID: c_uint = 0x012B;
pub const MTXORB_FTDI_RANGE_012C_PID: c_uint = 0x012C;
pub const MTXORB_FTDI_RANGE_012D_PID: c_uint = 0x012D;
pub const MTXORB_FTDI_RANGE_012E_PID: c_uint = 0x012E;
pub const MTXORB_FTDI_RANGE_012F_PID: c_uint = 0x012F;
pub const MTXORB_FTDI_RANGE_0130_PID: c_uint = 0x0130;
pub const MTXORB_FTDI_RANGE_0131_PID: c_uint = 0x0131;
pub const MTXORB_FTDI_RANGE_0132_PID: c_uint = 0x0132;
pub const MTXORB_FTDI_RANGE_0133_PID: c_uint = 0x0133;
pub const MTXORB_FTDI_RANGE_0134_PID: c_uint = 0x0134;
pub const MTXORB_FTDI_RANGE_0135_PID: c_uint = 0x0135;
pub const MTXORB_FTDI_RANGE_0136_PID: c_uint = 0x0136;
pub const MTXORB_FTDI_RANGE_0137_PID: c_uint = 0x0137;
pub const MTXORB_FTDI_RANGE_0138_PID: c_uint = 0x0138;
pub const MTXORB_FTDI_RANGE_0139_PID: c_uint = 0x0139;
pub const MTXORB_FTDI_RANGE_013A_PID: c_uint = 0x013A;
pub const MTXORB_FTDI_RANGE_013B_PID: c_uint = 0x013B;
pub const MTXORB_FTDI_RANGE_013C_PID: c_uint = 0x013C;
pub const MTXORB_FTDI_RANGE_013D_PID: c_uint = 0x013D;
pub const MTXORB_FTDI_RANGE_013E_PID: c_uint = 0x013E;
pub const MTXORB_FTDI_RANGE_013F_PID: c_uint = 0x013F;
pub const MTXORB_FTDI_RANGE_0140_PID: c_uint = 0x0140;
pub const MTXORB_FTDI_RANGE_0141_PID: c_uint = 0x0141;
pub const MTXORB_FTDI_RANGE_0142_PID: c_uint = 0x0142;
pub const MTXORB_FTDI_RANGE_0143_PID: c_uint = 0x0143;
pub const MTXORB_FTDI_RANGE_0144_PID: c_uint = 0x0144;
pub const MTXORB_FTDI_RANGE_0145_PID: c_uint = 0x0145;
pub const MTXORB_FTDI_RANGE_0146_PID: c_uint = 0x0146;
pub const MTXORB_FTDI_RANGE_0147_PID: c_uint = 0x0147;
pub const MTXORB_FTDI_RANGE_0148_PID: c_uint = 0x0148;
pub const MTXORB_FTDI_RANGE_0149_PID: c_uint = 0x0149;
pub const MTXORB_FTDI_RANGE_014A_PID: c_uint = 0x014A;
pub const MTXORB_FTDI_RANGE_014B_PID: c_uint = 0x014B;
pub const MTXORB_FTDI_RANGE_014C_PID: c_uint = 0x014C;
pub const MTXORB_FTDI_RANGE_014D_PID: c_uint = 0x014D;
pub const MTXORB_FTDI_RANGE_014E_PID: c_uint = 0x014E;
pub const MTXORB_FTDI_RANGE_014F_PID: c_uint = 0x014F;
pub const MTXORB_FTDI_RANGE_0150_PID: c_uint = 0x0150;
pub const MTXORB_FTDI_RANGE_0151_PID: c_uint = 0x0151;
pub const MTXORB_FTDI_RANGE_0152_PID: c_uint = 0x0152;
pub const MTXORB_FTDI_RANGE_0153_PID: c_uint = 0x0153;
pub const MTXORB_FTDI_RANGE_0154_PID: c_uint = 0x0154;
pub const MTXORB_FTDI_RANGE_0155_PID: c_uint = 0x0155;
pub const MTXORB_FTDI_RANGE_0156_PID: c_uint = 0x0156;
pub const MTXORB_FTDI_RANGE_0157_PID: c_uint = 0x0157;
pub const MTXORB_FTDI_RANGE_0158_PID: c_uint = 0x0158;
pub const MTXORB_FTDI_RANGE_0159_PID: c_uint = 0x0159;
pub const MTXORB_FTDI_RANGE_015A_PID: c_uint = 0x015A;
pub const MTXORB_FTDI_RANGE_015B_PID: c_uint = 0x015B;
pub const MTXORB_FTDI_RANGE_015C_PID: c_uint = 0x015C;
pub const MTXORB_FTDI_RANGE_015D_PID: c_uint = 0x015D;
pub const MTXORB_FTDI_RANGE_015E_PID: c_uint = 0x015E;
pub const MTXORB_FTDI_RANGE_015F_PID: c_uint = 0x015F;
pub const MTXORB_FTDI_RANGE_0160_PID: c_uint = 0x0160;
pub const MTXORB_FTDI_RANGE_0161_PID: c_uint = 0x0161;
pub const MTXORB_FTDI_RANGE_0162_PID: c_uint = 0x0162;
pub const MTXORB_FTDI_RANGE_0163_PID: c_uint = 0x0163;
pub const MTXORB_FTDI_RANGE_0164_PID: c_uint = 0x0164;
pub const MTXORB_FTDI_RANGE_0165_PID: c_uint = 0x0165;
pub const MTXORB_FTDI_RANGE_0166_PID: c_uint = 0x0166;
pub const MTXORB_FTDI_RANGE_0167_PID: c_uint = 0x0167;
pub const MTXORB_FTDI_RANGE_0168_PID: c_uint = 0x0168;
pub const MTXORB_FTDI_RANGE_0169_PID: c_uint = 0x0169;
pub const MTXORB_FTDI_RANGE_016A_PID: c_uint = 0x016A;
pub const MTXORB_FTDI_RANGE_016B_PID: c_uint = 0x016B;
pub const MTXORB_FTDI_RANGE_016C_PID: c_uint = 0x016C;
pub const MTXORB_FTDI_RANGE_016D_PID: c_uint = 0x016D;
pub const MTXORB_FTDI_RANGE_016E_PID: c_uint = 0x016E;
pub const MTXORB_FTDI_RANGE_016F_PID: c_uint = 0x016F;
pub const MTXORB_FTDI_RANGE_0170_PID: c_uint = 0x0170;
pub const MTXORB_FTDI_RANGE_0171_PID: c_uint = 0x0171;
pub const MTXORB_FTDI_RANGE_0172_PID: c_uint = 0x0172;
pub const MTXORB_FTDI_RANGE_0173_PID: c_uint = 0x0173;
pub const MTXORB_FTDI_RANGE_0174_PID: c_uint = 0x0174;
pub const MTXORB_FTDI_RANGE_0175_PID: c_uint = 0x0175;
pub const MTXORB_FTDI_RANGE_0176_PID: c_uint = 0x0176;
pub const MTXORB_FTDI_RANGE_0177_PID: c_uint = 0x0177;
pub const MTXORB_FTDI_RANGE_0178_PID: c_uint = 0x0178;
pub const MTXORB_FTDI_RANGE_0179_PID: c_uint = 0x0179;
pub const MTXORB_FTDI_RANGE_017A_PID: c_uint = 0x017A;
pub const MTXORB_FTDI_RANGE_017B_PID: c_uint = 0x017B;
pub const MTXORB_FTDI_RANGE_017C_PID: c_uint = 0x017C;
pub const MTXORB_FTDI_RANGE_017D_PID: c_uint = 0x017D;
pub const MTXORB_FTDI_RANGE_017E_PID: c_uint = 0x017E;
pub const MTXORB_FTDI_RANGE_017F_PID: c_uint = 0x017F;
pub const MTXORB_FTDI_RANGE_0180_PID: c_uint = 0x0180;
pub const MTXORB_FTDI_RANGE_0181_PID: c_uint = 0x0181;
pub const MTXORB_FTDI_RANGE_0182_PID: c_uint = 0x0182;
pub const MTXORB_FTDI_RANGE_0183_PID: c_uint = 0x0183;
pub const MTXORB_FTDI_RANGE_0184_PID: c_uint = 0x0184;
pub const MTXORB_FTDI_RANGE_0185_PID: c_uint = 0x0185;
pub const MTXORB_FTDI_RANGE_0186_PID: c_uint = 0x0186;
pub const MTXORB_FTDI_RANGE_0187_PID: c_uint = 0x0187;
pub const MTXORB_FTDI_RANGE_0188_PID: c_uint = 0x0188;
pub const MTXORB_FTDI_RANGE_0189_PID: c_uint = 0x0189;
pub const MTXORB_FTDI_RANGE_018A_PID: c_uint = 0x018A;
pub const MTXORB_FTDI_RANGE_018B_PID: c_uint = 0x018B;
pub const MTXORB_FTDI_RANGE_018C_PID: c_uint = 0x018C;
pub const MTXORB_FTDI_RANGE_018D_PID: c_uint = 0x018D;
pub const MTXORB_FTDI_RANGE_018E_PID: c_uint = 0x018E;
pub const MTXORB_FTDI_RANGE_018F_PID: c_uint = 0x018F;
pub const MTXORB_FTDI_RANGE_0190_PID: c_uint = 0x0190;
pub const MTXORB_FTDI_RANGE_0191_PID: c_uint = 0x0191;
pub const MTXORB_FTDI_RANGE_0192_PID: c_uint = 0x0192;
pub const MTXORB_FTDI_RANGE_0193_PID: c_uint = 0x0193;
pub const MTXORB_FTDI_RANGE_0194_PID: c_uint = 0x0194;
pub const MTXORB_FTDI_RANGE_0195_PID: c_uint = 0x0195;
pub const MTXORB_FTDI_RANGE_0196_PID: c_uint = 0x0196;
pub const MTXORB_FTDI_RANGE_0197_PID: c_uint = 0x0197;
pub const MTXORB_FTDI_RANGE_0198_PID: c_uint = 0x0198;
pub const MTXORB_FTDI_RANGE_0199_PID: c_uint = 0x0199;
pub const MTXORB_FTDI_RANGE_019A_PID: c_uint = 0x019A;
pub const MTXORB_FTDI_RANGE_019B_PID: c_uint = 0x019B;
pub const MTXORB_FTDI_RANGE_019C_PID: c_uint = 0x019C;
pub const MTXORB_FTDI_RANGE_019D_PID: c_uint = 0x019D;
pub const MTXORB_FTDI_RANGE_019E_PID: c_uint = 0x019E;
pub const MTXORB_FTDI_RANGE_019F_PID: c_uint = 0x019F;
pub const MTXORB_FTDI_RANGE_01A0_PID: c_uint = 0x01A0;
pub const MTXORB_FTDI_RANGE_01A1_PID: c_uint = 0x01A1;
pub const MTXORB_FTDI_RANGE_01A2_PID: c_uint = 0x01A2;
pub const MTXORB_FTDI_RANGE_01A3_PID: c_uint = 0x01A3;
pub const MTXORB_FTDI_RANGE_01A4_PID: c_uint = 0x01A4;
pub const MTXORB_FTDI_RANGE_01A5_PID: c_uint = 0x01A5;
pub const MTXORB_FTDI_RANGE_01A6_PID: c_uint = 0x01A6;
pub const MTXORB_FTDI_RANGE_01A7_PID: c_uint = 0x01A7;
pub const MTXORB_FTDI_RANGE_01A8_PID: c_uint = 0x01A8;
pub const MTXORB_FTDI_RANGE_01A9_PID: c_uint = 0x01A9;
pub const MTXORB_FTDI_RANGE_01AA_PID: c_uint = 0x01AA;
pub const MTXORB_FTDI_RANGE_01AB_PID: c_uint = 0x01AB;
pub const MTXORB_FTDI_RANGE_01AC_PID: c_uint = 0x01AC;
pub const MTXORB_FTDI_RANGE_01AD_PID: c_uint = 0x01AD;
pub const MTXORB_FTDI_RANGE_01AE_PID: c_uint = 0x01AE;
pub const MTXORB_FTDI_RANGE_01AF_PID: c_uint = 0x01AF;
pub const MTXORB_FTDI_RANGE_01B0_PID: c_uint = 0x01B0;
pub const MTXORB_FTDI_RANGE_01B1_PID: c_uint = 0x01B1;
pub const MTXORB_FTDI_RANGE_01B2_PID: c_uint = 0x01B2;
pub const MTXORB_FTDI_RANGE_01B3_PID: c_uint = 0x01B3;
pub const MTXORB_FTDI_RANGE_01B4_PID: c_uint = 0x01B4;
pub const MTXORB_FTDI_RANGE_01B5_PID: c_uint = 0x01B5;
pub const MTXORB_FTDI_RANGE_01B6_PID: c_uint = 0x01B6;
pub const MTXORB_FTDI_RANGE_01B7_PID: c_uint = 0x01B7;
pub const MTXORB_FTDI_RANGE_01B8_PID: c_uint = 0x01B8;
pub const MTXORB_FTDI_RANGE_01B9_PID: c_uint = 0x01B9;
pub const MTXORB_FTDI_RANGE_01BA_PID: c_uint = 0x01BA;
pub const MTXORB_FTDI_RANGE_01BB_PID: c_uint = 0x01BB;
pub const MTXORB_FTDI_RANGE_01BC_PID: c_uint = 0x01BC;
pub const MTXORB_FTDI_RANGE_01BD_PID: c_uint = 0x01BD;
pub const MTXORB_FTDI_RANGE_01BE_PID: c_uint = 0x01BE;
pub const MTXORB_FTDI_RANGE_01BF_PID: c_uint = 0x01BF;
pub const MTXORB_FTDI_RANGE_01C0_PID: c_uint = 0x01C0;
pub const MTXORB_FTDI_RANGE_01C1_PID: c_uint = 0x01C1;
pub const MTXORB_FTDI_RANGE_01C2_PID: c_uint = 0x01C2;
pub const MTXORB_FTDI_RANGE_01C3_PID: c_uint = 0x01C3;
pub const MTXORB_FTDI_RANGE_01C4_PID: c_uint = 0x01C4;
pub const MTXORB_FTDI_RANGE_01C5_PID: c_uint = 0x01C5;
pub const MTXORB_FTDI_RANGE_01C6_PID: c_uint = 0x01C6;
pub const MTXORB_FTDI_RANGE_01C7_PID: c_uint = 0x01C7;
pub const MTXORB_FTDI_RANGE_01C8_PID: c_uint = 0x01C8;
pub const MTXORB_FTDI_RANGE_01C9_PID: c_uint = 0x01C9;
pub const MTXORB_FTDI_RANGE_01CA_PID: c_uint = 0x01CA;
pub const MTXORB_FTDI_RANGE_01CB_PID: c_uint = 0x01CB;
pub const MTXORB_FTDI_RANGE_01CC_PID: c_uint = 0x01CC;
pub const MTXORB_FTDI_RANGE_01CD_PID: c_uint = 0x01CD;
pub const MTXORB_FTDI_RANGE_01CE_PID: c_uint = 0x01CE;
pub const MTXORB_FTDI_RANGE_01CF_PID: c_uint = 0x01CF;
pub const MTXORB_FTDI_RANGE_01D0_PID: c_uint = 0x01D0;
pub const MTXORB_FTDI_RANGE_01D1_PID: c_uint = 0x01D1;
pub const MTXORB_FTDI_RANGE_01D2_PID: c_uint = 0x01D2;
pub const MTXORB_FTDI_RANGE_01D3_PID: c_uint = 0x01D3;
pub const MTXORB_FTDI_RANGE_01D4_PID: c_uint = 0x01D4;
pub const MTXORB_FTDI_RANGE_01D5_PID: c_uint = 0x01D5;
pub const MTXORB_FTDI_RANGE_01D6_PID: c_uint = 0x01D6;
pub const MTXORB_FTDI_RANGE_01D7_PID: c_uint = 0x01D7;
pub const MTXORB_FTDI_RANGE_01D8_PID: c_uint = 0x01D8;
pub const MTXORB_FTDI_RANGE_01D9_PID: c_uint = 0x01D9;
pub const MTXORB_FTDI_RANGE_01DA_PID: c_uint = 0x01DA;
pub const MTXORB_FTDI_RANGE_01DB_PID: c_uint = 0x01DB;
pub const MTXORB_FTDI_RANGE_01DC_PID: c_uint = 0x01DC;
pub const MTXORB_FTDI_RANGE_01DD_PID: c_uint = 0x01DD;
pub const MTXORB_FTDI_RANGE_01DE_PID: c_uint = 0x01DE;
pub const MTXORB_FTDI_RANGE_01DF_PID: c_uint = 0x01DF;
pub const MTXORB_FTDI_RANGE_01E0_PID: c_uint = 0x01E0;
pub const MTXORB_FTDI_RANGE_01E1_PID: c_uint = 0x01E1;
pub const MTXORB_FTDI_RANGE_01E2_PID: c_uint = 0x01E2;
pub const MTXORB_FTDI_RANGE_01E3_PID: c_uint = 0x01E3;
pub const MTXORB_FTDI_RANGE_01E4_PID: c_uint = 0x01E4;
pub const MTXORB_FTDI_RANGE_01E5_PID: c_uint = 0x01E5;
pub const MTXORB_FTDI_RANGE_01E6_PID: c_uint = 0x01E6;
pub const MTXORB_FTDI_RANGE_01E7_PID: c_uint = 0x01E7;
pub const MTXORB_FTDI_RANGE_01E8_PID: c_uint = 0x01E8;
pub const MTXORB_FTDI_RANGE_01E9_PID: c_uint = 0x01E9;
pub const MTXORB_FTDI_RANGE_01EA_PID: c_uint = 0x01EA;
pub const MTXORB_FTDI_RANGE_01EB_PID: c_uint = 0x01EB;
pub const MTXORB_FTDI_RANGE_01EC_PID: c_uint = 0x01EC;
pub const MTXORB_FTDI_RANGE_01ED_PID: c_uint = 0x01ED;
pub const MTXORB_FTDI_RANGE_01EE_PID: c_uint = 0x01EE;
pub const MTXORB_FTDI_RANGE_01EF_PID: c_uint = 0x01EF;
pub const MTXORB_FTDI_RANGE_01F0_PID: c_uint = 0x01F0;
pub const MTXORB_FTDI_RANGE_01F1_PID: c_uint = 0x01F1;
pub const MTXORB_FTDI_RANGE_01F2_PID: c_uint = 0x01F2;
pub const MTXORB_FTDI_RANGE_01F3_PID: c_uint = 0x01F3;
pub const MTXORB_FTDI_RANGE_01F4_PID: c_uint = 0x01F4;
pub const MTXORB_FTDI_RANGE_01F5_PID: c_uint = 0x01F5;
pub const MTXORB_FTDI_RANGE_01F6_PID: c_uint = 0x01F6;
pub const MTXORB_FTDI_RANGE_01F7_PID: c_uint = 0x01F7;
pub const MTXORB_FTDI_RANGE_01F8_PID: c_uint = 0x01F8;
pub const MTXORB_FTDI_RANGE_01F9_PID: c_uint = 0x01F9;
pub const MTXORB_FTDI_RANGE_01FA_PID: c_uint = 0x01FA;
pub const MTXORB_FTDI_RANGE_01FB_PID: c_uint = 0x01FB;
pub const MTXORB_FTDI_RANGE_01FC_PID: c_uint = 0x01FC;
pub const MTXORB_FTDI_RANGE_01FD_PID: c_uint = 0x01FD;
pub const MTXORB_FTDI_RANGE_01FE_PID: c_uint = 0x01FE;
pub const MTXORB_FTDI_RANGE_01FF_PID: c_uint = 0x01FF;
pub const MTXORB_FTDI_RANGE_4701_PID: c_uint = 0x4701;
pub const MTXORB_FTDI_RANGE_9300_PID: c_uint = 0x9300;
pub const MTXORB_FTDI_RANGE_9301_PID: c_uint = 0x9301;
pub const MTXORB_FTDI_RANGE_9302_PID: c_uint = 0x9302;
pub const MTXORB_FTDI_RANGE_9303_PID: c_uint = 0x9303;
pub const MTXORB_FTDI_RANGE_9304_PID: c_uint = 0x9304;
pub const MTXORB_FTDI_RANGE_9305_PID: c_uint = 0x9305;
pub const MTXORB_FTDI_RANGE_9306_PID: c_uint = 0x9306;
pub const MTXORB_FTDI_RANGE_9307_PID: c_uint = 0x9307;
pub const MTXORB_FTDI_RANGE_9308_PID: c_uint = 0x9308;
pub const MTXORB_FTDI_RANGE_9309_PID: c_uint = 0x9309;
pub const MTXORB_FTDI_RANGE_930A_PID: c_uint = 0x930A;
pub const MTXORB_FTDI_RANGE_930B_PID: c_uint = 0x930B;
pub const MTXORB_FTDI_RANGE_930C_PID: c_uint = 0x930C;
pub const MTXORB_FTDI_RANGE_930D_PID: c_uint = 0x930D;
pub const MTXORB_FTDI_RANGE_930E_PID: c_uint = 0x930E;
pub const MTXORB_FTDI_RANGE_930F_PID: c_uint = 0x930F;
pub const MTXORB_FTDI_RANGE_9310_PID: c_uint = 0x9310;
pub const MTXORB_FTDI_RANGE_9311_PID: c_uint = 0x9311;
pub const MTXORB_FTDI_RANGE_9312_PID: c_uint = 0x9312;
pub const MTXORB_FTDI_RANGE_9313_PID: c_uint = 0x9313;
pub const MTXORB_FTDI_RANGE_9314_PID: c_uint = 0x9314;
pub const MTXORB_FTDI_RANGE_9315_PID: c_uint = 0x9315;
pub const MTXORB_FTDI_RANGE_9316_PID: c_uint = 0x9316;
pub const MTXORB_FTDI_RANGE_9317_PID: c_uint = 0x9317;
pub const MTXORB_FTDI_RANGE_9318_PID: c_uint = 0x9318;
pub const MTXORB_FTDI_RANGE_9319_PID: c_uint = 0x9319;
pub const MTXORB_FTDI_RANGE_931A_PID: c_uint = 0x931A;
pub const MTXORB_FTDI_RANGE_931B_PID: c_uint = 0x931B;
pub const MTXORB_FTDI_RANGE_931C_PID: c_uint = 0x931C;
pub const MTXORB_FTDI_RANGE_931D_PID: c_uint = 0x931D;
pub const MTXORB_FTDI_RANGE_931E_PID: c_uint = 0x931E;
pub const MTXORB_FTDI_RANGE_931F_PID: c_uint = 0x931F;
//
// The Mobility Lab (TML)
// Submitted by Pierre Castella
//
pub const TML_VID: c_uint = 0x1B91	/* Vendor ID */;
pub const TML_USB_SERIAL_PID: c_uint = 0x0064	/* USB - Serial Converter */;
// Alti-2 products  http://www.alti-2.com
pub const ALTI2_VID: c_uint = 0x1BC9;
pub const ALTI2_N3_PID: c_uint = 0x6001	/* Neptune 3 */;
//
// Ionics PlugComputer
//
pub const IONICS_VID: c_uint = 0x1c0c;
pub const IONICS_PLUGCOMPUTER_PID: c_uint = 0x0102;
//
// EZPrototypes (PID reseller)
//
pub const EZPROTOTYPES_VID: c_uint = 0x1c40;
pub const HJELMSLUND_USB485_ISO_PID: c_uint = 0x0477;
//
// Dresden Elektronik Sensor Terminal Board
//
pub const DE_VID: c_uint = 0x1cf1 /* Vendor ID */;
pub const STB_PID: c_uint = 0x0001 /* Sensor Terminal Board */;
pub const WHT_PID: c_uint = 0x0004 /* Wireless Handheld Terminal */;
//
// STMicroelectonics
//
pub const ST_VID: c_uint = 0x0483;
pub const ST_STMCLT_2232_PID: c_uint = 0x3746;
pub const ST_STMCLT_4232_PID: c_uint = 0x3747;
//
// Papouch products (http://www.papouch.com/)
// Submitted by Folkert van Heusden
//
pub const PAPOUCH_VID: c_uint = 0x5050	/* Vendor ID */;
pub const PAPOUCH_SB485_PID: c_uint = 0x0100	/* Papouch SB485 USB-485/422 Converter */;
pub const PAPOUCH_AP485_PID: c_uint = 0x0101	/* AP485 USB-RS485 Converter */;
pub const PAPOUCH_SB422_PID: c_uint = 0x0102	/* Papouch SB422 USB-RS422 Converter  */;
pub const PAPOUCH_SB485_2_PID: c_uint = 0x0103	/* Papouch SB485 USB-485/422 Converter */;
pub const PAPOUCH_AP485_2_PID: c_uint = 0x0104	/* AP485 USB-RS485 Converter */;
pub const PAPOUCH_SB422_2_PID: c_uint = 0x0105	/* Papouch SB422 USB-RS422 Converter  */;
pub const PAPOUCH_SB485S_PID: c_uint = 0x0106	/* Papouch SB485S USB-485/422 Converter */;
pub const PAPOUCH_SB485C_PID: c_uint = 0x0107	/* Papouch SB485C USB-485/422 Converter */;
pub const PAPOUCH_LEC_PID: c_uint = 0x0300	/* LEC USB Converter */;
pub const PAPOUCH_SB232_PID: c_uint = 0x0301	/* Papouch SB232 USB-RS232 Converter */;
pub const PAPOUCH_TMU_PID: c_uint = 0x0400	/* TMU USB Thermometer */;
pub const PAPOUCH_IRAMP_PID: c_uint = 0x0500	/* Papouch IRAmp Duplex */;
pub const PAPOUCH_DRAK5_PID: c_uint = 0x0700	/* Papouch DRAK5 */;
pub const PAPOUCH_QUIDO8x8_PID: c_uint = 0x0800	/* Papouch Quido 8/8 Module */;
pub const PAPOUCH_QUIDO4x4_PID: c_uint = 0x0900	/* Papouch Quido 4/4 Module */;
pub const PAPOUCH_QUIDO2x2_PID: c_uint = 0x0a00	/* Papouch Quido 2/2 Module */;
pub const PAPOUCH_QUIDO10x1_PID: c_uint = 0x0b00	/* Papouch Quido 10/1 Module */;
pub const PAPOUCH_QUIDO30x3_PID: c_uint = 0x0c00	/* Papouch Quido 30/3 Module */;
pub const PAPOUCH_QUIDO60x3_PID: c_uint = 0x0d00	/* Papouch Quido 60(100)/3 Module */;
pub const PAPOUCH_QUIDO2x16_PID: c_uint = 0x0e00	/* Papouch Quido 2/16 Module */;
pub const PAPOUCH_QUIDO3x32_PID: c_uint = 0x0f00	/* Papouch Quido 3/32 Module */;
pub const PAPOUCH_DRAK6_PID: c_uint = 0x1000	/* Papouch DRAK6 */;
pub const PAPOUCH_UPSUSB_PID: c_uint = 0x8000	/* Papouch UPS-USB adapter */;
pub const PAPOUCH_MU_PID: c_uint = 0x8001	/* MU controller */;
pub const PAPOUCH_SIMUKEY_PID: c_uint = 0x8002	/* Papouch SimuKey */;
pub const PAPOUCH_AD4USB_PID: c_uint = 0x8003	/* AD4USB Measurement Module */;
pub const PAPOUCH_GMUX_PID: c_uint = 0x8004	/* Papouch GOLIATH MUX */;
pub const PAPOUCH_GMSR_PID: c_uint = 0x8005	/* Papouch GOLIATH MSR */;
//
// Marvell SheevaPlug
//
pub const MARVELL_VID: c_uint = 0x9e88;
pub const MARVELL_SHEEVAPLUG_PID: c_uint = 0x9e8f;
//
// Evolution Robotics products (http://www.evolution.com/).
// Submitted by Shawn M. Lavelle.
//
pub const EVOLUTION_VID: c_uint = 0xDEEE	/* Vendor ID */;
pub const EVOLUTION_ER1_PID: c_uint = 0x0300	/* ER1 Control Module */;
pub const EVO_8U232AM_PID: c_uint = 0x02FF	/* Evolution robotics RCM2 (FT232AM)*/;
pub const EVO_HYBRID_PID: c_uint = 0x0302	/* Evolution robotics RCM4 PID (FT232BM)*/;
pub const EVO_RCM4_PID: c_uint = 0x0303	/* Evolution robotics RCM4 PID */;
//
// MJS Gadgets HD Radio / XM Radio / Sirius Radio interfaces (using VID 0x0403)
//
pub const MJSG_GENERIC_PID: c_uint = 0x9378;
pub const MJSG_SR_RADIO_PID: c_uint = 0x9379;
pub const MJSG_XM_RADIO_PID: c_uint = 0x937A;
pub const MJSG_HD_RADIO_PID: c_uint = 0x937C;
//
// D.O.Tec products (http://www.directout.eu)
//
pub const FTDI_DOTEC_PID: c_uint = 0x9868;
//
// Xverve Signalyzer tools (http://www.signalyzer.com/)
//
pub const XVERVE_SIGNALYZER_ST_PID: c_uint = 0xBCA0;
pub const XVERVE_SIGNALYZER_SLITE_PID: c_uint = 0xBCA1;
pub const XVERVE_SIGNALYZER_SH2_PID: c_uint = 0xBCA2;
pub const XVERVE_SIGNALYZER_SH4_PID: c_uint = 0xBCA4;
//
// Segway Robotic Mobility Platform USB interface (using VID 0x0403)
// Submitted by John G. Rogers
//
pub const SEGWAY_RMP200_PID: c_uint = 0xe729;
//
// Accesio USB Data Acquisition products (http://www.accesio.com/)
//
pub const ACCESIO_COM4SM_PID: c_uint = 0xD578;
// www.sciencescope.co.uk educational dataloggers
pub const FTDI_SCIENCESCOPE_LOGBOOKML_PID: c_uint = 0xFF18;
pub const FTDI_SCIENCESCOPE_LS_LOGBOOK_PID: c_uint = 0xFF1C;
pub const FTDI_SCIENCESCOPE_HS_LOGBOOK_PID: c_uint = 0xFF1D;
//
// Milkymist One JTAG/Serial
//
pub const QIHARDWARE_VID: c_uint = 0x20B7;
pub const MILKYMISTONE_JTAGSERIAL_PID: c_uint = 0x0713;
//
// CTI GmbH RS485 Converter http://www.cti-lean.com
//
// USB-485-Mini
pub const FTDI_CTI_MINI_PID: c_uint = 0xF608;
// USB-Nano-485
pub const FTDI_CTI_NANO_PID: c_uint = 0xF60B;
//
// ZeitControl cardsystems GmbH rfid-readers http://zeitcontrol.de
//
// TagTracer MIFARE
pub const FTDI_ZEITCONTROL_TAGTRACE_MIFARE_PID: c_uint = 0xF7C0;
//
// Rainforest Automation
//
// ZigBee controller
pub const FTDI_RF_R106: c_uint = 0x8A28;
//
// Product: HCP HIT GPRS modem
// Manufacturer: HCP d.o.o.
// ATI command output: Cinterion MC55i
//
pub const FTDI_CINTERION_MC55I_PID: c_uint = 0xA951;
//
// Product: FirmwareHubEmulator
// Manufacturer: Harman Becker Automotive Systems
//
pub const FTDI_FHE_PID: c_uint = 0xA9A0;
//
// Product: Comet Caller ID decoder
// Manufacturer: Crucible Technologies
//
pub const FTDI_CT_COMET_PID: c_uint = 0x8e08;
//
// Product: Z3X Box
// Manufacturer: Smart GSM Team
//
pub const FTDI_Z3X_PID: c_uint = 0x0011;
//
// Product: Cressi PC Interface
// Manufacturer: Cressi
//
pub const FTDI_CRESSI_PID: c_uint = 0x87d0;
//
// Brainboxes devices
//
pub const BRAINBOXES_VID: c_uint = 0x05d1;
pub const BRAINBOXES_VX_001_PID: c_uint = 0x1001 /* VX-001 ExpressCard 1 Port RS232 */;
pub const BRAINBOXES_VX_012_PID: c_uint = 0x1002 /* VX-012 ExpressCard 2 Port RS232 */;
pub const BRAINBOXES_VX_023_PID: c_uint = 0x1003 /* VX-023 ExpressCard 1 Port RS422/485 */;
pub const BRAINBOXES_VX_034_PID: c_uint = 0x1004 /* VX-034 ExpressCard 2 Port RS422/485 */;
pub const BRAINBOXES_US_101_PID: c_uint = 0x1011 /* US-101 1xRS232 */;
pub const BRAINBOXES_US_159_PID: c_uint = 0x1021 /* US-159 1xRS232 */;
pub const BRAINBOXES_US_235_PID: c_uint = 0x1017 /* US-235 1xRS232 */;
pub const BRAINBOXES_US_320_PID: c_uint = 0x1019 /* US-320 1xRS422/485 */;
pub const BRAINBOXES_US_324_PID: c_uint = 0x1013 /* US-324 1xRS422/485 1Mbaud */;
pub const BRAINBOXES_US_606_1_PID: c_uint = 0x2001 /* US-606 6 Port RS232 Serial Port 1 and 2 */;
pub const BRAINBOXES_US_606_2_PID: c_uint = 0x2002 /* US-606 6 Port RS232 Serial Port 3 and 4 */;
pub const BRAINBOXES_US_606_3_PID: c_uint = 0x2003 /* US-606 6 Port RS232 Serial Port 4 and 6 */;
pub const BRAINBOXES_US_701_1_PID: c_uint = 0x2011 /* US-701 4xRS232 1Mbaud Port 1 and 2 */;
pub const BRAINBOXES_US_701_2_PID: c_uint = 0x2012 /* US-701 4xRS422 1Mbaud Port 3 and 4 */;
pub const BRAINBOXES_US_279_1_PID: c_uint = 0x2021 /* US-279 8xRS422 1Mbaud Port 1 and 2 */;
pub const BRAINBOXES_US_279_2_PID: c_uint = 0x2022 /* US-279 8xRS422 1Mbaud Port 3 and 4 */;
pub const BRAINBOXES_US_279_3_PID: c_uint = 0x2023 /* US-279 8xRS422 1Mbaud Port 5 and 6 */;
pub const BRAINBOXES_US_279_4_PID: c_uint = 0x2024 /* US-279 8xRS422 1Mbaud Port 7 and 8 */;
pub const BRAINBOXES_US_346_1_PID: c_uint = 0x3011 /* US-346 4xRS422/485 1Mbaud Port 1 and 2 */;
pub const BRAINBOXES_US_346_2_PID: c_uint = 0x3012 /* US-346 4xRS422/485 1Mbaud Port 3 and 4 */;
pub const BRAINBOXES_US_257_PID: c_uint = 0x5001 /* US-257 2xRS232 1Mbaud */;
pub const BRAINBOXES_US_313_PID: c_uint = 0x6001 /* US-313 2xRS422/485 1Mbaud */;
pub const BRAINBOXES_US_357_PID: c_uint = 0x7001 /* US_357 1xRS232/422/485 */;
pub const BRAINBOXES_US_842_1_PID: c_uint = 0x8001 /* US-842 8xRS422/485 1Mbaud Port 1 and 2 */;
pub const BRAINBOXES_US_842_2_PID: c_uint = 0x8002 /* US-842 8xRS422/485 1Mbaud Port 3 and 4 */;
pub const BRAINBOXES_US_842_3_PID: c_uint = 0x8003 /* US-842 8xRS422/485 1Mbaud Port 5 and 6 */;
pub const BRAINBOXES_US_842_4_PID: c_uint = 0x8004 /* US-842 8xRS422/485 1Mbaud Port 7 and 8 */;
pub const BRAINBOXES_US_160_1_PID: c_uint = 0x9001 /* US-160 16xRS232 1Mbaud Port 1 and 2 */;
pub const BRAINBOXES_US_160_2_PID: c_uint = 0x9002 /* US-160 16xRS232 1Mbaud Port 3 and 4 */;
pub const BRAINBOXES_US_160_3_PID: c_uint = 0x9003 /* US-160 16xRS232 1Mbaud Port 5 and 6 */;
pub const BRAINBOXES_US_160_4_PID: c_uint = 0x9004 /* US-160 16xRS232 1Mbaud Port 7 and 8 */;
pub const BRAINBOXES_US_160_5_PID: c_uint = 0x9005 /* US-160 16xRS232 1Mbaud Port 9 and 10 */;
pub const BRAINBOXES_US_160_6_PID: c_uint = 0x9006 /* US-160 16xRS232 1Mbaud Port 11 and 12 */;
pub const BRAINBOXES_US_160_7_PID: c_uint = 0x9007 /* US-160 16xRS232 1Mbaud Port 13 and 14 */;
pub const BRAINBOXES_US_160_8_PID: c_uint = 0x9008 /* US-160 16xRS232 1Mbaud Port 15 and 16 */;
//
// ekey biometric systems GmbH (http://ekey.net/)
//
pub const FTDI_EKEY_CONV_USB_PID: c_uint = 0xCB08	/* Converter USB */;
//
// GE Healthcare devices
//
pub const GE_HEALTHCARE_VID: c_uint = 0x1901;
pub const GE_HEALTHCARE_NEMO_TRACKER_PID: c_uint = 0x0015;
//
// Active Research (Actisense) devices
//
pub const ACTISENSE_NDC_PID: c_uint = 0xD9A8 /* NDC USB Serial Adapter */;
pub const ACTISENSE_USG_PID: c_uint = 0xD9A9 /* USG USB Serial Adapter */;
pub const ACTISENSE_NGT_PID: c_uint = 0xD9AA /* NGT NMEA2000 Interface */;
pub const ACTISENSE_NGW_PID: c_uint = 0xD9AB /* NGW NMEA2000 Gateway */;
pub const ACTISENSE_UID_PID: c_uint = 0xD9AC /* USB Isolating Device */;
pub const ACTISENSE_USA_PID: c_uint = 0xD9AD /* USB to Serial Adapter */;
pub const ACTISENSE_NGX_PID: c_uint = 0xD9AE /* NGX NMEA2000 Gateway */;
pub const ACTISENSE_D9AF_PID: c_uint = 0xD9AF /* Actisense Reserved */;
pub const CHETCO_SEAGAUGE_PID: c_uint = 0xA548 /* SeaGauge USB Adapter */;
pub const CHETCO_SEASWITCH_PID: c_uint = 0xA549 /* SeaSwitch USB Adapter */;
pub const CHETCO_SEASMART_NMEA2000_PID: c_uint = 0xA54A /* SeaSmart NMEA2000 Gateway */;
pub const CHETCO_SEASMART_ETHERNET_PID: c_uint = 0xA54B /* SeaSmart Ethernet Gateway */;
pub const CHETCO_SEASMART_WIFI_PID: c_uint = 0xA5AC /* SeaSmart Wifi Gateway */;
pub const CHETCO_SEASMART_DISPLAY_PID: c_uint = 0xA5AD /* SeaSmart NMEA2000 Display */;
pub const CHETCO_SEASMART_LITE_PID: c_uint = 0xA5AE /* SeaSmart Lite USB Adapter */;
pub const CHETCO_SEASMART_ANALOG_PID: c_uint = 0xA5AF /* SeaSmart Analog Adapter */;
//
// Belimo Automation
//
pub const BELIMO_ZTH_PID: c_uint = 0x8050;
pub const BELIMO_ZIP_PID: c_uint = 0xC811;
//
// Unjo AB
//
pub const UNJO_VID: c_uint = 0x22B7;
pub const UNJO_ISODEBUG_V1_PID: c_uint = 0x150D;
//
// IDS GmbH
//
pub const IDS_VID: c_uint = 0x2CAF;
pub const IDS_SI31A_PID: c_uint = 0x13A2;
pub const IDS_CM31A_PID: c_uint = 0x13A3;
//
// U-Blox products (http://www.u-blox.com).
//
pub const UBLOX_VID: c_uint = 0x1546;
pub const UBLOX_C099F9P_ZED_PID: c_uint = 0x0502;
pub const UBLOX_C099F9P_ODIN_PID: c_uint = 0x0503;
pub const UBLOX_EVK_M101_PID: c_uint = 0x0506;
//
// GMC devices
//
pub const GMC_VID: c_uint = 0x1cd7;
pub const GMC_Z216C_PID: c_uint = 0x0217 /* GMC Z216C Adapter IR-USB */;
//
// Altera USB Blaster 3 (http://www.altera.com).
//
pub const ALTERA_VID: c_uint = 0x09fb;
pub const ALTERA_UB3_6022_PID: c_uint = 0x6022;
pub const ALTERA_UB3_6025_PID: c_uint = 0x6025;
pub const ALTERA_UB3_6026_PID: c_uint = 0x6026;
pub const ALTERA_UB3_6029_PID: c_uint = 0x6029;
pub const ALTERA_UB3_602A_PID: c_uint = 0x602a;
pub const ALTERA_UB3_602C_PID: c_uint = 0x602c;
pub const ALTERA_UB3_602D_PID: c_uint = 0x602d;
pub const ALTERA_UB3_602E_PID: c_uint = 0x602e;
