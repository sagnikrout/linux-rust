//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/option.c
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
    USB Driver for GSM modems
    Copyright (C) 2005  Matthias Urlichs <smurf@smurf.noris.de>
    Portions copied from the Keyspan driver by Hugh Blemings <hugh@blemings.org>
    History: see the git log.
    Work sponsored by: Sigos GmbH, Germany <info@sigos.de>
    This driver exists because the "normal" serial driver doesn't work too well
    with GSM modems. Issues:
    - data loss -- one single Receive URB is not nearly enough
    - nonstandard flow (Option devices) control
    - controlling the baud rate doesn't make sense
    This driver is named "option" because the most common device it's
    used for is a PC-Card (with an internal OHCI-USB interface, behind
    which the GSM interface sits), made by Option Inc.
    Some of the "one port" devices actually exhibit multiple USB instances
    on the USB bus. This is not a bug, these ports are used for different
    device features.
//

// Function prototypes
    static int  option_probe(struct usb_serial *serial,
    const struct usb_device_id *id);
    static int option_attach(struct usb_serial *serial);
    static void option_release(struct usb_serial *serial);
    static void option_instat_callback(struct urb *urb);
// Vendor and product IDs
pub const OPTION_VENDOR_ID: c_uint = 0x0AF0;
pub const OPTION_PRODUCT_COLT: c_uint = 0x5000;
pub const OPTION_PRODUCT_RICOLA: c_uint = 0x6000;
pub const OPTION_PRODUCT_RICOLA_LIGHT: c_uint = 0x6100;
pub const OPTION_PRODUCT_RICOLA_QUAD: c_uint = 0x6200;
pub const OPTION_PRODUCT_RICOLA_QUAD_LIGHT: c_uint = 0x6300;
pub const OPTION_PRODUCT_RICOLA_NDIS: c_uint = 0x6050;
pub const OPTION_PRODUCT_RICOLA_NDIS_LIGHT: c_uint = 0x6150;
pub const OPTION_PRODUCT_RICOLA_NDIS_QUAD: c_uint = 0x6250;
pub const OPTION_PRODUCT_RICOLA_NDIS_QUAD_LIGHT: c_uint = 0x6350;
pub const OPTION_PRODUCT_COBRA: c_uint = 0x6500;
pub const OPTION_PRODUCT_COBRA_BUS: c_uint = 0x6501;
pub const OPTION_PRODUCT_VIPER: c_uint = 0x6600;
pub const OPTION_PRODUCT_VIPER_BUS: c_uint = 0x6601;
pub const OPTION_PRODUCT_GT_MAX_READY: c_uint = 0x6701;
pub const OPTION_PRODUCT_FUJI_MODEM_LIGHT: c_uint = 0x6721;
pub const OPTION_PRODUCT_FUJI_MODEM_GT: c_uint = 0x6741;
pub const OPTION_PRODUCT_FUJI_MODEM_EX: c_uint = 0x6761;
pub const OPTION_PRODUCT_KOI_MODEM: c_uint = 0x6800;
pub const OPTION_PRODUCT_SCORPION_MODEM: c_uint = 0x6901;
pub const OPTION_PRODUCT_ETNA_MODEM: c_uint = 0x7001;
pub const OPTION_PRODUCT_ETNA_MODEM_LITE: c_uint = 0x7021;
pub const OPTION_PRODUCT_ETNA_MODEM_GT: c_uint = 0x7041;
pub const OPTION_PRODUCT_ETNA_MODEM_EX: c_uint = 0x7061;
pub const OPTION_PRODUCT_ETNA_KOI_MODEM: c_uint = 0x7100;
pub const OPTION_PRODUCT_GTM380_MODEM: c_uint = 0x7201;
pub const HUAWEI_VENDOR_ID: c_uint = 0x12D1;
pub const HUAWEI_PRODUCT_E173: c_uint = 0x140C;
pub const HUAWEI_PRODUCT_E1750: c_uint = 0x1406;
pub const HUAWEI_PRODUCT_K4505: c_uint = 0x1464;
pub const HUAWEI_PRODUCT_K3765: c_uint = 0x1465;
pub const HUAWEI_PRODUCT_K4605: c_uint = 0x14C6;
pub const HUAWEI_PRODUCT_E173S6: c_uint = 0x1C07;
pub const QUANTA_VENDOR_ID: c_uint = 0x0408;
pub const QUANTA_PRODUCT_Q101: c_uint = 0xEA02;
pub const QUANTA_PRODUCT_Q111: c_uint = 0xEA03;
pub const QUANTA_PRODUCT_GLX: c_uint = 0xEA04;
pub const QUANTA_PRODUCT_GKE: c_uint = 0xEA05;
pub const QUANTA_PRODUCT_GLE: c_uint = 0xEA06;
pub const NOVATELWIRELESS_VENDOR_ID: c_uint = 0x1410;
// YISO PRODUCTS
pub const YISO_VENDOR_ID: c_uint = 0x0EAB;
pub const YISO_PRODUCT_U893: c_uint = 0xC893;
//
// NOVATEL WIRELESS PRODUCTS
//
// Note from Novatel Wireless:
// If your Novatel modem does not work on linux, don't
// change the option module, but check our website. If
// that does not help, contact ddeschepper@nvtl.com
//
// MERLIN EVDO PRODUCTS
pub const NOVATELWIRELESS_PRODUCT_V640: c_uint = 0x1100;
pub const NOVATELWIRELESS_PRODUCT_V620: c_uint = 0x1110;
pub const NOVATELWIRELESS_PRODUCT_V740: c_uint = 0x1120;
pub const NOVATELWIRELESS_PRODUCT_V720: c_uint = 0x1130;
// MERLIN HSDPA/HSPA PRODUCTS
pub const NOVATELWIRELESS_PRODUCT_U730: c_uint = 0x1400;
pub const NOVATELWIRELESS_PRODUCT_U740: c_uint = 0x1410;
pub const NOVATELWIRELESS_PRODUCT_U870: c_uint = 0x1420;
pub const NOVATELWIRELESS_PRODUCT_XU870: c_uint = 0x1430;
pub const NOVATELWIRELESS_PRODUCT_X950D: c_uint = 0x1450;
// EXPEDITE PRODUCTS
pub const NOVATELWIRELESS_PRODUCT_EV620: c_uint = 0x2100;
pub const NOVATELWIRELESS_PRODUCT_ES720: c_uint = 0x2110;
pub const NOVATELWIRELESS_PRODUCT_E725: c_uint = 0x2120;
pub const NOVATELWIRELESS_PRODUCT_ES620: c_uint = 0x2130;
pub const NOVATELWIRELESS_PRODUCT_EU730: c_uint = 0x2400;
pub const NOVATELWIRELESS_PRODUCT_EU740: c_uint = 0x2410;
pub const NOVATELWIRELESS_PRODUCT_EU870D: c_uint = 0x2420;
// OVATION PRODUCTS
pub const NOVATELWIRELESS_PRODUCT_MC727: c_uint = 0x4100;
pub const NOVATELWIRELESS_PRODUCT_MC950D: c_uint = 0x4400;
//
// Note from Novatel Wireless:
// All PID in the 5xxx range are currently reserved for
// auto-install CDROMs, and should not be added to this
// module.
//
// #define NOVATELWIRELESS_PRODUCT_U727		0x5010
// #define NOVATELWIRELESS_PRODUCT_MC727_NEW	0x5100
//
pub const NOVATELWIRELESS_PRODUCT_OVMC760: c_uint = 0x6002;
pub const NOVATELWIRELESS_PRODUCT_MC780: c_uint = 0x6010;
pub const NOVATELWIRELESS_PRODUCT_EVDO_FULLSPEED: c_uint = 0x6000;
pub const NOVATELWIRELESS_PRODUCT_EVDO_HIGHSPEED: c_uint = 0x6001;
pub const NOVATELWIRELESS_PRODUCT_HSPA_FULLSPEED: c_uint = 0x7000;
pub const NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED: c_uint = 0x7001;
pub const NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED3: c_uint = 0x7003;
pub const NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED4: c_uint = 0x7004;
pub const NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED5: c_uint = 0x7005;
pub const NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED6: c_uint = 0x7006;
pub const NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED7: c_uint = 0x7007;
pub const NOVATELWIRELESS_PRODUCT_MC996D: c_uint = 0x7030;
pub const NOVATELWIRELESS_PRODUCT_MF3470: c_uint = 0x7041;
pub const NOVATELWIRELESS_PRODUCT_MC547: c_uint = 0x7042;
pub const NOVATELWIRELESS_PRODUCT_EVDO_EMBEDDED_FULLSPEED: c_uint = 0x8000;
pub const NOVATELWIRELESS_PRODUCT_EVDO_EMBEDDED_HIGHSPEED: c_uint = 0x8001;
pub const NOVATELWIRELESS_PRODUCT_HSPA_EMBEDDED_FULLSPEED: c_uint = 0x9000;
pub const NOVATELWIRELESS_PRODUCT_HSPA_EMBEDDED_HIGHSPEED: c_uint = 0x9001;
pub const NOVATELWIRELESS_PRODUCT_E362: c_uint = 0x9010;
pub const NOVATELWIRELESS_PRODUCT_E371: c_uint = 0x9011;
pub const NOVATELWIRELESS_PRODUCT_U620L: c_uint = 0x9022;
pub const NOVATELWIRELESS_PRODUCT_G2: c_uint = 0xA010;
pub const NOVATELWIRELESS_PRODUCT_MC551: c_uint = 0xB001;
pub const UBLOX_VENDOR_ID: c_uint = 0x1546;
// AMOI PRODUCTS
pub const AMOI_VENDOR_ID: c_uint = 0x1614;
pub const AMOI_PRODUCT_H01: c_uint = 0x0800;
pub const AMOI_PRODUCT_H01A: c_uint = 0x7002;
pub const AMOI_PRODUCT_H02: c_uint = 0x0802;
pub const AMOI_PRODUCT_SKYPEPHONE_S2: c_uint = 0x0407;
pub const DELL_VENDOR_ID: c_uint = 0x413C;
// Dell modems
pub const DELL_PRODUCT_5700_MINICARD: c_uint = 0x8114;
pub const DELL_PRODUCT_5500_MINICARD: c_uint = 0x8115;
pub const DELL_PRODUCT_5505_MINICARD: c_uint = 0x8116;
pub const DELL_PRODUCT_5700_EXPRESSCARD: c_uint = 0x8117;
pub const DELL_PRODUCT_5510_EXPRESSCARD: c_uint = 0x8118;
pub const DELL_PRODUCT_5700_MINICARD_SPRINT: c_uint = 0x8128;
pub const DELL_PRODUCT_5700_MINICARD_TELUS: c_uint = 0x8129;
pub const DELL_PRODUCT_5720_MINICARD_VZW: c_uint = 0x8133;
pub const DELL_PRODUCT_5720_MINICARD_SPRINT: c_uint = 0x8134;
pub const DELL_PRODUCT_5720_MINICARD_TELUS: c_uint = 0x8135;
pub const DELL_PRODUCT_5520_MINICARD_CINGULAR: c_uint = 0x8136;
pub const DELL_PRODUCT_5520_MINICARD_GENERIC_L: c_uint = 0x8137;
pub const DELL_PRODUCT_5520_MINICARD_GENERIC_I: c_uint = 0x8138;
pub const DELL_PRODUCT_5730_MINICARD_SPRINT: c_uint = 0x8180;
pub const DELL_PRODUCT_5730_MINICARD_TELUS: c_uint = 0x8181;
pub const DELL_PRODUCT_5730_MINICARD_VZW: c_uint = 0x8182;
pub const DELL_PRODUCT_5800_MINICARD_VZW: c_uint = 0x8195  /* Novatel E362 */;
pub const DELL_PRODUCT_5800_V2_MINICARD_VZW: c_uint = 0x8196  /* Novatel E362 */;
pub const DELL_PRODUCT_5804_MINICARD_ATT: c_uint = 0x819b  /* Novatel E371 */;
pub const DELL_PRODUCT_5821E: c_uint = 0x81d7;
pub const DELL_PRODUCT_5821E_ESIM: c_uint = 0x81e0;
pub const DELL_PRODUCT_5829E_ESIM: c_uint = 0x81e4;
pub const DELL_PRODUCT_5829E: c_uint = 0x81e6;
pub const DELL_PRODUCT_5826E_ESIM: c_uint = 0x81ea;
pub const DELL_PRODUCT_FM101R_ESIM: c_uint = 0x8213;
pub const DELL_PRODUCT_FM101R: c_uint = 0x8215;
pub const KYOCERA_VENDOR_ID: c_uint = 0x0c88;
pub const KYOCERA_PRODUCT_KPC650: c_uint = 0x17da;
pub const KYOCERA_PRODUCT_KPC680: c_uint = 0x180a;
pub const ANYDATA_VENDOR_ID: c_uint = 0x16d5;
pub const ANYDATA_PRODUCT_ADU_620UW: c_uint = 0x6202;
pub const ANYDATA_PRODUCT_ADU_E100A: c_uint = 0x6501;
pub const ANYDATA_PRODUCT_ADU_500A: c_uint = 0x6502;
pub const AXESSTEL_VENDOR_ID: c_uint = 0x1726;
pub const AXESSTEL_PRODUCT_MV110H: c_uint = 0x1000;
pub const BANDRICH_VENDOR_ID: c_uint = 0x1A8D;
pub const BANDRICH_PRODUCT_C100_1: c_uint = 0x1002;
pub const BANDRICH_PRODUCT_C100_2: c_uint = 0x1003;
pub const BANDRICH_PRODUCT_1004: c_uint = 0x1004;
pub const BANDRICH_PRODUCT_1005: c_uint = 0x1005;
pub const BANDRICH_PRODUCT_1006: c_uint = 0x1006;
pub const BANDRICH_PRODUCT_1007: c_uint = 0x1007;
pub const BANDRICH_PRODUCT_1008: c_uint = 0x1008;
pub const BANDRICH_PRODUCT_1009: c_uint = 0x1009;
pub const BANDRICH_PRODUCT_100A: c_uint = 0x100a;
pub const BANDRICH_PRODUCT_100B: c_uint = 0x100b;
pub const BANDRICH_PRODUCT_100C: c_uint = 0x100c;
pub const BANDRICH_PRODUCT_100D: c_uint = 0x100d;
pub const BANDRICH_PRODUCT_100E: c_uint = 0x100e;
pub const BANDRICH_PRODUCT_100F: c_uint = 0x100f;
pub const BANDRICH_PRODUCT_1010: c_uint = 0x1010;
pub const BANDRICH_PRODUCT_1011: c_uint = 0x1011;
pub const BANDRICH_PRODUCT_1012: c_uint = 0x1012;
pub const QUALCOMM_VENDOR_ID: c_uint = 0x05C6;
// These Quectel products use Qualcomm's vendor ID
pub const QUECTEL_PRODUCT_UC20: c_uint = 0x9003;
pub const QUECTEL_PRODUCT_UC15: c_uint = 0x9090;
// These u-blox products use Qualcomm's vendor ID
pub const UBLOX_PRODUCT_R410M: c_uint = 0x90b2;
// These Yuga products use Qualcomm's vendor ID
pub const YUGA_PRODUCT_CLM920_NC5: c_uint = 0x9625;
pub const QUECTEL_VENDOR_ID: c_uint = 0x2c7c;
// These Quectel products use Quectel's vendor ID
pub const QUECTEL_PRODUCT_EC21: c_uint = 0x0121;
pub const QUECTEL_PRODUCT_RG650V: c_uint = 0x0122;
pub const QUECTEL_PRODUCT_EM061K_LTA: c_uint = 0x0123;
pub const QUECTEL_PRODUCT_EM061K_LMS: c_uint = 0x0124;
pub const QUECTEL_PRODUCT_EC25: c_uint = 0x0125;
pub const QUECTEL_PRODUCT_EM060K_128: c_uint = 0x0128;
pub const QUECTEL_PRODUCT_EM060K_129: c_uint = 0x0129;
pub const QUECTEL_PRODUCT_EM060K_12a: c_uint = 0x012a;
pub const QUECTEL_PRODUCT_EM060K_12b: c_uint = 0x012b;
pub const QUECTEL_PRODUCT_EM060K_12c: c_uint = 0x012c;
pub const QUECTEL_PRODUCT_EG91: c_uint = 0x0191;
pub const QUECTEL_PRODUCT_EG95: c_uint = 0x0195;
pub const QUECTEL_PRODUCT_BG96: c_uint = 0x0296;
pub const QUECTEL_PRODUCT_EP06: c_uint = 0x0306;
pub const QUECTEL_PRODUCT_EM05G: c_uint = 0x030a;
pub const QUECTEL_PRODUCT_EM060K: c_uint = 0x030b;
pub const QUECTEL_PRODUCT_EM05G_CS: c_uint = 0x030c;
pub const QUECTEL_PRODUCT_EM05GV2: c_uint = 0x030e;
pub const QUECTEL_PRODUCT_EM05CN_SG: c_uint = 0x0310;
pub const QUECTEL_PRODUCT_EM05G_SG: c_uint = 0x0311;
pub const QUECTEL_PRODUCT_EM05CN: c_uint = 0x0312;
pub const QUECTEL_PRODUCT_EM05G_GR: c_uint = 0x0313;
pub const QUECTEL_PRODUCT_EM05G_RS: c_uint = 0x0314;
pub const QUECTEL_PRODUCT_RG255C: c_uint = 0x0316;
pub const QUECTEL_PRODUCT_EM12: c_uint = 0x0512;
pub const QUECTEL_PRODUCT_RM500Q: c_uint = 0x0800;
pub const QUECTEL_PRODUCT_RM520N: c_uint = 0x0801;
pub const QUECTEL_PRODUCT_EC200U: c_uint = 0x0901;
pub const QUECTEL_PRODUCT_EG912Y: c_uint = 0x6001;
pub const QUECTEL_PRODUCT_EC200S_CN: c_uint = 0x6002;
pub const QUECTEL_PRODUCT_EC200A: c_uint = 0x6005;
pub const QUECTEL_PRODUCT_EG916Q: c_uint = 0x6007;
pub const QUECTEL_PRODUCT_EM061K_LWW: c_uint = 0x6008;
pub const QUECTEL_PRODUCT_EM061K_LCN: c_uint = 0x6009;
pub const QUECTEL_PRODUCT_EC200T: c_uint = 0x6026;
pub const QUECTEL_PRODUCT_RM500K: c_uint = 0x7001;
pub const CMOTECH_VENDOR_ID: c_uint = 0x16d8;
pub const CMOTECH_PRODUCT_6001: c_uint = 0x6001;
pub const CMOTECH_PRODUCT_CMU_300: c_uint = 0x6002;
pub const CMOTECH_PRODUCT_6003: c_uint = 0x6003;
pub const CMOTECH_PRODUCT_6004: c_uint = 0x6004;
pub const CMOTECH_PRODUCT_6005: c_uint = 0x6005;
pub const CMOTECH_PRODUCT_CGU_628A: c_uint = 0x6006;
pub const CMOTECH_PRODUCT_CHE_628S: c_uint = 0x6007;
pub const CMOTECH_PRODUCT_CMU_301: c_uint = 0x6008;
pub const CMOTECH_PRODUCT_CHU_628: c_uint = 0x6280;
pub const CMOTECH_PRODUCT_CHU_628S: c_uint = 0x6281;
pub const CMOTECH_PRODUCT_CDU_680: c_uint = 0x6803;
pub const CMOTECH_PRODUCT_CDU_685A: c_uint = 0x6804;
pub const CMOTECH_PRODUCT_CHU_720S: c_uint = 0x7001;
pub const CMOTECH_PRODUCT_7002: c_uint = 0x7002;
pub const CMOTECH_PRODUCT_CHU_629K: c_uint = 0x7003;
pub const CMOTECH_PRODUCT_7004: c_uint = 0x7004;
pub const CMOTECH_PRODUCT_7005: c_uint = 0x7005;
pub const CMOTECH_PRODUCT_CGU_629: c_uint = 0x7006;
pub const CMOTECH_PRODUCT_CHU_629S: c_uint = 0x700a;
pub const CMOTECH_PRODUCT_CHU_720I: c_uint = 0x7211;
pub const CMOTECH_PRODUCT_7212: c_uint = 0x7212;
pub const CMOTECH_PRODUCT_7213: c_uint = 0x7213;
pub const CMOTECH_PRODUCT_7251: c_uint = 0x7251;
pub const CMOTECH_PRODUCT_7252: c_uint = 0x7252;
pub const CMOTECH_PRODUCT_7253: c_uint = 0x7253;
pub const TELIT_VENDOR_ID: c_uint = 0x1bc7;
pub const TELIT_PRODUCT_UC864E: c_uint = 0x1003;
pub const TELIT_PRODUCT_UC864G: c_uint = 0x1004;
pub const TELIT_PRODUCT_CC864_DUAL: c_uint = 0x1005;
pub const TELIT_PRODUCT_CC864_SINGLE: c_uint = 0x1006;
pub const TELIT_PRODUCT_DE910_DUAL: c_uint = 0x1010;
pub const TELIT_PRODUCT_UE910_V2: c_uint = 0x1012;
pub const TELIT_PRODUCT_LE922_USBCFG1: c_uint = 0x1040;
pub const TELIT_PRODUCT_LE922_USBCFG2: c_uint = 0x1041;
pub const TELIT_PRODUCT_LE922_USBCFG0: c_uint = 0x1042;
pub const TELIT_PRODUCT_LE922_USBCFG3: c_uint = 0x1043;
pub const TELIT_PRODUCT_LE922_USBCFG5: c_uint = 0x1045;
pub const TELIT_PRODUCT_ME910: c_uint = 0x1100;
pub const TELIT_PRODUCT_ME910_DUAL_MODEM: c_uint = 0x1101;
pub const TELIT_PRODUCT_LE920: c_uint = 0x1200;
pub const TELIT_PRODUCT_LE910: c_uint = 0x1201;
pub const TELIT_PRODUCT_LE910_USBCFG4: c_uint = 0x1206;
pub const TELIT_PRODUCT_LE920A4_1207: c_uint = 0x1207;
pub const TELIT_PRODUCT_LE920A4_1208: c_uint = 0x1208;
pub const TELIT_PRODUCT_LE920A4_1211: c_uint = 0x1211;
pub const TELIT_PRODUCT_LE920A4_1212: c_uint = 0x1212;
pub const TELIT_PRODUCT_LE920A4_1213: c_uint = 0x1213;
pub const TELIT_PRODUCT_LE920A4_1214: c_uint = 0x1214;
// ZTE PRODUCTS
pub const ZTE_VENDOR_ID: c_uint = 0x19d2;
pub const ZTE_PRODUCT_MF622: c_uint = 0x0001;
pub const ZTE_PRODUCT_MF628: c_uint = 0x0015;
pub const ZTE_PRODUCT_MF626: c_uint = 0x0031;
pub const ZTE_PRODUCT_ZM8620_X: c_uint = 0x0396;
pub const ZTE_PRODUCT_ME3620_MBIM: c_uint = 0x0426;
pub const ZTE_PRODUCT_ME3620_X: c_uint = 0x1432;
pub const ZTE_PRODUCT_ME3620_L: c_uint = 0x1433;
pub const ZTE_PRODUCT_AC2726: c_uint = 0xfff1;
pub const ZTE_PRODUCT_MG880: c_uint = 0xfffd;
pub const ZTE_PRODUCT_CDMA_TECH: c_uint = 0xfffe;
pub const ZTE_PRODUCT_AC8710T: c_uint = 0xffff;
pub const ZTE_PRODUCT_MC2718: c_uint = 0xffe8;
pub const ZTE_PRODUCT_AD3812: c_uint = 0xffeb;
pub const ZTE_PRODUCT_MC2716: c_uint = 0xffed;
pub const BENQ_VENDOR_ID: c_uint = 0x04a5;
pub const BENQ_PRODUCT_H10: c_uint = 0x4068;
pub const DLINK_VENDOR_ID: c_uint = 0x1186;
pub const DLINK_PRODUCT_DWM_652: c_uint = 0x3e04;
pub const DLINK_PRODUCT_DWM_652_U5: c_uint = 0xce16;
pub const DLINK_PRODUCT_DWM_652_U5A: c_uint = 0xce1e;
pub const QISDA_VENDOR_ID: c_uint = 0x1da5;
pub const QISDA_PRODUCT_H21_4512: c_uint = 0x4512;
pub const QISDA_PRODUCT_H21_4523: c_uint = 0x4523;
pub const QISDA_PRODUCT_H20_4515: c_uint = 0x4515;
pub const QISDA_PRODUCT_H20_4518: c_uint = 0x4518;
pub const QISDA_PRODUCT_H20_4519: c_uint = 0x4519;
// TLAYTECH PRODUCTS
pub const TLAYTECH_VENDOR_ID: c_uint = 0x20B9;
pub const TLAYTECH_PRODUCT_TEU800: c_uint = 0x1682;
// TOSHIBA PRODUCTS
pub const TOSHIBA_VENDOR_ID: c_uint = 0x0930;
pub const TOSHIBA_PRODUCT_HSDPA_MINICARD: c_uint = 0x1302;
pub const TOSHIBA_PRODUCT_G450: c_uint = 0x0d45;
pub const ALINK_VENDOR_ID: c_uint = 0x1e0e;
pub const SIMCOM_PRODUCT_SIM7100E: c_uint = 0x9001 /* Yes, ALINK_VENDOR_ID */;
pub const ALINK_PRODUCT_PH300: c_uint = 0x9100;
pub const ALINK_PRODUCT_3GU: c_uint = 0x9200;
// ALCATEL PRODUCTS
pub const ALCATEL_VENDOR_ID: c_uint = 0x1bbb;
pub const ALCATEL_PRODUCT_X060S_X200: c_uint = 0x0000;
pub const ALCATEL_PRODUCT_X220_X500D: c_uint = 0x0017;
pub const ALCATEL_PRODUCT_L100V: c_uint = 0x011e;
pub const ALCATEL_PRODUCT_L800MA: c_uint = 0x0203;
pub const PIRELLI_VENDOR_ID: c_uint = 0x1266;
pub const PIRELLI_PRODUCT_C100_1: c_uint = 0x1002;
pub const PIRELLI_PRODUCT_C100_2: c_uint = 0x1003;
pub const PIRELLI_PRODUCT_1004: c_uint = 0x1004;
pub const PIRELLI_PRODUCT_1005: c_uint = 0x1005;
pub const PIRELLI_PRODUCT_1006: c_uint = 0x1006;
pub const PIRELLI_PRODUCT_1007: c_uint = 0x1007;
pub const PIRELLI_PRODUCT_1008: c_uint = 0x1008;
pub const PIRELLI_PRODUCT_1009: c_uint = 0x1009;
pub const PIRELLI_PRODUCT_100A: c_uint = 0x100a;
pub const PIRELLI_PRODUCT_100B: c_uint = 0x100b;
pub const PIRELLI_PRODUCT_100C: c_uint = 0x100c;
pub const PIRELLI_PRODUCT_100D: c_uint = 0x100d;
pub const PIRELLI_PRODUCT_100E: c_uint = 0x100e;
pub const PIRELLI_PRODUCT_100F: c_uint = 0x100f;
pub const PIRELLI_PRODUCT_1011: c_uint = 0x1011;
pub const PIRELLI_PRODUCT_1012: c_uint = 0x1012;
// Airplus products
pub const AIRPLUS_VENDOR_ID: c_uint = 0x1011;
pub const AIRPLUS_PRODUCT_MCD650: c_uint = 0x3198;
// Longcheer/Longsung vendor ID; makes whitelabel devices that
// many other vendors like 4G Systems, Alcatel, ChinaBird,
// Mobidata, etc sell under their own brand names.
//
pub const LONGCHEER_VENDOR_ID: c_uint = 0x1c9e;
// 4G Systems products
// This one was sold as the VW and Skoda "Carstick LTE"
pub const FOUR_G_SYSTEMS_PRODUCT_CARSTICK_LTE: c_uint = 0x7605;
// This is the 4G XS Stick W14 a.k.a. Mobilcom Debitel Surf-Stick
// It seems to contain a Qualcomm QSC6240/6290 chipset
pub const FOUR_G_SYSTEMS_PRODUCT_W14: c_uint = 0x9603;
pub const FOUR_G_SYSTEMS_PRODUCT_W100: c_uint = 0x9b01;
// Fujisoft products
pub const FUJISOFT_PRODUCT_FS040U: c_uint = 0x9b02;
// iBall 3.5G connect wireless modem
pub const IBALL_3_5G_CONNECT: c_uint = 0x9605;
// Zoom
pub const ZOOM_PRODUCT_4597: c_uint = 0x9607;
// SpeedUp SU9800 usb 3g modem
pub const SPEEDUP_PRODUCT_SU9800: c_uint = 0x9800;
// Haier products
pub const HAIER_VENDOR_ID: c_uint = 0x201e;
pub const HAIER_PRODUCT_CE81B: c_uint = 0x10f8;
pub const HAIER_PRODUCT_CE100: c_uint = 0x2009;
// Gemalto's Cinterion products (formerly Siemens)
pub const SIEMENS_VENDOR_ID: c_uint = 0x0681;
pub const CINTERION_VENDOR_ID: c_uint = 0x1e2d;
pub const CINTERION_PRODUCT_HC25_MDMNET: c_uint = 0x0040;
pub const CINTERION_PRODUCT_HC25_MDM: c_uint = 0x0047;
pub const CINTERION_PRODUCT_HC28_MDMNET: c_uint = 0x004A /* same for HC28J */;
pub const CINTERION_PRODUCT_HC28_MDM: c_uint = 0x004C;
pub const CINTERION_PRODUCT_EU3_E: c_uint = 0x0051;
pub const CINTERION_PRODUCT_EU3_P: c_uint = 0x0052;
pub const CINTERION_PRODUCT_PH8: c_uint = 0x0053;
pub const CINTERION_PRODUCT_AHXX: c_uint = 0x0055;
pub const CINTERION_PRODUCT_PLXX: c_uint = 0x0060;
pub const CINTERION_PRODUCT_EXS82: c_uint = 0x006c;
pub const CINTERION_PRODUCT_PH8_2RMNET: c_uint = 0x0082;
pub const CINTERION_PRODUCT_PH8_AUDIO: c_uint = 0x0083;
pub const CINTERION_PRODUCT_AHXX_2RMNET: c_uint = 0x0084;
pub const CINTERION_PRODUCT_AHXX_AUDIO: c_uint = 0x0085;
pub const CINTERION_PRODUCT_CLS8: c_uint = 0x00b0;
pub const CINTERION_PRODUCT_MV31_MBIM: c_uint = 0x00b3;
pub const CINTERION_PRODUCT_MV31_RMNET: c_uint = 0x00b7;
pub const CINTERION_PRODUCT_MV31_2_MBIM: c_uint = 0x00b8;
pub const CINTERION_PRODUCT_MV31_2_RMNET: c_uint = 0x00b9;
pub const CINTERION_PRODUCT_MV32_WA: c_uint = 0x00f1;
pub const CINTERION_PRODUCT_MV32_WB: c_uint = 0x00f2;
pub const CINTERION_PRODUCT_MV32_WA_RMNET: c_uint = 0x00f3;
pub const CINTERION_PRODUCT_MV32_WB_RMNET: c_uint = 0x00f4;
// Olivetti products
pub const OLIVETTI_VENDOR_ID: c_uint = 0x0b3c;
pub const OLIVETTI_PRODUCT_OLICARD100: c_uint = 0xc000;
pub const OLIVETTI_PRODUCT_OLICARD120: c_uint = 0xc001;
pub const OLIVETTI_PRODUCT_OLICARD140: c_uint = 0xc002;
pub const OLIVETTI_PRODUCT_OLICARD145: c_uint = 0xc003;
pub const OLIVETTI_PRODUCT_OLICARD155: c_uint = 0xc004;
pub const OLIVETTI_PRODUCT_OLICARD200: c_uint = 0xc005;
pub const OLIVETTI_PRODUCT_OLICARD160: c_uint = 0xc00a;
pub const OLIVETTI_PRODUCT_OLICARD500: c_uint = 0xc00b;
// Celot products
pub const CELOT_VENDOR_ID: c_uint = 0x211f;
pub const CELOT_PRODUCT_CT680M: c_uint = 0x6801;
// Samsung products
pub const SAMSUNG_VENDOR_ID: c_uint = 0x04e8;
pub const SAMSUNG_PRODUCT_GT_B3730: c_uint = 0x6889;
// YUGA products  www.yuga-info.com gavin.kx@qq.com
pub const YUGA_VENDOR_ID: c_uint = 0x257A;
pub const YUGA_PRODUCT_CEM600: c_uint = 0x1601;
pub const YUGA_PRODUCT_CEM610: c_uint = 0x1602;
pub const YUGA_PRODUCT_CEM500: c_uint = 0x1603;
pub const YUGA_PRODUCT_CEM510: c_uint = 0x1604;
pub const YUGA_PRODUCT_CEM800: c_uint = 0x1605;
pub const YUGA_PRODUCT_CEM900: c_uint = 0x1606;
pub const YUGA_PRODUCT_CEU818: c_uint = 0x1607;
pub const YUGA_PRODUCT_CEU816: c_uint = 0x1608;
pub const YUGA_PRODUCT_CEU828: c_uint = 0x1609;
pub const YUGA_PRODUCT_CEU826: c_uint = 0x160A;
pub const YUGA_PRODUCT_CEU518: c_uint = 0x160B;
pub const YUGA_PRODUCT_CEU516: c_uint = 0x160C;
pub const YUGA_PRODUCT_CEU528: c_uint = 0x160D;
pub const YUGA_PRODUCT_CEU526: c_uint = 0x160F;
pub const YUGA_PRODUCT_CEU881: c_uint = 0x161F;
pub const YUGA_PRODUCT_CEU882: c_uint = 0x162F;
pub const YUGA_PRODUCT_CWM600: c_uint = 0x2601;
pub const YUGA_PRODUCT_CWM610: c_uint = 0x2602;
pub const YUGA_PRODUCT_CWM500: c_uint = 0x2603;
pub const YUGA_PRODUCT_CWM510: c_uint = 0x2604;
pub const YUGA_PRODUCT_CWM800: c_uint = 0x2605;
pub const YUGA_PRODUCT_CWM900: c_uint = 0x2606;
pub const YUGA_PRODUCT_CWU718: c_uint = 0x2607;
pub const YUGA_PRODUCT_CWU716: c_uint = 0x2608;
pub const YUGA_PRODUCT_CWU728: c_uint = 0x2609;
pub const YUGA_PRODUCT_CWU726: c_uint = 0x260A;
pub const YUGA_PRODUCT_CWU518: c_uint = 0x260B;
pub const YUGA_PRODUCT_CWU516: c_uint = 0x260C;
pub const YUGA_PRODUCT_CWU528: c_uint = 0x260D;
pub const YUGA_PRODUCT_CWU581: c_uint = 0x260E;
pub const YUGA_PRODUCT_CWU526: c_uint = 0x260F;
pub const YUGA_PRODUCT_CWU582: c_uint = 0x261F;
pub const YUGA_PRODUCT_CWU583: c_uint = 0x262F;
pub const YUGA_PRODUCT_CLM600: c_uint = 0x3601;
pub const YUGA_PRODUCT_CLM610: c_uint = 0x3602;
pub const YUGA_PRODUCT_CLM500: c_uint = 0x3603;
pub const YUGA_PRODUCT_CLM510: c_uint = 0x3604;
pub const YUGA_PRODUCT_CLM800: c_uint = 0x3605;
pub const YUGA_PRODUCT_CLM900: c_uint = 0x3606;
pub const YUGA_PRODUCT_CLU718: c_uint = 0x3607;
pub const YUGA_PRODUCT_CLU716: c_uint = 0x3608;
pub const YUGA_PRODUCT_CLU728: c_uint = 0x3609;
pub const YUGA_PRODUCT_CLU726: c_uint = 0x360A;
pub const YUGA_PRODUCT_CLU518: c_uint = 0x360B;
pub const YUGA_PRODUCT_CLU516: c_uint = 0x360C;
pub const YUGA_PRODUCT_CLU528: c_uint = 0x360D;
pub const YUGA_PRODUCT_CLU526: c_uint = 0x360F;
// Viettel products
pub const VIETTEL_VENDOR_ID: c_uint = 0x2262;
pub const VIETTEL_PRODUCT_VT1000: c_uint = 0x0002;
// ZD Incorporated
pub const ZD_VENDOR_ID: c_uint = 0x0685;
pub const ZD_PRODUCT_7000: c_uint = 0x7000;
// LG products
pub const LG_VENDOR_ID: c_uint = 0x1004;
pub const LG_PRODUCT_L02C: c_uint = 0x618f;
// MediaTek products
pub const MEDIATEK_VENDOR_ID: c_uint = 0x0e8d;
pub const MEDIATEK_PRODUCT_DC_1COM: c_uint = 0x00a0;
pub const MEDIATEK_PRODUCT_DC_4COM: c_uint = 0x00a5;
pub const MEDIATEK_PRODUCT_DC_4COM2: c_uint = 0x00a7;
pub const MEDIATEK_PRODUCT_DC_5COM: c_uint = 0x00a4;
pub const MEDIATEK_PRODUCT_7208_1COM: c_uint = 0x7101;
pub const MEDIATEK_PRODUCT_7208_2COM: c_uint = 0x7102;
pub const MEDIATEK_PRODUCT_7103_2COM: c_uint = 0x7103;
pub const MEDIATEK_PRODUCT_7106_2COM: c_uint = 0x7106;
pub const MEDIATEK_PRODUCT_FP_1COM: c_uint = 0x0003;
pub const MEDIATEK_PRODUCT_FP_2COM: c_uint = 0x0023;
pub const MEDIATEK_PRODUCT_FPDC_1COM: c_uint = 0x0043;
pub const MEDIATEK_PRODUCT_FPDC_2COM: c_uint = 0x0033;
// Cellient products
pub const CELLIENT_VENDOR_ID: c_uint = 0x2692;
pub const CELLIENT_PRODUCT_MEN200: c_uint = 0x9005;
pub const CELLIENT_PRODUCT_MPL200: c_uint = 0x9025;
// Hyundai Petatel Inc. products
pub const PETATEL_VENDOR_ID: c_uint = 0x1ff4;
pub const PETATEL_PRODUCT_NP10T_600A: c_uint = 0x600a;
pub const PETATEL_PRODUCT_NP10T_600E: c_uint = 0x600e;
// TP-LINK Incorporated products
pub const TPLINK_VENDOR_ID: c_uint = 0x2357;
pub const TPLINK_PRODUCT_LTE: c_uint = 0x000D;
pub const TPLINK_PRODUCT_MA180: c_uint = 0x0201;
// Changhong products
pub const CHANGHONG_VENDOR_ID: c_uint = 0x2077;
pub const CHANGHONG_PRODUCT_CH690: c_uint = 0x7001;
// Inovia
pub const INOVIA_VENDOR_ID: c_uint = 0x20a6;
pub const INOVIA_SEW858: c_uint = 0x1105;
// VIA Telecom
pub const VIATELECOM_VENDOR_ID: c_uint = 0x15eb;
pub const VIATELECOM_PRODUCT_CDS7: c_uint = 0x0001;
// WeTelecom products
pub const WETELECOM_VENDOR_ID: c_uint = 0x22de;
pub const WETELECOM_PRODUCT_WMD200: c_uint = 0x6801;
pub const WETELECOM_PRODUCT_6802: c_uint = 0x6802;
pub const WETELECOM_PRODUCT_WMD300: c_uint = 0x6803;
// OPPO products
pub const OPPO_VENDOR_ID: c_uint = 0x22d9;
pub const OPPO_PRODUCT_R11: c_uint = 0x276c;
// Sierra Wireless products
pub const SIERRA_VENDOR_ID: c_uint = 0x1199;
pub const SIERRA_PRODUCT_EM9191: c_uint = 0x90d3;
pub const SIERRA_PRODUCT_EM9291: c_uint = 0x90e3;
// UNISOC (Spreadtrum) products
pub const UNISOC_VENDOR_ID: c_uint = 0x1782;
// TOZED LT70-C based on UNISOC SL8563 uses UNISOC's vendor ID
pub const TOZED_PRODUCT_LT70C: c_uint = 0x4055;
pub const UNISOC_PRODUCT_UIS7720: c_uint = 0x4064;
// Luat Air72*U series based on UNISOC UIS8910 uses UNISOC's vendor ID
pub const LUAT_PRODUCT_AIR720U: c_uint = 0x4e00;
// Device flags
// Highest interface number which can be used with NCTRL() and RSVD()
pub const FLAG_IFNUM_MAX: c_int = 7;
// Interface does not support modem-control requests

