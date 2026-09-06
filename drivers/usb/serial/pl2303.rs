//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/pl2303.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Prolific PL2303 USB to serial adaptor driver header file
//
pub const BENQ_VENDOR_ID: c_uint = 0x04a5;
pub const BENQ_PRODUCT_ID_S81: c_uint = 0x4027;
pub const PL2303_VENDOR_ID: c_uint = 0x067b;
pub const PL2303_PRODUCT_ID: c_uint = 0x2303;
pub const PL2303_PRODUCT_ID_TB: c_uint = 0x2304;
pub const PL2303_PRODUCT_ID_GC: c_uint = 0x23a3;
pub const PL2303_PRODUCT_ID_GB: c_uint = 0x23b3;
pub const PL2303_PRODUCT_ID_GT: c_uint = 0x23c3;
pub const PL2303_PRODUCT_ID_GL: c_uint = 0x23d3;
pub const PL2303_PRODUCT_ID_GE: c_uint = 0x23e3;
pub const PL2303_PRODUCT_ID_GS: c_uint = 0x23f3;
pub const PL2303_PRODUCT_ID_RSAQ2: c_uint = 0x04bb;
pub const PL2303_PRODUCT_ID_DCU11: c_uint = 0x1234;
pub const PL2303_PRODUCT_ID_PHAROS: c_uint = 0xaaa0;
pub const PL2303_PRODUCT_ID_RSAQ3: c_uint = 0xaaa2;
pub const PL2303_PRODUCT_ID_CHILITAG: c_uint = 0xaaa8;
pub const PL2303_PRODUCT_ID_ALDIGA: c_uint = 0x0611;
pub const PL2303_PRODUCT_ID_MMX: c_uint = 0x0612;
pub const PL2303_PRODUCT_ID_GPRS: c_uint = 0x0609;
pub const PL2303_PRODUCT_ID_HCR331: c_uint = 0x331a;
pub const PL2303_PRODUCT_ID_MOTOROLA: c_uint = 0x0307;
pub const PL2303_PRODUCT_ID_ZTEK: c_uint = 0xe1f1;
pub const PL256X_PRODUCT_ID_4P: c_uint = 0x2533;
pub const ATEN_VENDOR_ID: c_uint = 0x0557;
pub const ATEN_VENDOR_ID2: c_uint = 0x0547;
pub const ATEN_PRODUCT_ID: c_uint = 0x2008;
pub const ATEN_PRODUCT_UC485: c_uint = 0x2021;
pub const ATEN_PRODUCT_UC232B: c_uint = 0x2022;
pub const ATEN_PRODUCT_ID2: c_uint = 0x2118;
pub const IBM_VENDOR_ID: c_uint = 0x04b3;
pub const IBM_PRODUCT_ID: c_uint = 0x4016;
pub const IODATA_VENDOR_ID: c_uint = 0x04bb;
pub const IODATA_PRODUCT_ID: c_uint = 0x0a03;
pub const IODATA_PRODUCT_ID_RSAQ5: c_uint = 0x0a0e;
pub const ELCOM_VENDOR_ID: c_uint = 0x056e;
pub const ELCOM_PRODUCT_ID: c_uint = 0x5003;
pub const ELCOM_PRODUCT_ID_UCSGT: c_uint = 0x5004;
pub const ITEGNO_VENDOR_ID: c_uint = 0x0eba;
pub const ITEGNO_PRODUCT_ID: c_uint = 0x1080;
pub const ITEGNO_PRODUCT_ID_2080: c_uint = 0x2080;
pub const MA620_VENDOR_ID: c_uint = 0x0df7;
pub const MA620_PRODUCT_ID: c_uint = 0x0620;
pub const RATOC_VENDOR_ID: c_uint = 0x0584;
pub const RATOC_PRODUCT_ID: c_uint = 0xb000;
pub const TRIPP_VENDOR_ID: c_uint = 0x2478;
pub const TRIPP_PRODUCT_ID: c_uint = 0x2008;
pub const RADIOSHACK_VENDOR_ID: c_uint = 0x1453;
pub const RADIOSHACK_PRODUCT_ID: c_uint = 0x4026;
pub const DCU10_VENDOR_ID: c_uint = 0x0731;
pub const DCU10_PRODUCT_ID: c_uint = 0x0528;
pub const SITECOM_VENDOR_ID: c_uint = 0x6189;
pub const SITECOM_PRODUCT_ID: c_uint = 0x2068;
// Alcatel OT535/735 USB cable
pub const ALCATEL_VENDOR_ID: c_uint = 0x11f7;
pub const ALCATEL_PRODUCT_ID: c_uint = 0x02df;
pub const SIEMENS_VENDOR_ID: c_uint = 0x11f5;
pub const SIEMENS_PRODUCT_ID_SX1: c_uint = 0x0001;
pub const SIEMENS_PRODUCT_ID_X65: c_uint = 0x0003;
pub const SIEMENS_PRODUCT_ID_X75: c_uint = 0x0004;
pub const SIEMENS_PRODUCT_ID_EF81: c_uint = 0x0005;
pub const SYNTECH_VENDOR_ID: c_uint = 0x0745;
pub const SYNTECH_PRODUCT_ID: c_uint = 0x0001;
// Nokia CA-42 Cable
pub const NOKIA_CA42_VENDOR_ID: c_uint = 0x078b;
pub const NOKIA_CA42_PRODUCT_ID: c_uint = 0x1234;
// CA-42 CLONE Cable www.ca-42.com chipset: Prolific Technology Inc
pub const CA_42_CA42_VENDOR_ID: c_uint = 0x10b5;
pub const CA_42_CA42_PRODUCT_ID: c_uint = 0xac70;
pub const SAGEM_VENDOR_ID: c_uint = 0x079b;
pub const SAGEM_PRODUCT_ID: c_uint = 0x0027;
// Leadtek GPS 9531 (ID 0413:2101)
pub const LEADTEK_VENDOR_ID: c_uint = 0x0413;
pub const LEADTEK_9531_PRODUCT_ID: c_uint = 0x2101;
// USB GSM cable from Speed Dragon Multimedia, Ltd
pub const SPEEDDRAGON_VENDOR_ID: c_uint = 0x0e55;
pub const SPEEDDRAGON_PRODUCT_ID: c_uint = 0x110b;
// DATAPILOT Universal-2 Phone Cable
pub const DATAPILOT_U2_VENDOR_ID: c_uint = 0x0731;
pub const DATAPILOT_U2_PRODUCT_ID: c_uint = 0x2003;
// Belkin "F5U257" Serial Adapter
pub const BELKIN_VENDOR_ID: c_uint = 0x050d;
pub const BELKIN_PRODUCT_ID: c_uint = 0x0257;
// Alcor Micro Corp. USB 2.0 TO RS-232
pub const ALCOR_VENDOR_ID: c_uint = 0x058F;
pub const ALCOR_PRODUCT_ID: c_uint = 0x9720;
// Willcom WS002IN Data Driver (by NetIndex Inc.)
pub const WS002IN_VENDOR_ID: c_uint = 0x11f6;
pub const WS002IN_PRODUCT_ID: c_uint = 0x2001;
// Corega CG-USBRS232R Serial Adapter
pub const COREGA_VENDOR_ID: c_uint = 0x07aa;
pub const COREGA_PRODUCT_ID: c_uint = 0x002a;
// Y.C. Cable U.S.A., Inc - USB to RS-232
pub const YCCABLE_VENDOR_ID: c_uint = 0x05ad;
pub const YCCABLE_PRODUCT_ID: c_uint = 0x0fba;
// "Superial" USB - Serial
pub const SUPERIAL_VENDOR_ID: c_uint = 0x5372;
pub const SUPERIAL_PRODUCT_ID: c_uint = 0x2303;
// Hewlett-Packard POS Pole Displays
pub const HP_VENDOR_ID: c_uint = 0x03f0;
pub const HP_LD381GC_PRODUCT_ID: c_uint = 0x0183;
pub const HP_LM920_PRODUCT_ID: c_uint = 0x026b;
pub const HP_TD620_PRODUCT_ID: c_uint = 0x0956;
pub const HP_LD960_PRODUCT_ID: c_uint = 0x0b39;
pub const HP_LD381_PRODUCT_ID: c_uint = 0x0f7f;
pub const HP_LM930_PRODUCT_ID: c_uint = 0x0f9b;
pub const HP_LCM220_PRODUCT_ID: c_uint = 0x3139;
pub const HP_LCM960_PRODUCT_ID: c_uint = 0x3239;
pub const HP_LD220_PRODUCT_ID: c_uint = 0x3524;
pub const HP_LD220TA_PRODUCT_ID: c_uint = 0x4349;
pub const HP_LD960TA_PRODUCT_ID: c_uint = 0x4439;
pub const HP_LM940_PRODUCT_ID: c_uint = 0x5039;
// Cressi Edy (diving computer) PC interface
pub const CRESSI_VENDOR_ID: c_uint = 0x04b8;
pub const CRESSI_EDY_PRODUCT_ID: c_uint = 0x0521;
// Zeagle dive computer interface
pub const ZEAGLE_VENDOR_ID: c_uint = 0x04b8;
pub const ZEAGLE_N2ITION3_PRODUCT_ID: c_uint = 0x0522;
// Sony, USB data cable for CMD-Jxx mobile phones
pub const SONY_VENDOR_ID: c_uint = 0x054c;
pub const SONY_QN3USB_PRODUCT_ID: c_uint = 0x0437;
// Sanwa KB-USB2 multimeter cable (ID: 11ad:0001)
pub const SANWA_VENDOR_ID: c_uint = 0x11ad;
pub const SANWA_PRODUCT_ID: c_uint = 0x0001;
// ADLINK ND-6530 RS232,RS485 and RS422 adapter
pub const ADLINK_VENDOR_ID: c_uint = 0x0b63;
pub const ADLINK_ND6530_PRODUCT_ID: c_uint = 0x6530;
pub const ADLINK_ND6530GC_PRODUCT_ID: c_uint = 0x653a;
// SMART USB Serial Adapter
pub const SMART_VENDOR_ID: c_uint = 0x0b8c;
pub const SMART_PRODUCT_ID: c_uint = 0x2303;
// Allied Telesis VT-Kit3
pub const AT_VENDOR_ID: c_uint = 0x0caa;
pub const AT_VTKIT3_PRODUCT_ID: c_uint = 0x3001;
// Macrosilicon MS3020
pub const MACROSILICON_VENDOR_ID: c_uint = 0x345f;
pub const MACROSILICON_MS3020_PRODUCT_ID: c_uint = 0x3020;