// Interface is reserved

// Interface must have two endpoints

// Device needs ZLP

    static const struct usb_device_id option_ids[] = {
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_COLT) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_RICOLA) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_RICOLA_LIGHT) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_RICOLA_QUAD) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_RICOLA_QUAD_LIGHT) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_RICOLA_NDIS) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_RICOLA_NDIS_LIGHT) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_RICOLA_NDIS_QUAD) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_RICOLA_NDIS_QUAD_LIGHT) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_COBRA) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_COBRA_BUS) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_VIPER) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_VIPER_BUS) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_GT_MAX_READY) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_FUJI_MODEM_LIGHT) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_FUJI_MODEM_GT) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_FUJI_MODEM_EX) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_KOI_MODEM) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_SCORPION_MODEM) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_ETNA_MODEM) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_ETNA_MODEM_LITE) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_ETNA_MODEM_GT) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_ETNA_MODEM_EX) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_ETNA_KOI_MODEM) },
    { USB_DEVICE(OPTION_VENDOR_ID, OPTION_PRODUCT_GTM380_MODEM) },
    { USB_DEVICE(QUANTA_VENDOR_ID, QUANTA_PRODUCT_Q101) },
    { USB_DEVICE(QUANTA_VENDOR_ID, QUANTA_PRODUCT_Q111) },
    { USB_DEVICE(QUANTA_VENDOR_ID, QUANTA_PRODUCT_GLX) },
    { USB_DEVICE(QUANTA_VENDOR_ID, QUANTA_PRODUCT_GKE) },
    { USB_DEVICE(QUANTA_VENDOR_ID, QUANTA_PRODUCT_GLE) },
    { USB_DEVICE(QUANTA_VENDOR_ID, 0xea42),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0x1c05, USB_CLASS_COMM, 0x02, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0x1c1f, USB_CLASS_COMM, 0x02, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0x1c23, USB_CLASS_COMM, 0x02, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, HUAWEI_PRODUCT_E173, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, HUAWEI_PRODUCT_E173S6, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, HUAWEI_PRODUCT_E1750, 0xff, 0xff, 0xff),
    .driver_info = RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0x1441, USB_CLASS_COMM, 0x02, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0x1442, USB_CLASS_COMM, 0x02, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, HUAWEI_PRODUCT_K4505, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) | RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, HUAWEI_PRODUCT_K3765, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) | RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0x14ac, 0xff, 0xff, 0xff),	/* Huawei E1820 */
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, HUAWEI_PRODUCT_K4605, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) | RSVD(2) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0xff, 0xff) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x01) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x02) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x03) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x04) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x05) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x06) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x0A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x0B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x0D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x0E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x0F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x10) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x12) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x13) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x14) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x15) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x17) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x18) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x19) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x1A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x1B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x1C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x31) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x32) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x33) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x34) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x35) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x36) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x3A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x3B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x3D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x3E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x3F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x48) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x49) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x4A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x4B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x4C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x61) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x62) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x63) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x64) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x65) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x66) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x6A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x6B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x6D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x6E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x6F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x72) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x73) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x74) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x75) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x78) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x79) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x7A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x7B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x01, 0x7C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x01) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x02) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x03) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x04) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x05) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x06) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x0A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x0B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x0D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x0E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x0F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x10) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x12) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x13) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x14) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x15) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x17) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x18) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x19) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x1A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x1B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x1C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x31) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x32) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x33) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x34) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x35) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x36) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x3A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x3B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x3D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x3E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x3F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x48) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x49) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x4A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x4B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x4C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x61) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x62) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x63) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x64) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x65) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x66) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x6A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x6B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x6D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x6E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x6F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x72) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x73) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x74) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x75) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x78) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x79) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x7A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x7B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x02, 0x7C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x01) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x02) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x03) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x04) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x05) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x06) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x0A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x0B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x0D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x0E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x0F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x10) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x12) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x13) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x14) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x15) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x17) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x18) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x19) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x1A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x1B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x1C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x31) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x32) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x33) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x34) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x35) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x36) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x3A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x3B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x3D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x3E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x3F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x48) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x49) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x4A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x4B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x4C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x61) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x62) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x63) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x64) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x65) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x66) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x6A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x6B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x6D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x6E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x6F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x72) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x73) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x74) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x75) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x78) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x79) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x7A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x7B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x03, 0x7C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x01) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x02) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x03) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x04) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x05) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x06) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x0A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x0B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x0D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x0E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x0F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x10) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x12) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x13) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x14) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x15) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x17) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x18) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x19) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x1A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x1B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x1C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x31) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x32) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x33) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x34) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x35) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x36) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x3A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x3B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x3D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x3E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x3F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x48) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x49) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x4A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x4B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x4C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x61) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x62) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x63) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x64) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x65) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x66) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x6A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x6B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x6D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x6E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x6F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x72) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x73) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x74) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x75) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x78) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x79) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x7A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x7B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x04, 0x7C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x01) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x02) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x03) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x04) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x05) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x06) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x0A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x0B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x0D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x0E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x0F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x10) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x12) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x13) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x14) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x15) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x17) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x18) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x19) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x1A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x1B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x1C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x31) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x32) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x33) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x34) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x35) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x36) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x3A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x3B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x3D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x3E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x3F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x48) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x49) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x4A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x4B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x4C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x61) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x62) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x63) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x64) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x65) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x66) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x6A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x6B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x6D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x6E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x6F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x72) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x73) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x74) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x75) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x78) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x79) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x7A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x7B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x05, 0x7C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x01) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x02) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x03) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x04) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x05) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x06) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x0A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x0B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x0D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x0E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x0F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x10) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x12) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x13) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x14) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x15) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x17) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x18) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x19) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x1A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x1B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x1C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x31) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x32) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x33) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x34) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x35) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x36) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x3A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x3B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x3D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x3E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x3F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x48) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x49) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x4A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x4B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x4C) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x61) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x62) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x63) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x64) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x65) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x66) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x6A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x6B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x6D) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x6E) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x6F) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x72) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x73) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x74) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x75) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x78) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x79) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x7A) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x7B) },
    { USB_VENDOR_AND_INTERFACE_INFO(HUAWEI_VENDOR_ID, 0xff, 0x06, 0x7C) },
// Motorola devices
    { USB_DEVICE_AND_INTERFACE_INFO(0x22b8, 0x2a70, 0xff, 0xff, 0xff) },	/* mdm6600 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x22b8, 0x2e0a, 0xff, 0xff, 0xff) },	/* mdm9600 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x22b8, 0x4281, 0x0a, 0x00, 0xfc) },	/* mdm ram dl */
    { USB_DEVICE_AND_INTERFACE_INFO(0x22b8, 0x900e, 0xff, 0xff, 0xff) },	/* mdm qc dl */
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_V640) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_V620) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_V740) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_V720) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_U730) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_U740) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_U870) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_XU870) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_X950D) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_EV620) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_ES720) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_E725) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_ES620) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_EU730) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_EU740) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_EU870D) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_MC950D) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_MC727) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_OVMC760) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_MC780) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_EVDO_FULLSPEED) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_FULLSPEED) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_EVDO_EMBEDDED_FULLSPEED) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_EMBEDDED_FULLSPEED) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_EVDO_HIGHSPEED) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED3) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED4) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED5) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED6) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_HIGHSPEED7) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_MC996D) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_MF3470) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_MC547) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_EVDO_EMBEDDED_HIGHSPEED) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_HSPA_EMBEDDED_HIGHSPEED) },
    { USB_DEVICE(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_G2) },
// Novatel Ovation MC551 a.k.a. Verizon USB551L
    { USB_DEVICE_AND_INTERFACE_INFO(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_MC551, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_E362, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_E371, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(NOVATELWIRELESS_VENDOR_ID, NOVATELWIRELESS_PRODUCT_U620L, 0xff, 0x00, 0x00) },
    { USB_DEVICE(AMOI_VENDOR_ID, AMOI_PRODUCT_H01) },
    { USB_DEVICE(AMOI_VENDOR_ID, AMOI_PRODUCT_H01A) },
    { USB_DEVICE(AMOI_VENDOR_ID, AMOI_PRODUCT_H02) },
    { USB_DEVICE(AMOI_VENDOR_ID, AMOI_PRODUCT_SKYPEPHONE_S2) },
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5700_MINICARD) },		/* Dell Wireless 5700 Mobile Broadband CDMA/EVDO Mini-Card == Novatel Expedite EV620 CDMA/EV-DO */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5500_MINICARD) },		/* Dell Wireless 5500 Mobile Broadband HSDPA Mini-Card == Novatel Expedite EU740 HSDPA/3G */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5505_MINICARD) },		/* Dell Wireless 5505 Mobile Broadband HSDPA Mini-Card == Novatel Expedite EU740 HSDPA/3G */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5700_EXPRESSCARD) },		/* Dell Wireless 5700 Mobile Broadband CDMA/EVDO ExpressCard == Novatel Merlin XV620 CDMA/EV-DO */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5510_EXPRESSCARD) },		/* Dell Wireless 5510 Mobile Broadband HSDPA ExpressCard == Novatel Merlin XU870 HSDPA/3G */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5700_MINICARD_SPRINT) },	/* Dell Wireless 5700 Mobile Broadband CDMA/EVDO Mini-Card == Novatel Expedite E720 CDMA/EV-DO */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5700_MINICARD_TELUS) },	/* Dell Wireless 5700 Mobile Broadband CDMA/EVDO Mini-Card == Novatel Expedite ET620 CDMA/EV-DO */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5720_MINICARD_VZW) }, 	/* Dell Wireless 5720 == Novatel EV620 CDMA/EV-DO */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5720_MINICARD_SPRINT) }, 	/* Dell Wireless 5720 == Novatel EV620 CDMA/EV-DO */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5720_MINICARD_TELUS) }, 	/* Dell Wireless 5720 == Novatel EV620 CDMA/EV-DO */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5520_MINICARD_CINGULAR) },	/* Dell Wireless HSDPA 5520 == Novatel Expedite EU860D */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5520_MINICARD_GENERIC_L) },	/* Dell Wireless HSDPA 5520 */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5520_MINICARD_GENERIC_I) },	/* Dell Wireless 5520 Voda I Mobile Broadband (3G HSDPA) Minicard */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5730_MINICARD_SPRINT) },	/* Dell Wireless 5730 Mobile Broadband EVDO/HSPA Mini-Card */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5730_MINICARD_TELUS) },	/* Dell Wireless 5730 Mobile Broadband EVDO/HSPA Mini-Card */
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5730_MINICARD_VZW) }, 	/* Dell Wireless 5730 Mobile Broadband EVDO/HSPA Mini-Card */
    { USB_DEVICE_AND_INTERFACE_INFO(DELL_VENDOR_ID, DELL_PRODUCT_5800_MINICARD_VZW, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(DELL_VENDOR_ID, DELL_PRODUCT_5800_V2_MINICARD_VZW, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(DELL_VENDOR_ID, DELL_PRODUCT_5804_MINICARD_ATT, 0xff, 0xff, 0xff) },
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5821E),
    .driver_info = RSVD(0) | RSVD(1) | RSVD(6) },
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5821E_ESIM),
    .driver_info = RSVD(0) | RSVD(1) | RSVD(6) },
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5829E),
    .driver_info = RSVD(0) | RSVD(6) },
    { USB_DEVICE(DELL_VENDOR_ID, DELL_PRODUCT_5829E_ESIM),
    .driver_info = RSVD(0) | RSVD(6) },
    { USB_DEVICE_INTERFACE_CLASS(DELL_VENDOR_ID, DELL_PRODUCT_5826E_ESIM, 0xff),
    .driver_info = RSVD(1) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(DELL_VENDOR_ID, DELL_PRODUCT_FM101R, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(DELL_VENDOR_ID, DELL_PRODUCT_FM101R_ESIM, 0xff) },
    { USB_DEVICE(ANYDATA_VENDOR_ID, ANYDATA_PRODUCT_ADU_E100A) },	/* ADU-E100, ADU-310 */
    { USB_DEVICE(ANYDATA_VENDOR_ID, ANYDATA_PRODUCT_ADU_500A) },
    { USB_DEVICE(ANYDATA_VENDOR_ID, ANYDATA_PRODUCT_ADU_620UW) },
    { USB_DEVICE(AXESSTEL_VENDOR_ID, AXESSTEL_PRODUCT_MV110H) },
    { USB_DEVICE(YISO_VENDOR_ID, YISO_PRODUCT_U893) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_C100_1, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_C100_2, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1004, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1005, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1006, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1007, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1008, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1009, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_100A, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_100B, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_100C, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_100D, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_100E, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_100F, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1010, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1011, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(BANDRICH_VENDOR_ID, BANDRICH_PRODUCT_1012, 0xff) },
    { USB_DEVICE(KYOCERA_VENDOR_ID, KYOCERA_PRODUCT_KPC650) },
    { USB_DEVICE(KYOCERA_VENDOR_ID, KYOCERA_PRODUCT_KPC680) },
    { USB_DEVICE(QUALCOMM_VENDOR_ID, 0x6000)}, /* ZTE AC8700 */
    { USB_DEVICE_AND_INTERFACE_INFO(QUALCOMM_VENDOR_ID, 0x6001, 0xff, 0xff, 0xff), /* 4G LTE usb-modem U901 */
    .driver_info = RSVD(3) },
    { USB_DEVICE(QUALCOMM_VENDOR_ID, 0x6613)}, /* Onda H600/ZTE MF330 */
    { USB_DEVICE(QUALCOMM_VENDOR_ID, 0x0023)}, /* ONYX 3G device */
    { USB_DEVICE(QUALCOMM_VENDOR_ID, 0x9000), /* SIMCom SIM5218 */
    .driver_info = NCTRL(0) | NCTRL(1) | NCTRL(2) | NCTRL(3) | RSVD(4) },
// Quectel products using Qualcomm vendor ID
    { USB_DEVICE(QUALCOMM_VENDOR_ID, QUECTEL_PRODUCT_UC15)},
    { USB_DEVICE(QUALCOMM_VENDOR_ID, QUECTEL_PRODUCT_UC20),
    .driver_info = RSVD(4) },
// Yuga products use Qualcomm vendor ID
    { USB_DEVICE(QUALCOMM_VENDOR_ID, YUGA_PRODUCT_CLM920_NC5),
    .driver_info = RSVD(1) | RSVD(4) },
// u-blox products using Qualcomm vendor ID
    { USB_DEVICE(QUALCOMM_VENDOR_ID, UBLOX_PRODUCT_R410M),
    .driver_info = RSVD(1) | RSVD(3) },
    { USB_DEVICE(QUALCOMM_VENDOR_ID, 0x908b),	/* u-blox LARA-R6 00B */
    .driver_info = RSVD(4) },
    { USB_DEVICE(QUALCOMM_VENDOR_ID, 0x90fa),
    .driver_info = RSVD(3) },
// u-blox products
    { USB_DEVICE(UBLOX_VENDOR_ID, 0x1311) },	/* u-blox LARA-R6 01B */
    { USB_DEVICE(UBLOX_VENDOR_ID, 0x1312),		/* u-blox LARA-R6 01B (RMNET) */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(UBLOX_VENDOR_ID, 0x1313, 0xff) },	/* u-blox LARA-R6 01B (ECM) */
    { USB_DEVICE(UBLOX_VENDOR_ID, 0x1341) },	/* u-blox LARA-L6 */
    { USB_DEVICE(UBLOX_VENDOR_ID, 0x1342),		/* u-blox LARA-L6 (RMNET) */
    .driver_info = RSVD(4) },
    { USB_DEVICE(UBLOX_VENDOR_ID, 0x1343),		/* u-blox LARA-L6 (ECM) */
    .driver_info = RSVD(4) },
// Quectel products using Quectel vendor ID
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EC21, 0xff, 0xff, 0xff),
    .driver_info = NUMEP2 },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EC21, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EC25, 0xff, 0xff, 0xff),
    .driver_info = NUMEP2 },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EC25, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EG91, 0xff, 0xff, 0xff),
    .driver_info = NUMEP2 },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EG91, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EG95, 0xff, 0xff, 0xff),
    .driver_info = NUMEP2 },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EG95, 0xff, 0, 0) },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, 0x0203, 0xff), /* BG95-M3 */
    .driver_info = ZLP },
    { USB_DEVICE(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_BG96),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EP06, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) | RSVD(2) | RSVD(3) | RSVD(4) | NUMEP2 },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EP06, 0xff, 0, 0) },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM05CN, 0xff),
    .driver_info = RSVD(6) | ZLP },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM05CN_SG, 0xff),
    .driver_info = RSVD(6) | ZLP },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM05G, 0xff),
    .driver_info = RSVD(6) | ZLP },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM05GV2, 0xff),
    .driver_info = RSVD(4) | ZLP },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM05G_CS, 0xff),
    .driver_info = RSVD(6) | ZLP },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM05G_GR, 0xff),
    .driver_info = RSVD(6) | ZLP },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM05G_RS, 0xff),
    .driver_info = RSVD(6) | ZLP },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM05G_SG, 0xff),
    .driver_info = RSVD(6) | ZLP },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_128, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_128, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_128, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_129, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_129, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_129, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12a, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12a, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12a, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12b, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12b, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12b, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12c, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12c, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM060K_12c, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LCN, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LCN, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LCN, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LMS, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LMS, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LMS, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LTA, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LTA, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LTA, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LWW, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LWW, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM061K_LWW, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM12, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) | RSVD(2) | RSVD(3) | RSVD(4) | NUMEP2 },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EM12, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, 0x0620, 0xff, 0xff, 0x30) },	/* EM160R-GL */
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, 0x0620, 0xff, 0, 0) },
    { USB_DEVICE_INTERFACE_CLASS(QUECTEL_VENDOR_ID, 0x0700, 0xff), /* BG95 */
    .driver_info = RSVD(3) | ZLP },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RM500Q, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RM500Q, 0xff, 0, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RM500Q, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RM500Q, 0xff, 0xff, 0x10),
    .driver_info = ZLP },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RM520N, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RM520N, 0xff, 0, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RM520N, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, 0x0900, 0xff, 0, 0), /* RM500U-CN */
    .driver_info = ZLP },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EC200A, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EC200U, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EC200S_CN, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EC200T, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EG912Y, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_EG916Q, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RM500K, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RG650V, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RG650V, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RG255C, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RG255C, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(QUECTEL_VENDOR_ID, QUECTEL_PRODUCT_RG255C, 0xff, 0xff, 0x40) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_6001) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CMU_300) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_6003),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_6004) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_6005) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CGU_628A) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CHE_628S),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CMU_301),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CHU_628),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CHU_628S) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CDU_680) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CDU_685A) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CHU_720S),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_7002),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CHU_629K),
    .driver_info = RSVD(4) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_7004),
    .driver_info = RSVD(3) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_7005) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CGU_629),
    .driver_info = RSVD(5) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CHU_629S),
    .driver_info = RSVD(4) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CHU_720I),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_7212),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_7213),
    .driver_info = RSVD(0) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_7251),
    .driver_info = RSVD(1) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_7252),
    .driver_info = RSVD(1) },
    { USB_DEVICE(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_7253),
    .driver_info = RSVD(1) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_UC864E) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_UC864G) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_CC864_DUAL) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_CC864_SINGLE) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_DE910_DUAL) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_UE910_V2) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0990, 0xff, 0xff, 0x30),	/* Telit FE990D50 (RNDIS) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0990, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0990, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0991, 0xff, 0xff, 0x30),	/* Telit FE990D50 (rmnet) */
    .driver_info = NCTRL(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0991, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0991, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0992, 0xff, 0xff, 0x30),	/* Telit FE990D50 (MBIM) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0992, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0992, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0993, 0xff, 0xff, 0x30),	/* Telit FE990D50 (ECM) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0993, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x0993, 0xff, 0xff, 0x60) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1031, 0xff),	/* Telit LE910C1-EUX */
    .driver_info = NCTRL(0) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1033, 0xff),	/* Telit LE910C1-EUX (ECM) */
    .driver_info = NCTRL(0) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1034, 0xff),	/* Telit LE910C4-WWX (rmnet) */
    .driver_info = RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1035, 0xff) }, /* Telit LE910C4-WWX (ECM) */
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1036, 0xff) },  /* Telit LE910C4-WWX */
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1037, 0xff),	/* Telit LE910C4-WWX (rmnet) */
    .driver_info = NCTRL(0) | NCTRL(1) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1038, 0xff),	/* Telit LE910C4-WWX (rmnet) */
    .driver_info = NCTRL(0) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x103b, 0xff),	/* Telit LE910C4-WWX */
    .driver_info = NCTRL(0) | NCTRL(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x103c, 0xff),	/* Telit LE910C4-WWX */
    .driver_info = NCTRL(0) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE922_USBCFG0),
    .driver_info = RSVD(0) | RSVD(1) | NCTRL(2) | RSVD(3) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE922_USBCFG1),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE922_USBCFG2),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) | RSVD(3) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE922_USBCFG3),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, TELIT_PRODUCT_LE922_USBCFG5, 0xff),
    .driver_info = RSVD(0) | RSVD(1) | NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1050, 0xff),	/* Telit FN980 (rmnet) */
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1051, 0xff),	/* Telit FN980 (MBIM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1052, 0xff),	/* Telit FN980 (RNDIS) */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1053, 0xff),	/* Telit FN980 (ECM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1054, 0xff),	/* Telit FT980-KS */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1055, 0xff),	/* Telit FN980 (PCIe) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1056, 0xff),	/* Telit FD980 */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1057, 0xff),	/* Telit FN980 */
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1058, 0xff),	/* Telit FN980 (PCIe) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1060, 0xff),	/* Telit LN920 (rmnet) */
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1061, 0xff),	/* Telit LN920 (MBIM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1062, 0xff),	/* Telit LN920 (RNDIS) */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1063, 0xff),	/* Telit LN920 (ECM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1070, 0xff),	/* Telit FN990A (rmnet) */
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1071, 0xff),	/* Telit FN990A (MBIM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1072, 0xff),	/* Telit FN990A (RNDIS) */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1073, 0xff),	/* Telit FN990A (ECM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1074, 0xff),	/* Telit FN990A (MBIM) */
    .driver_info = NCTRL(5) | RSVD(6) | RSVD(7) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1075, 0xff),	/* Telit FN990A (PCIe) */
    .driver_info = RSVD(0) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1077, 0xff),	/* Telit FN990A (rmnet + audio) */
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1078, 0xff),	/* Telit FN990A (MBIM + audio) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1079, 0xff),	/* Telit FN990A (RNDIS + audio) */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1080, 0xff),	/* Telit FE990A (rmnet) */
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1081, 0xff),	/* Telit FE990A (MBIM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1082, 0xff),	/* Telit FE990A (RNDIS) */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1083, 0xff),	/* Telit FE990A (ECM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a0, 0xff),	/* Telit FN20C04 (rmnet) */
    .driver_info = RSVD(0) | NCTRL(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a1, 0xff),	/* Telit FN20C04 (RNDIS) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a2, 0xff),	/* Telit FN920C04 (MBIM) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a3, 0xff),	/* Telit FN920C04 (ECM) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a4, 0xff),	/* Telit FN20C04 (rmnet) */
    .driver_info = RSVD(0) | NCTRL(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a6, 0xff),	/* Telit FN920C04 (RNDIS) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a7, 0xff),	/* Telit FN920C04 (MBIM) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a8, 0xff),	/* Telit FN920C04 (ECM) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10a9, 0xff),	/* Telit FN20C04 (rmnet) */
    .driver_info = RSVD(0) | NCTRL(2) | RSVD(3) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10aa, 0xff),	/* Telit FN920C04 (MBIM) */
    .driver_info = NCTRL(3) | RSVD(4) | RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10ab, 0xff),	/* Telit FN920C04 (RNDIS) */
    .driver_info = NCTRL(3) | RSVD(4) | RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b0, 0xff, 0xff, 0x30),	/* Telit FE990B (rmnet) */
    .driver_info = NCTRL(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b0, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b0, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b1, 0xff, 0xff, 0x30),	/* Telit FE990B (MBIM) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b1, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b1, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b2, 0xff, 0xff, 0x30),	/* Telit FE990B (RNDIS) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b2, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b2, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b3, 0xff, 0xff, 0x30),	/* Telit FE990B (ECM) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b3, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10b3, 0xff, 0xff, 0x60) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c0, 0xff),	/* Telit FE910C04 (rmnet) */
    .driver_info = RSVD(0) | NCTRL(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c1, 0xff),	/* Telit FE910C04 (RNDIS) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c2, 0xff),	/* Telit FE910C04 (MBIM) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c3, 0xff),	/* Telit FE910C04 (ECM) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c4, 0xff),	/* Telit FE910C04 (rmnet) */
    .driver_info = RSVD(0) | NCTRL(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c5, 0xff),	/* Telit FE910C04 (RNDIS) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c6, 0xff),	/* Telit FE910C04 (MBIM) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10c7, 0xff, 0xff, 0x30),	/* Telit FE910C04 (ECM) */
    .driver_info = NCTRL(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10c7, 0xff, 0xff, 0x40) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c8, 0xff),	/* Telit FE910C04 (rmnet) */
    .driver_info = RSVD(0) | NCTRL(2) | RSVD(3) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10c9, 0xff),	/* Telit FE910C04 (MBIM) */
    .driver_info = NCTRL(3) | RSVD(4) | RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x10cb, 0xff),	/* Telit FE910C04 (RNDIS) */
    .driver_info = NCTRL(3) | RSVD(4) | RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d0, 0xff, 0xff, 0x30),	/* Telit FN990B (rmnet) */
    .driver_info = NCTRL(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d0, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d0, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d1, 0xff, 0xff, 0x30),	/* Telit FN990B (MBIM) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d1, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d1, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d2, 0xff, 0xff, 0x30),	/* Telit FN990B (RNDIS) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d2, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d2, 0xff, 0xff, 0x60) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d3, 0xff, 0xff, 0x30),	/* Telit FN990B (ECM) */
    .driver_info = NCTRL(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d3, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x10d3, 0xff, 0xff, 0x60) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_ME910),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(3) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_ME910_DUAL_MODEM),
    .driver_info = NCTRL(0) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1102, 0xff),	/* Telit ME910 (ECM) */
    .driver_info = NCTRL(0) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x110a, 0xff),	/* Telit ME910G1 */
    .driver_info = NCTRL(0) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x110b, 0xff),	/* Telit ME910G1 (ECM) */
    .driver_info = NCTRL(0) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE910),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1203, 0xff),	/* Telit LE910Cx (RNDIS) */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1204, 0xff),	/* Telit LE910Cx (MBIM) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE910_USBCFG4),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) | RSVD(3) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE920),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(5) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE920A4_1207) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE920A4_1208),
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE920A4_1211),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) | RSVD(3) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE920A4_1212),
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, TELIT_PRODUCT_LE920A4_1213, 0xff) },
    { USB_DEVICE(TELIT_VENDOR_ID, TELIT_PRODUCT_LE920A4_1214),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) | RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1230, 0xff),	/* Telit LE910Cx (rmnet) */
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1231, 0xff),	/* Telit LE910Cx (RNDIS) */
    .driver_info = NCTRL(2) | RSVD(3) },
    { USB_DEVICE_AND_INTERFACE_INFO(TELIT_VENDOR_ID, 0x1250, 0xff, 0x00, 0x00) },	/* Telit LE910Cx (rmnet) */
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1251, 0xff) },	/* Telit LE910Cx (RNDIS) */
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1252, 0xff) },	/* Telit LE910Cx (MBIM) */
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1253, 0xff) },	/* Telit LE910Cx (ECM) */
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1254, 0xff) },	/* Telit LE910Cx */
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1255, 0xff) },	/* Telit LE910Cx */
    { USB_DEVICE(TELIT_VENDOR_ID, 0x1260),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE(TELIT_VENDOR_ID, 0x1261),
    .driver_info = NCTRL(0) | RSVD(1) | RSVD(2) },
    { USB_DEVICE(TELIT_VENDOR_ID, 0x1900),				/* Telit LN940 (QMI) */
    .driver_info = NCTRL(0) | RSVD(1) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x1901, 0xff),	/* Telit LN940 (MBIM) */
    .driver_info = NCTRL(0) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x3000, 0xff),	/* Telit FN912 */
    .driver_info = RSVD(0) | NCTRL(3) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x3001, 0xff),	/* Telit FN912 */
    .driver_info = RSVD(0) | NCTRL(2) | RSVD(3) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x7010, 0xff),	/* Telit LE910-S1 (RNDIS) */
    .driver_info = NCTRL(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x7011, 0xff),	/* Telit LE910-S1 (ECM) */
    .driver_info = NCTRL(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x701a, 0xff),	/* Telit LE910R1 (RNDIS) */
    .driver_info = NCTRL(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x701b, 0xff),	/* Telit LE910R1 (ECM) */
    .driver_info = NCTRL(2) },
    { USB_DEVICE_INTERFACE_CLASS(TELIT_VENDOR_ID, 0x9000, 0xff),	/* Telit generic core-dump device */
    .driver_info = NCTRL(0) },
    { USB_DEVICE(TELIT_VENDOR_ID, 0x9010),				/* Telit SBL FN980 flashing device */
    .driver_info = NCTRL(0) | ZLP },
    { USB_DEVICE(TELIT_VENDOR_ID, 0x9200),				/* Telit LE910S1 flashing device */
    .driver_info = NCTRL(0) | ZLP },
    { USB_DEVICE(TELIT_VENDOR_ID, 0x9201),				/* Telit LE910R1 flashing device */
    .driver_info = NCTRL(0) | ZLP },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_MF622, 0xff, 0xff, 0xff) }, /* ZTE WCDMA products */
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0002, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0003, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0004, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0005, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0006, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0008, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0009, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x000a, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x000b, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x000c, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x000d, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x000e, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x000f, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0010, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0011, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0012, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0013, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_MF628, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0016, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0017, 0xff, 0xff, 0xff),
    .driver_info = RSVD(3) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0018, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0019, 0xff, 0xff, 0xff),
    .driver_info = RSVD(3) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0020, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0021, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0022, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0023, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0024, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0025, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0028, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0029, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0030, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_MF626, 0xff, 0xff, 0xff),
    .driver_info = NCTRL(0) | NCTRL(1) | RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0032, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0033, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0034, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0037, 0xff, 0xff, 0xff),
    .driver_info = NCTRL(0) | NCTRL(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0038, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0039, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0040, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0042, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0043, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0044, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0048, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0049, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0050, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0051, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0052, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0054, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0055, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0056, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0057, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0058, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0061, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0062, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0063, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0064, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0065, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0066, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0067, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0069, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0076, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0077, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0078, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0079, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0082, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0083, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0086, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0087, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0088, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0089, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0090, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0091, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0092, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0093, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0094, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0095, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0096, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0097, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0104, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0105, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0106, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0108, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0113, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0117, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0118, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0121, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0122, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0123, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0124, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0125, 0xff, 0xff, 0xff),
    .driver_info = RSVD(6) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0126, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0128, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0135, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0136, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0137, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0139, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0142, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0143, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0144, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0145, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0148, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0151, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0153, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0155, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0156, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0157, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0158, 0xff, 0xff, 0xff),
    .driver_info = RSVD(3) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0159, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0161, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0162, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0164, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0165, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0167, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0189, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0191, 0xff, 0xff, 0xff), /* ZTE EuFi890 */
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0196, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0197, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0199, 0xff, 0xff, 0xff), /* ZTE MF820S */
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0200, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0201, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0254, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0257, 0xff, 0xff, 0xff), /* ZTE MF821 */
    .driver_info = RSVD(3) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0265, 0xff, 0xff, 0xff), /* ONDA MT8205 */
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0284, 0xff, 0xff, 0xff), /* ZTE MF880 */
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0317, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0326, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0330, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0395, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0412, 0xff, 0xff, 0xff), /* Telewell TW-LTE 4G */
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0414, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0417, 0xff, 0xff, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(ZTE_VENDOR_ID, 0x0601, 0xff) },	/* GosunCn ZTE WeLink ME3630 (RNDIS mode) */
    { USB_DEVICE_INTERFACE_CLASS(ZTE_VENDOR_ID, 0x0602, 0xff) },	/* GosunCn ZTE WeLink ME3630 (MBIM mode) */
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1008, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1010, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1012, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1018, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1021, 0xff, 0xff, 0xff),
    .driver_info = RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1057, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1058, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1059, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1060, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1061, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1062, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1063, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1064, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1065, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1066, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1067, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1068, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1069, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1070, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1071, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1072, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1073, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1074, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1075, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1076, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1077, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1078, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1079, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1080, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1081, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1082, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1083, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1084, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1085, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1086, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1087, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1088, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1089, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1090, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1091, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1092, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1093, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1094, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1095, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1096, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1097, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1098, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1099, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1100, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1101, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1102, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1103, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1104, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1105, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1106, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1107, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1108, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1109, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1110, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1111, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1112, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1113, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1114, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1115, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1116, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1117, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1118, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1119, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1120, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1121, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1122, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1123, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1124, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1125, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1126, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1127, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1128, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1129, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1130, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1131, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1132, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1133, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1134, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1135, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1136, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1137, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1138, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1139, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1140, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1141, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1142, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1143, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1144, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1145, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1146, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1147, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1148, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1149, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1150, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1151, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1152, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1153, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1154, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1155, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1156, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1157, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1158, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1159, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1160, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1161, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1162, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1163, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1164, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1165, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1166, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1167, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1168, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1169, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1170, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1244, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1245, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1246, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1247, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1248, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1249, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1250, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1251, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1252, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1253, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1254, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1255, 0xff, 0xff, 0xff),
    .driver_info = RSVD(3) | RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1256, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1257, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1258, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1259, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1260, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1261, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1262, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1263, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1264, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1265, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1266, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1267, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1268, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1269, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1270, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1271, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1272, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1273, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1274, 0xff, 0xff, 0xff) },
    { USB_DEVICE(ZTE_VENDOR_ID, 0x1275),	/* ZTE P685M */
    .driver_info = RSVD(3) | RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1276, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1277, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1278, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1279, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1280, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1281, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1282, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1283, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1284, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1285, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1286, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1287, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1288, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1289, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1290, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1291, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1292, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1293, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1294, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1295, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1296, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1297, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1298, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1299, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1300, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1301, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1302, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1303, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1333, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1401, 0xff, 0xff, 0xff),
    .driver_info = RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1402, 0xff, 0xff, 0xff),
    .driver_info = RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1424, 0xff, 0xff, 0xff),
    .driver_info = RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1425, 0xff, 0xff, 0xff),
    .driver_info = RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1426, 0xff, 0xff, 0xff),  /* ZTE MF91 */
    .driver_info = RSVD(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1428, 0xff, 0xff, 0xff),  /* Telewell TW-LTE 4G v2 */
    .driver_info = RSVD(2) },
    { USB_DEVICE_INTERFACE_CLASS(ZTE_VENDOR_ID, 0x1476, 0xff) },	/* GosunCn ZTE WeLink ME3630 (ECM/NCM mode) */
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1481, 0xff, 0x00, 0x00) }, /* ZTE MF871A */
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1485, 0xff, 0xff, 0xff),  /* ZTE MF286D */
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1533, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1534, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1535, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1545, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1546, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1547, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1565, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1566, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1567, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1589, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1590, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1591, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1592, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1594, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1596, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1598, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x1600, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x2002, 0xff, 0xff, 0xff),
    .driver_info = NCTRL(0) | NCTRL(1) | NCTRL(2) | RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x2003, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0014, 0xff, 0xff, 0xff) }, /* ZTE CDMA products */
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0027, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0059, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0060, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0070, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0073, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0130, 0xff, 0xff, 0xff),
    .driver_info = RSVD(1) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0133, 0xff, 0xff, 0xff),
    .driver_info = RSVD(3) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0141, 0xff, 0xff, 0xff),
    .driver_info = RSVD(5) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0147, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0152, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0168, 0xff, 0xff, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0170, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0176, 0xff, 0xff, 0xff),
    .driver_info = RSVD(3) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0x0178, 0xff, 0xff, 0xff),
    .driver_info = RSVD(3) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff42, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff43, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff44, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff45, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff46, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff47, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff48, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff49, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff4a, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff4b, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff4c, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff4d, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff4e, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff4f, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff50, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff51, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff52, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff53, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff54, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff55, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff56, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff57, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff58, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff59, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff5a, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff5b, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff5c, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff5d, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff5e, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff5f, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff60, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff61, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff62, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff63, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff64, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff65, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff66, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff67, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff68, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff69, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff6a, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff6b, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff6c, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff6d, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff6e, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff6f, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff70, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff71, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff72, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff73, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff74, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff75, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff76, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff77, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff78, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff79, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff7a, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff7b, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff7c, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff7d, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff7e, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff7f, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff80, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff81, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff82, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff83, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff84, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff85, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff86, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff87, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff88, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff89, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff8a, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff8b, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff8c, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff8d, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff8e, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff8f, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff90, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff91, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff92, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff93, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff94, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff9f, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa0, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa1, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa2, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa3, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa4, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa5, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa6, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa7, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa8, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffa9, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffaa, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffab, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffac, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffae, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffaf, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb0, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb1, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb2, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb3, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb4, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb5, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb6, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb7, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb8, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffb9, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffba, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffbb, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffbc, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffbd, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffbe, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffbf, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc0, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc1, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc2, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc3, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc4, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc5, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc6, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc7, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc8, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffc9, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffca, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffcb, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffcc, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffcd, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffce, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffcf, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffd0, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffd1, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffd2, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffd3, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffd4, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffd5, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffe9, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffec, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xffee, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xfff6, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xfff7, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xfff8, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xfff9, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xfffb, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xfffc, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_MG880, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_CDMA_TECH, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_AC2726, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_AC8710T, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_MC2718, 0xff, 0xff, 0xff),
    .driver_info = NCTRL(1) | NCTRL(2) | NCTRL(3) | NCTRL(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_AD3812, 0xff, 0xff, 0xff),
    .driver_info = NCTRL(0) | NCTRL(1) | NCTRL(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZTE_VENDOR_ID, ZTE_PRODUCT_MC2716, 0xff, 0xff, 0xff),
    .driver_info = NCTRL(1) | NCTRL(2) | NCTRL(3) },
    { USB_DEVICE(ZTE_VENDOR_ID, ZTE_PRODUCT_ME3620_L),
    .driver_info = RSVD(3) | RSVD(4) | RSVD(5) },
    { USB_DEVICE(ZTE_VENDOR_ID, ZTE_PRODUCT_ME3620_MBIM),
    .driver_info = RSVD(2) | RSVD(3) | RSVD(4) },
    { USB_DEVICE(ZTE_VENDOR_ID, ZTE_PRODUCT_ME3620_X),
    .driver_info = RSVD(3) | RSVD(4) | RSVD(5) },
    { USB_DEVICE(ZTE_VENDOR_ID, ZTE_PRODUCT_ZM8620_X),
    .driver_info = RSVD(3) | RSVD(4) | RSVD(5) },
    { USB_VENDOR_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff, 0x02, 0x01) },
    { USB_VENDOR_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff, 0x02, 0x05) },
    { USB_VENDOR_AND_INTERFACE_INFO(ZTE_VENDOR_ID, 0xff, 0x86, 0x10) },
    { USB_DEVICE(BENQ_VENDOR_ID, BENQ_PRODUCT_H10) },
    { USB_DEVICE(DLINK_VENDOR_ID, DLINK_PRODUCT_DWM_652) },
    { USB_DEVICE(ALINK_VENDOR_ID, DLINK_PRODUCT_DWM_652_U5) }, /* Yes, ALINK_VENDOR_ID */
    { USB_DEVICE(ALINK_VENDOR_ID, DLINK_PRODUCT_DWM_652_U5A) },
    { USB_DEVICE(QISDA_VENDOR_ID, QISDA_PRODUCT_H21_4512) },
    { USB_DEVICE(QISDA_VENDOR_ID, QISDA_PRODUCT_H21_4523) },
    { USB_DEVICE(QISDA_VENDOR_ID, QISDA_PRODUCT_H20_4515) },
    { USB_DEVICE(QISDA_VENDOR_ID, QISDA_PRODUCT_H20_4518) },
    { USB_DEVICE(QISDA_VENDOR_ID, QISDA_PRODUCT_H20_4519) },
    { USB_DEVICE(TOSHIBA_VENDOR_ID, TOSHIBA_PRODUCT_G450) },
    { USB_DEVICE(TOSHIBA_VENDOR_ID, TOSHIBA_PRODUCT_HSDPA_MINICARD ) }, /* Toshiba 3G HSDPA == Novatel Expedite EU870D MiniCard */
    { USB_DEVICE(ALINK_VENDOR_ID, 0x9000) },
    { USB_DEVICE(ALINK_VENDOR_ID, ALINK_PRODUCT_PH300) },
    { USB_DEVICE_AND_INTERFACE_INFO(ALINK_VENDOR_ID, ALINK_PRODUCT_3GU, 0xff, 0xff, 0xff) },
    { USB_DEVICE(ALINK_VENDOR_ID, SIMCOM_PRODUCT_SIM7100E),
    .driver_info = RSVD(5) | RSVD(6) },
    { USB_DEVICE_INTERFACE_CLASS(0x1e0e, 0x9003, 0xff) },	/* Simcom SIM7500/SIM7600 MBIM mode */
    { USB_DEVICE_INTERFACE_CLASS(0x1e0e, 0x9011, 0xff),	/* Simcom SIM7500/SIM7600 RNDIS mode */
    .driver_info = RSVD(7) },
    { USB_DEVICE(0x1e0e, 0x9071),				/* Simcom SIM8230 RMNET mode */
    .driver_info = RSVD(3) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x1e0e, 0x9078, 0xff),	/* Simcom SIM8230 ECM mode */
    .driver_info = RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x1e0e, 0x907b, 0xff),	/* Simcom SIM8230 RNDIS mode */
    .driver_info = RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x1e0e, 0x9205, 0xff) },	/* Simcom SIM7070/SIM7080/SIM7090 AT+ECM mode */
    { USB_DEVICE_INTERFACE_CLASS(0x1e0e, 0x9206, 0xff) },	/* Simcom SIM7070/SIM7080/SIM7090 AT-only mode */
    { USB_DEVICE(ALCATEL_VENDOR_ID, ALCATEL_PRODUCT_X060S_X200),
    .driver_info = NCTRL(0) | NCTRL(1) | RSVD(4) },
    { USB_DEVICE(ALCATEL_VENDOR_ID, ALCATEL_PRODUCT_X220_X500D),
    .driver_info = RSVD(6) },
    { USB_DEVICE(ALCATEL_VENDOR_ID, 0x0052),
    .driver_info = RSVD(6) },
    { USB_DEVICE(ALCATEL_VENDOR_ID, 0x00b6),
    .driver_info = RSVD(3) },
    { USB_DEVICE(ALCATEL_VENDOR_ID, 0x00b7),
    .driver_info = RSVD(5) },
    { USB_DEVICE(ALCATEL_VENDOR_ID, ALCATEL_PRODUCT_L100V),
    .driver_info = RSVD(4) },
    { USB_DEVICE(ALCATEL_VENDOR_ID, ALCATEL_PRODUCT_L800MA),
    .driver_info = RSVD(2) },
    { USB_DEVICE(AIRPLUS_VENDOR_ID, AIRPLUS_PRODUCT_MCD650) },
    { USB_DEVICE(TLAYTECH_VENDOR_ID, TLAYTECH_PRODUCT_TEU800) },
    { USB_DEVICE(LONGCHEER_VENDOR_ID, FOUR_G_SYSTEMS_PRODUCT_CARSTICK_LTE),
    .driver_info = RSVD(0) },
    { USB_DEVICE(LONGCHEER_VENDOR_ID, FOUR_G_SYSTEMS_PRODUCT_W14),
    .driver_info = NCTRL(0) | NCTRL(1) },
    { USB_DEVICE(LONGCHEER_VENDOR_ID, FOUR_G_SYSTEMS_PRODUCT_W100),
    .driver_info = NCTRL(1) | NCTRL(2) | RSVD(3) },
    {USB_DEVICE(LONGCHEER_VENDOR_ID, FUJISOFT_PRODUCT_FS040U),
    .driver_info = RSVD(3)},
    { USB_DEVICE_INTERFACE_CLASS(LONGCHEER_VENDOR_ID, SPEEDUP_PRODUCT_SU9800, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(LONGCHEER_VENDOR_ID, 0x9801, 0xff),
    .driver_info = RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(LONGCHEER_VENDOR_ID, 0x9803, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE(LONGCHEER_VENDOR_ID, 0x9b05),	/* Longsung U8300 */
    .driver_info = RSVD(4) | RSVD(5) },
    { USB_DEVICE(LONGCHEER_VENDOR_ID, 0x9b3c),	/* Longsung U9300 */
    .driver_info = RSVD(0) | RSVD(4) },
    { USB_DEVICE(LONGCHEER_VENDOR_ID, ZOOM_PRODUCT_4597) },
    { USB_DEVICE(LONGCHEER_VENDOR_ID, IBALL_3_5G_CONNECT) },
    { USB_DEVICE(HAIER_VENDOR_ID, HAIER_PRODUCT_CE100) },
    { USB_DEVICE_AND_INTERFACE_INFO(HAIER_VENDOR_ID, HAIER_PRODUCT_CE81B, 0xff, 0xff, 0xff) },
// Pirelli
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_C100_1, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_C100_2, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_1004, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_1005, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_1006, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_1007, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_1008, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_1009, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_100A, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_100B, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_100C, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_100D, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_100E, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_100F, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_1011, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(PIRELLI_VENDOR_ID, PIRELLI_PRODUCT_1012, 0xff) },
// Cinterion
    { USB_DEVICE(CINTERION_VENDOR_ID, CINTERION_PRODUCT_EU3_E) },
    { USB_DEVICE(CINTERION_VENDOR_ID, CINTERION_PRODUCT_EU3_P) },
    { USB_DEVICE(CINTERION_VENDOR_ID, CINTERION_PRODUCT_PH8),
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_AHXX, 0xff) },
    { USB_DEVICE(CINTERION_VENDOR_ID, CINTERION_PRODUCT_PLXX),
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_PH8_2RMNET, 0xff),
    .driver_info = RSVD(4) | RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_PH8_AUDIO, 0xff),
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_AHXX_2RMNET, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_AHXX_AUDIO, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_CLS8, 0xff),
    .driver_info = RSVD(0) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_EXS82, 0xff) },
    { USB_DEVICE(CINTERION_VENDOR_ID, CINTERION_PRODUCT_HC28_MDM) },
    { USB_DEVICE(CINTERION_VENDOR_ID, CINTERION_PRODUCT_HC28_MDMNET) },
    { USB_DEVICE(SIEMENS_VENDOR_ID, CINTERION_PRODUCT_HC25_MDM) },
    { USB_DEVICE(SIEMENS_VENDOR_ID, CINTERION_PRODUCT_HC25_MDMNET) },
    { USB_DEVICE(SIEMENS_VENDOR_ID, CINTERION_PRODUCT_HC28_MDM) }, /* HC28 enumerates with Siemens or Cinterion VID depending on FW revision */
    { USB_DEVICE(SIEMENS_VENDOR_ID, CINTERION_PRODUCT_HC28_MDMNET) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_MV31_MBIM, 0xff),
    .driver_info = RSVD(3)},
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_MV31_RMNET, 0xff),
    .driver_info = RSVD(0)},
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_MV31_2_MBIM, 0xff),
    .driver_info = RSVD(3)},
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_MV31_2_RMNET, 0xff),
    .driver_info = RSVD(0)},
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_MV32_WA, 0xff),
    .driver_info = RSVD(3)},
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_MV32_WA_RMNET, 0xff),
    .driver_info = RSVD(0) },
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_MV32_WB, 0xff),
    .driver_info = RSVD(3)},
    { USB_DEVICE_INTERFACE_CLASS(CINTERION_VENDOR_ID, CINTERION_PRODUCT_MV32_WB_RMNET, 0xff),
    .driver_info = RSVD(0) },
    { USB_DEVICE(OLIVETTI_VENDOR_ID, OLIVETTI_PRODUCT_OLICARD100),
    .driver_info = RSVD(4) },
    { USB_DEVICE(OLIVETTI_VENDOR_ID, OLIVETTI_PRODUCT_OLICARD120),
    .driver_info = RSVD(4) },
    { USB_DEVICE(OLIVETTI_VENDOR_ID, OLIVETTI_PRODUCT_OLICARD140),
    .driver_info = RSVD(4) },
    { USB_DEVICE(OLIVETTI_VENDOR_ID, OLIVETTI_PRODUCT_OLICARD145) },
    { USB_DEVICE(OLIVETTI_VENDOR_ID, OLIVETTI_PRODUCT_OLICARD155),
    .driver_info = RSVD(6) },
    { USB_DEVICE(OLIVETTI_VENDOR_ID, OLIVETTI_PRODUCT_OLICARD200),
    .driver_info = RSVD(6) },
    { USB_DEVICE(OLIVETTI_VENDOR_ID, OLIVETTI_PRODUCT_OLICARD160),
    .driver_info = RSVD(6) },
    { USB_DEVICE(OLIVETTI_VENDOR_ID, OLIVETTI_PRODUCT_OLICARD500),
    .driver_info = RSVD(4) },
    { USB_DEVICE(CELOT_VENDOR_ID, CELOT_PRODUCT_CT680M) }, /* CT-650 CDMA 450 1xEVDO modem */
    { USB_DEVICE_AND_INTERFACE_INFO(SAMSUNG_VENDOR_ID, SAMSUNG_PRODUCT_GT_B3730, USB_CLASS_CDC_DATA, 0x00, 0x00) }, /* Samsung GT-B3730 LTE USB modem.*/
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEM600) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEM610) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEM500) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEM510) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEM800) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEM900) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU818) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU816) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU828) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU826) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU518) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU516) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU528) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU526) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWM600) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWM610) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWM500) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWM510) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWM800) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWM900) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU718) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU716) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU728) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU726) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU518) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU516) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU528) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU526) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLM600) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLM610) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLM500) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLM510) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLM800) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLM900) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLU718) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLU716) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLU728) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLU726) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLU518) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLU516) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLU528) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CLU526) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU881) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CEU882) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU581) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU582) },
    { USB_DEVICE(YUGA_VENDOR_ID, YUGA_PRODUCT_CWU583) },
    { USB_DEVICE_AND_INTERFACE_INFO(VIETTEL_VENDOR_ID, VIETTEL_PRODUCT_VT1000, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(ZD_VENDOR_ID, ZD_PRODUCT_7000, 0xff, 0xff, 0xff) },
    { USB_DEVICE(LG_VENDOR_ID, LG_PRODUCT_L02C) }, /* docomo L-02C modem */
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, 0x00a1, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, 0x00a1, 0xff, 0x02, 0x01) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, 0x00a2, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, 0x00a2, 0xff, 0x02, 0x01) },        /* MediaTek MT6276M modem & app port */
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_DC_1COM, 0x0a, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_DC_5COM, 0xff, 0x02, 0x01) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_DC_5COM, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_DC_4COM, 0xff, 0x02, 0x01) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_DC_4COM, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_7208_1COM, 0x02, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_7208_2COM, 0x02, 0x02, 0x01) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_FP_1COM, 0x0a, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_FP_2COM, 0x0a, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_FPDC_1COM, 0x0a, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_FPDC_2COM, 0x0a, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_7103_2COM, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_7106_2COM, 0x02, 0x02, 0x01) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_DC_4COM2, 0xff, 0x02, 0x01) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, MEDIATEK_PRODUCT_DC_4COM2, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, 0x7126, 0xff, 0x00, 0x00),
    .driver_info = NCTRL(2) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, 0x7127, 0xff, 0x00, 0x00),
    .driver_info = NCTRL(2) | NCTRL(3) | NCTRL(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(MEDIATEK_VENDOR_ID, 0x7129, 0xff, 0x00, 0x00),        /* MediaTek T7XX  */
    .driver_info = NCTRL(2) | NCTRL(3) | NCTRL(4) },
    { USB_DEVICE(CELLIENT_VENDOR_ID, CELLIENT_PRODUCT_MEN200) },
    { USB_DEVICE(CELLIENT_VENDOR_ID, CELLIENT_PRODUCT_MPL200),
    .driver_info = RSVD(1) | RSVD(4) },
    { USB_DEVICE(PETATEL_VENDOR_ID, PETATEL_PRODUCT_NP10T_600A) },
    { USB_DEVICE(PETATEL_VENDOR_ID, PETATEL_PRODUCT_NP10T_600E) },
    { USB_DEVICE_AND_INTERFACE_INFO(TPLINK_VENDOR_ID, TPLINK_PRODUCT_LTE, 0xff, 0x00, 0x00) },	/* TP-Link LTE Module */
    { USB_DEVICE(TPLINK_VENDOR_ID, TPLINK_PRODUCT_MA180),
    .driver_info = RSVD(4) },
    { USB_DEVICE(TPLINK_VENDOR_ID, 0x9000),					/* TP-Link MA260 */
    .driver_info = RSVD(4) },
    { USB_DEVICE(CHANGHONG_VENDOR_ID, CHANGHONG_PRODUCT_CH690) },
    { USB_DEVICE_INTERFACE_CLASS(0x2001, 0x7d01, 0xff) },			/* D-Link DWM-156 (variant) */
    { USB_DEVICE_INTERFACE_CLASS(0x2001, 0x7d02, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(0x2001, 0x7d03, 0xff) },
    { USB_DEVICE_INTERFACE_CLASS(0x2001, 0x7d04, 0xff),			/* D-Link DWM-158 */
    .driver_info = RSVD(4) | RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x2001, 0x7d0e, 0xff) },			/* D-Link DWM-157 C1 */
    { USB_DEVICE_INTERFACE_CLASS(0x2001, 0x7e19, 0xff),			/* D-Link DWM-221 B1 */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x2001, 0x7e35, 0xff),			/* D-Link DWM-222 */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x2001, 0x7e3d, 0xff),			/* D-Link DWM-222 A2 */
    .driver_info = RSVD(4) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x07d1, 0x3e01, 0xff, 0xff, 0xff) },	/* D-Link DWM-152/C1 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x07d1, 0x3e02, 0xff, 0xff, 0xff) },	/* D-Link DWM-156/C1 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x07d1, 0x7e11, 0xff, 0xff, 0xff) },	/* D-Link DWM-156/A3 */
    { USB_DEVICE_INTERFACE_CLASS(0x1435, 0xd191, 0xff),			/* Wistron Neweb D19Q1 */
    .driver_info = RSVD(1) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x1690, 0x7588, 0xff),			/* ASKEY WWHC050 */
    .driver_info = RSVD(1) | RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x2020, 0x2031, 0xff),			/* Olicard 600 */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x2020, 0x2033, 0xff),			/* BroadMobi BM806U */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x2020, 0x2060, 0xff),			/* BroadMobi BM818 */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x2020, 0x4000, 0xff) },			/* OLICARD300 - MT6225 */
    { USB_DEVICE(INOVIA_VENDOR_ID, INOVIA_SEW858) },
    { USB_DEVICE(VIATELECOM_VENDOR_ID, VIATELECOM_PRODUCT_CDS7) },
    { USB_DEVICE_AND_INTERFACE_INFO(WETELECOM_VENDOR_ID, WETELECOM_PRODUCT_WMD200, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(WETELECOM_VENDOR_ID, WETELECOM_PRODUCT_6802, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(WETELECOM_VENDOR_ID, WETELECOM_PRODUCT_WMD300, 0xff, 0xff, 0xff) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x03f0, 0x421d, 0xff, 0xff, 0xff) },	/* HP lt2523 (Novatel E371) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x03f0, 0xa31d, 0xff, 0x06, 0x10) },	/* HP lt4132 (Huawei ME906s-158) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x03f0, 0xa31d, 0xff, 0x06, 0x12) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x03f0, 0xa31d, 0xff, 0x06, 0x13) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x03f0, 0xa31d, 0xff, 0x06, 0x14) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x03f0, 0xa31d, 0xff, 0x06, 0x1b) },
    { USB_DEVICE(0x0489, 0xe0b4),						/* Foxconn T77W968 */
    .driver_info = RSVD(0) | RSVD(1) | RSVD(6) },
    { USB_DEVICE(0x0489, 0xe0b5),						/* Foxconn T77W968 ESIM */
    .driver_info = RSVD(0) | RSVD(1) | RSVD(6) },
    { USB_DEVICE_INTERFACE_CLASS(0x0489, 0xe0da, 0xff),                     /* Foxconn T99W265 MBIM variant */
    .driver_info = RSVD(3) | RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x0489, 0xe0db, 0xff),			/* Foxconn T99W265 MBIM */
    .driver_info = RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(0x0489, 0xe0ee, 0xff),			/* Foxconn T99W368 MBIM */
    .driver_info = RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(0x0489, 0xe0f0, 0xff),			/* Foxconn T99W373 MBIM */
    .driver_info = RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(0x0489, 0xe123, 0xff),			/* Foxconn T99W760 MBIM */
    .driver_info = RSVD(3) },
    { USB_DEVICE_INTERFACE_CLASS(0x0489, 0xe145, 0xff),			/* Foxconn T99W651 RNDIS */
    .driver_info = RSVD(5) | RSVD(6) },
    { USB_DEVICE_INTERFACE_CLASS(0x0489, 0xe15f, 0xff),                     /* Foxconn T99W709 */
    .driver_info = RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x0489, 0xe167, 0xff),                     /* Foxconn T99W640 MBIM */
    .driver_info = RSVD(3) },
    { USB_DEVICE(0x1508, 0x1001),						/* Fibocom NL668 (IOT version) */
    .driver_info = RSVD(4) | RSVD(5) | RSVD(6) },
    { USB_DEVICE(0x1782, 0x4d10) },						/* Fibocom L610 (AT mode) */
    { USB_DEVICE_INTERFACE_CLASS(0x1782, 0x4d11, 0xff) },			/* Fibocom L610 (ECM/RNDIS mode) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2cb7, 0x0001, 0xff, 0xff, 0xff) },	/* Fibocom L716-EU (ECM/RNDIS mode) */
    { USB_DEVICE(0x2cb7, 0x0104),						/* Fibocom NL678 series */
    .driver_info = RSVD(4) | RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x0105, 0xff),			/* Fibocom NL678 series */
    .driver_info = RSVD(6) },
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x0106, 0xff) },			/* Fibocom MA510 (ECM mode w/ diag intf.) */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x010a, 0xff) },			/* Fibocom MA510 (ECM mode) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2cb7, 0x010b, 0xff, 0xff, 0x30) },	/* Fibocom FG150 Diag */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2cb7, 0x010b, 0xff, 0, 0) },		/* Fibocom FG150 AT */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x0111, 0xff) },			/* Fibocom FM160 (MBIM mode) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2cb7, 0x0112, 0xff, 0xff, 0x30) },	/* Fibocom FG132 Diag */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2cb7, 0x0112, 0xff, 0xff, 0x40) },	/* Fibocom FG132 AT */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2cb7, 0x0112, 0xff, 0, 0) },		/* Fibocom FG132 NMEA */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x0115, 0xff),			/* Fibocom FM135 (laptop MBIM) */
    .driver_info = RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x01a0, 0xff) },			/* Fibocom NL668-AM/NL652-EU (laptop MBIM) */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x01a2, 0xff) },			/* Fibocom FM101-GL (laptop MBIM) */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x01a3, 0xff) },			/* Fibocom FM101-GL (laptop MBIM) */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x01a4, 0xff),			/* Fibocom FM101-GL (laptop MBIM) */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x0a04, 0xff) },			/* Fibocom FM650-CN (ECM mode) */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x0a05, 0xff) },			/* Fibocom FM650-CN (NCM mode) */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x0a06, 0xff) },			/* Fibocom FM650-CN (RNDIS mode) */
    { USB_DEVICE_INTERFACE_CLASS(0x2cb7, 0x0a07, 0xff) },			/* Fibocom FM650-CN (MBIM mode) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d41, 0xff, 0, 0) },		/* MeiG Smart SLM320 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d57, 0xff, 0, 0) },		/* MeiG Smart SLM770A */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d22, 0xff, 0, 0) },		/* MeiG Smart SRM815 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d22, 0xff, 0x10, 0x02) },	/* MeiG Smart SLM828 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d22, 0xff, 0x10, 0x03) },	/* MeiG Smart SLM828 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d22, 0xff, 0xff, 0x30) },	/* MeiG Smart SRM815 and SRM825L */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d22, 0xff, 0xff, 0x40) },	/* MeiG Smart SRM825L */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d22, 0xff, 0xff, 0x60) },	/* MeiG Smart SRM825L */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d38, 0xff, 0xff, 0x30) },	/* MeiG Smart SRM825WN (Diag) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d38, 0xff, 0xff, 0x40) },	/* MeiG Smart SRM825WN (AT) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d38, 0xff, 0xff, 0x60) },	/* MeiG Smart SRM825WN (NMEA) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d63, 0xff, 0xff, 0x30) },	/* MeiG SRM813Q (Diag) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d63, 0xff, 0xff, 0x40) },	/* MeiG SRM813Q (AT) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d64, 0xff, 0xff, 0x30) },	/* MeiG SRM813Q (Diag) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d64, 0xff, 0xff, 0x40) },	/* MeiG SRM813Q (AT) */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2dee, 0x4d64, 0xff, 0xff, 0x60) },	/* MeiG SRM813Q (NMEA) */
    { USB_DEVICE_INTERFACE_CLASS(0x2df3, 0x9d03, 0xff) },			/* LongSung M5710 */
    { USB_DEVICE_INTERFACE_CLASS(0x305a, 0x1404, 0xff) },			/* GosunCn GM500 RNDIS */
    { USB_DEVICE_INTERFACE_CLASS(0x305a, 0x1405, 0xff) },			/* GosunCn GM500 MBIM */
    { USB_DEVICE_INTERFACE_CLASS(0x305a, 0x1406, 0xff) },			/* GosunCn GM500 ECM/NCM */
    { USB_DEVICE(0x33f8, 0x0104),						/* Rolling RW101-GL (laptop RMNET) */
    .driver_info = RSVD(4) | RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x0115, 0xff),			/* Rolling RW135-GL (laptop MBIM) */
    .driver_info = RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x01a2, 0xff) },			/* Rolling RW101-GL (laptop MBIM) */
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x01a3, 0xff) },			/* Rolling RW101-GL (laptop MBIM) */
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x01a4, 0xff),			/* Rolling RW101-GL (laptop MBIM) */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x01a8, 0xff),			/* Rolling RW101R-GL (laptop MBIM) */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x01a9, 0xff),			/* Rolling RW101R-GL (laptop MBIM) */
    .driver_info = RSVD(4) },
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x0301, 0xff) },			/* Rolling RW101R-GL (laptop MBIM) */
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x0302, 0xff) },			/* Rolling RW101R-GL (laptop MBIM) */
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x0802, 0xff),			/* Rolling RW350-GL (laptop MBIM) */
    .driver_info = RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x33f8, 0x1003, 0xff),			/* Rolling RW135R-GL (laptop MBIM) */
    .driver_info = RSVD(5) },
    { USB_DEVICE_INTERFACE_CLASS(0x3466, 0x3301, 0xff) },			/* TDTECH MT5710-CN */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0100, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WWD for Global */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0100, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0100, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0101, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WRD for Global SKU */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0101, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0101, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0106, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WRD for China SKU */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0106, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0106, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0111, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WWD for SA */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0111, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0111, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0112, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WWD for EU */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0112, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0112, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0113, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WWD for NA */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0113, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0113, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0115, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WWD for China EDU */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0115, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0115, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0116, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WWD for Golbal EDU */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0116, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x0116, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010a, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WRD for WWAN Ready */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010a, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010a, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010b, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WWD for WWAN Ready */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010b, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010b, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010c, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WRD for WWAN Ready */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010c, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010c, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010d, 0xff, 0xff, 0x30) },	/* NetPrisma LCUK54-WWD for WWAN Ready */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010d, 0xff, 0x00, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x3731, 0x010d, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(OPPO_VENDOR_ID, OPPO_PRODUCT_R11, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(SIERRA_VENDOR_ID, SIERRA_PRODUCT_EM9191, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(SIERRA_VENDOR_ID, SIERRA_PRODUCT_EM9191, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(SIERRA_VENDOR_ID, SIERRA_PRODUCT_EM9191, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(SIERRA_VENDOR_ID, SIERRA_PRODUCT_EM9291, 0xff, 0xff, 0x30) },
    { USB_DEVICE_AND_INTERFACE_INFO(SIERRA_VENDOR_ID, SIERRA_PRODUCT_EM9291, 0xff, 0xff, 0x40) },
    { USB_DEVICE_AND_INTERFACE_INFO(UNISOC_VENDOR_ID, TOZED_PRODUCT_LT70C, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(UNISOC_VENDOR_ID, UNISOC_PRODUCT_UIS7720, 0xff, 0, 0) },
    { USB_DEVICE_AND_INTERFACE_INFO(UNISOC_VENDOR_ID, LUAT_PRODUCT_AIR720U, 0xff, 0, 0) },
    { USB_DEVICE_INTERFACE_CLASS(0x1bbb, 0x0530, 0xff),			/* TCL IK512 MBIM */
    .driver_info = NCTRL(1) },
    { USB_DEVICE_INTERFACE_CLASS(0x1bbb, 0x0640, 0xff),			/* TCL IK512 ECM */
    .driver_info = NCTRL(3) },
    { USB_DEVICE_INTERFACE_CLASS(0x2949, 0x8700, 0xff) },			/* Neoway N723-EA */
    { } /* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, option_ids);
// The card has three separate interfaces, which the serial driver
// recognizes separately, thus num_port=1.
//
    static struct usb_serial_driver option_1port_device = {
    .driver = {
    .name =		"option1",
    },
    .description       = "GSM modem (1-port)",
    .id_table          = option_ids,
    .num_ports         = 1,
    .probe             = option_probe,
    .open              = usb_wwan_open,
    .close             = usb_wwan_close,
    .dtr_rts	   = usb_wwan_dtr_rts,
    .write             = usb_wwan_write,
    .write_room        = usb_wwan_write_room,
    .chars_in_buffer   = usb_wwan_chars_in_buffer,
    .tiocmget          = usb_wwan_tiocmget,
    .tiocmset          = usb_wwan_tiocmset,
    .attach            = option_attach,
    .release           = option_release,
    .port_probe        = usb_wwan_port_probe,
    .port_remove	   = usb_wwan_port_remove,
    .read_int_callback = option_instat_callback,

    .suspend           = usb_wwan_suspend,
    .resume            = usb_wwan_resume,

    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &option_1port_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, option_ids);
#[no_mangle]
unsafe extern "C" fn iface_is_reserved(device_flags: c_ulong, ifnum: u8) -> bool {
    static bool iface_is_reserved(unsigned long device_flags, u8 ifnum)
    {
    if (ifnum > FLAG_IFNUM_MAX)
    return false;
    return device_flags & RSVD(ifnum);
    }
    static int option_probe(struct usb_serial *serial,
    const struct usb_device_id *id)
    {
    struct usb_interface_descriptor *iface_desc =
    &serial.interface.cur_altsetting.desc;
    let mut device_flags: c_ulong = id.driver_info;
// Never bind to the CD-Rom emulation interface
    if (iface_desc.bInterfaceClass == USB_CLASS_MASS_STORAGE)
    return -ENODEV;
//
// Don't bind reserved interfaces (like network ones) which often have
// the same class/subclass/protocol as the serial interfaces.  Look at
// the Windows driver .INF files for reserved interface numbers.
//
    if (iface_is_reserved(device_flags, iface_desc.bInterfaceNumber))
    return -ENODEV;
//
// Allow matching on bNumEndpoints for devices whose interface numbers
// can change (e.g. Quectel EP06).
//
    if (device_flags & NUMEP2 && iface_desc.bNumEndpoints != 2)
    return -ENODEV;
// Store the device flags so we can use them during attach.
    usb_set_serial_data(serial, (void *)device_flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iface_no_modem_control(device_flags: c_ulong, ifnum: u8) -> bool {
    static bool iface_no_modem_control(unsigned long device_flags, u8 ifnum)
    {
    if (ifnum > FLAG_IFNUM_MAX)
    return false;
    return device_flags & NCTRL(ifnum);
    }
#[no_mangle]
unsafe extern "C" fn option_attach(serial: *mut usb_serial) -> c_int {
    static int option_attach(struct usb_serial *serial)
    {
    struct usb_interface_descriptor *iface_desc;
    struct usb_wwan_intf_private *data;
    unsigned long device_flags;
    data = kzalloc_obj(struct usb_wwan_intf_private);
    if (!data)
    return -ENOMEM;
// Retrieve device flags stored at probe.
    device_flags = (unsigned long)usb_get_serial_data(serial);
    iface_desc = &serial.interface.cur_altsetting.desc;
    if (!iface_no_modem_control(device_flags, iface_desc.bInterfaceNumber))
    data.use_send_setup = 1;
    if (device_flags & ZLP)
    data.use_zlp = 1;
    spin_lock_init(&data.susp_lock);
    usb_set_serial_data(serial, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn option_release(serial: *mut usb_serial) {
    static void option_release(struct usb_serial *serial)
    {
    struct usb_wwan_intf_private *intfdata = usb_get_serial_data(serial);
    kfree(intfdata);
    }
#[no_mangle]
unsafe extern "C" fn option_instat_callback(urb: *mut urb) {
    static void option_instat_callback(struct urb *urb)
    {
    int err;
    let mut status: c_int = urb.status;
    struct usb_serial_port *port = urb.context;
    struct device *dev = &port.dev;
    struct usb_wwan_port_private *portdata =
    usb_get_serial_port_data(port);
    dev_dbg(dev, "%s: urb %p port %p has data %p\n", __func__, urb, port, portdata);
    if (status == 0) {
    struct usb_ctrlrequest *req_pkt = urb.transfer_buffer;
    if (!req_pkt) {
    dev_dbg(dev, "%s: core::ptr::null_mut() req_pkt\n", __func__);
    return;
    }
    if (urb.actual_length < sizeof(*req_pkt)) {
    dev_err(dev, "%s: short packet: %u bytes\n", __func__,
    urb.actual_length);
    return;
    }
    if ((req_pkt.bRequestType == 0xA1) &&
    (req_pkt.bRequest == 0x20)) {
    unsigned char signals;
    int old_dcd_state;
    if (urb.actual_length < sizeof(*req_pkt) + 1) {
    dev_err(dev, "%s: short interrupt transfer: %u bytes\n",
    __func__, urb.actual_length);
    return;
    }
    signals = *((unsigned char *)urb.transfer_buffer +
    sizeof(*req_pkt));
    dev_dbg(dev, "%s: signal x%x\n", __func__, signals);
    old_dcd_state = portdata.dcd_state;
    portdata.cts_state = 1;
    portdata.dcd_state = ((signals & 0x01) ? 1 : 0);
    portdata.dsr_state = ((signals & 0x02) ? 1 : 0);
    portdata.ri_state = ((signals & 0x08) ? 1 : 0);
    if (old_dcd_state && !portdata.dcd_state)
    tty_port_tty_hangup(&port.port, true);
    } else {
    dev_dbg(dev, "%s: type %x req %x\n", __func__,
    req_pkt.bRequestType, req_pkt.bRequest);
    }
    } else if (status == -ENOENT || status == -ESHUTDOWN) {
    dev_dbg(dev, "%s: urb stopped: %d\n", __func__, status);
    } else
    dev_dbg(dev, "%s: error %d\n", __func__, status);
// Resubmit urb so we continue receiving IRQ data
    if (status != -ESHUTDOWN && status != -ENOENT) {
    usb_mark_last_busy(port.serial.dev);
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if (err)
    dev_dbg(dev, "%s: resubmit intr urb failed. (%d)\n",
    __func__, err);
    }
    }
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL v2");
