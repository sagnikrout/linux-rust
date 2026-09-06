//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/zc3xx-reg.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// zc030x registers
//
// Copyright (c) 2008 Mauro Carvalho Chehab <mchehab@kernel.org>
//
// The register aliases used here came from this driver:
// http://zc0302.sourceforge.net/zc0302.php
//
// Define the register map
pub const ZC3XX_R000_SYSTEMCONTROL: c_uint = 0x0000;
pub const ZC3XX_R001_SYSTEMOPERATING: c_uint = 0x0001;
// Picture size
pub const ZC3XX_R002_CLOCKSELECT: c_uint = 0x0002;
pub const ZC3XX_R003_FRAMEWIDTHHIGH: c_uint = 0x0003;
pub const ZC3XX_R004_FRAMEWIDTHLOW: c_uint = 0x0004;
pub const ZC3XX_R005_FRAMEHEIGHTHIGH: c_uint = 0x0005;
pub const ZC3XX_R006_FRAMEHEIGHTLOW: c_uint = 0x0006;
// JPEG control
pub const ZC3XX_R008_CLOCKSETTING: c_uint = 0x0008;
// Test mode
pub const ZC3XX_R00B_TESTMODECONTROL: c_uint = 0x000b;
// Frame retrieving
pub const ZC3XX_R00C_LASTACQTIME: c_uint = 0x000c;
pub const ZC3XX_R00D_MONITORRES: c_uint = 0x000d;
pub const ZC3XX_R00E_TIMESTAMPHIGH: c_uint = 0x000e;
pub const ZC3XX_R00F_TIMESTAMPLOW: c_uint = 0x000f;
pub const ZC3XX_R018_FRAMELOST: c_uint = 0x0018;
pub const ZC3XX_R019_AUTOADJUSTFPS: c_uint = 0x0019;
pub const ZC3XX_R01A_LASTFRAMESTATE: c_uint = 0x001a;
pub const ZC3XX_R025_DATACOUNTER: c_uint = 0x0025;
// Stream and sensor specific
pub const ZC3XX_R010_CMOSSENSORSELECT: c_uint = 0x0010;
pub const ZC3XX_R011_VIDEOSTATUS: c_uint = 0x0011;
pub const ZC3XX_R012_VIDEOCONTROLFUNC: c_uint = 0x0012;
// Horizontal and vertical synchros
pub const ZC3XX_R01D_HSYNC_0: c_uint = 0x001d;
pub const ZC3XX_R01E_HSYNC_1: c_uint = 0x001e;
pub const ZC3XX_R01F_HSYNC_2: c_uint = 0x001f;
pub const ZC3XX_R020_HSYNC_3: c_uint = 0x0020;
// Target picture size in byte
pub const ZC3XX_R022_TARGETPICTSIZE_0: c_uint = 0x0022;
pub const ZC3XX_R023_TARGETPICTSIZE_1: c_uint = 0x0023;
pub const ZC3XX_R024_TARGETPICTSIZE_2: c_uint = 0x0024;
// Audio registers
pub const ZC3XX_R030_AUDIOADC: c_uint = 0x0030;
pub const ZC3XX_R031_AUDIOSTREAMSTATUS: c_uint = 0x0031;
pub const ZC3XX_R032_AUDIOSTATUS: c_uint = 0x0032;
// Sensor interface
pub const ZC3XX_R080_HBLANKHIGH: c_uint = 0x0080;
pub const ZC3XX_R081_HBLANKLOW: c_uint = 0x0081;
pub const ZC3XX_R082_RESETLEVELADDR: c_uint = 0x0082;
pub const ZC3XX_R083_RGAINADDR: c_uint = 0x0083;
pub const ZC3XX_R084_GGAINADDR: c_uint = 0x0084;
pub const ZC3XX_R085_BGAINADDR: c_uint = 0x0085;
pub const ZC3XX_R086_EXPTIMEHIGH: c_uint = 0x0086;
pub const ZC3XX_R087_EXPTIMEMID: c_uint = 0x0087;
pub const ZC3XX_R088_EXPTIMELOW: c_uint = 0x0088;
pub const ZC3XX_R089_RESETBLACKHIGH: c_uint = 0x0089;
pub const ZC3XX_R08A_RESETWHITEHIGH: c_uint = 0x008a;
pub const ZC3XX_R08B_I2CDEVICEADDR: c_uint = 0x008b;
pub const ZC3XX_R08C_I2CIDLEANDNACK: c_uint = 0x008c;
pub const ZC3XX_R08D_COMPABILITYMODE: c_uint = 0x008d;
pub const ZC3XX_R08E_COMPABILITYMODE2: c_uint = 0x008e;
// I2C control
pub const ZC3XX_R090_I2CCOMMAND: c_uint = 0x0090;
pub const ZC3XX_R091_I2CSTATUS: c_uint = 0x0091;
pub const ZC3XX_R092_I2CADDRESSSELECT: c_uint = 0x0092;
pub const ZC3XX_R093_I2CSETVALUE: c_uint = 0x0093;
pub const ZC3XX_R094_I2CWRITEACK: c_uint = 0x0094;
pub const ZC3XX_R095_I2CREAD: c_uint = 0x0095;
pub const ZC3XX_R096_I2CREADACK: c_uint = 0x0096;
// Window inside the sensor array
pub const ZC3XX_R097_WINYSTARTHIGH: c_uint = 0x0097;
pub const ZC3XX_R098_WINYSTARTLOW: c_uint = 0x0098;
pub const ZC3XX_R099_WINXSTARTHIGH: c_uint = 0x0099;
pub const ZC3XX_R09A_WINXSTARTLOW: c_uint = 0x009a;
pub const ZC3XX_R09B_WINHEIGHTHIGH: c_uint = 0x009b;
pub const ZC3XX_R09C_WINHEIGHTLOW: c_uint = 0x009c;
pub const ZC3XX_R09D_WINWIDTHHIGH: c_uint = 0x009d;
pub const ZC3XX_R09E_WINWIDTHLOW: c_uint = 0x009e;
pub const ZC3XX_R119_FIRSTYHIGH: c_uint = 0x0119;
pub const ZC3XX_R11A_FIRSTYLOW: c_uint = 0x011a;
pub const ZC3XX_R11B_FIRSTXHIGH: c_uint = 0x011b;
pub const ZC3XX_R11C_FIRSTXLOW: c_uint = 0x011c;
// Max sensor array size
pub const ZC3XX_R09F_MAXXHIGH: c_uint = 0x009f;
pub const ZC3XX_R0A0_MAXXLOW: c_uint = 0x00a0;
pub const ZC3XX_R0A1_MAXYHIGH: c_uint = 0x00a1;
pub const ZC3XX_R0A2_MAXYLOW: c_uint = 0x00a2;
pub const ZC3XX_R0A3_EXPOSURETIMEHIGH: c_uint = 0x00a3;
pub const ZC3XX_R0A4_EXPOSURETIMELOW: c_uint = 0x00a4;
pub const ZC3XX_R0A5_EXPOSUREGAIN: c_uint = 0x00a5;
pub const ZC3XX_R0A6_EXPOSUREBLACKLVL: c_uint = 0x00a6;
// Other registers
pub const ZC3XX_R100_OPERATIONMODE: c_uint = 0x0100;
pub const ZC3XX_R101_SENSORCORRECTION: c_uint = 0x0101;
// Gains
pub const ZC3XX_R116_RGAIN: c_uint = 0x0116;
pub const ZC3XX_R117_GGAIN: c_uint = 0x0117;
pub const ZC3XX_R118_BGAIN: c_uint = 0x0118;
pub const ZC3XX_R11D_GLOBALGAIN: c_uint = 0x011d;
pub const ZC3XX_R1A8_DIGITALGAIN: c_uint = 0x01a8;
pub const ZC3XX_R1A9_DIGITALLIMITDIFF: c_uint = 0x01a9;
pub const ZC3XX_R1AA_DIGITALGAINSTEP: c_uint = 0x01aa;
// Auto correction
pub const ZC3XX_R180_AUTOCORRECTENABLE: c_uint = 0x0180;
pub const ZC3XX_R181_WINXSTART: c_uint = 0x0181;
pub const ZC3XX_R182_WINXWIDTH: c_uint = 0x0182;
pub const ZC3XX_R183_WINXCENTER: c_uint = 0x0183;
pub const ZC3XX_R184_WINYSTART: c_uint = 0x0184;
pub const ZC3XX_R185_WINYWIDTH: c_uint = 0x0185;
pub const ZC3XX_R186_WINYCENTER: c_uint = 0x0186;
// Gain range
pub const ZC3XX_R187_MAXGAIN: c_uint = 0x0187;
pub const ZC3XX_R188_MINGAIN: c_uint = 0x0188;
// Auto exposure and white balance
pub const ZC3XX_R189_AWBSTATUS: c_uint = 0x0189;
pub const ZC3XX_R18A_AWBFREEZE: c_uint = 0x018a;
pub const ZC3XX_R18B_AESTATUS: c_uint = 0x018b;
pub const ZC3XX_R18C_AEFREEZE: c_uint = 0x018c;
pub const ZC3XX_R18F_AEUNFREEZE: c_uint = 0x018f;
pub const ZC3XX_R190_EXPOSURELIMITHIGH: c_uint = 0x0190;
pub const ZC3XX_R191_EXPOSURELIMITMID: c_uint = 0x0191;
pub const ZC3XX_R192_EXPOSURELIMITLOW: c_uint = 0x0192;
pub const ZC3XX_R195_ANTIFLICKERHIGH: c_uint = 0x0195;
pub const ZC3XX_R196_ANTIFLICKERMID: c_uint = 0x0196;
pub const ZC3XX_R197_ANTIFLICKERLOW: c_uint = 0x0197;
// What is this ?
pub const ZC3XX_R18D_YTARGET: c_uint = 0x018d;
pub const ZC3XX_R18E_RESETLVL: c_uint = 0x018e;
// Color
pub const ZC3XX_R1A0_REDMEANAFTERAGC: c_uint = 0x01a0;
pub const ZC3XX_R1A1_GREENMEANAFTERAGC: c_uint = 0x01a1;
pub const ZC3XX_R1A2_BLUEMEANAFTERAGC: c_uint = 0x01a2;
pub const ZC3XX_R1A3_REDMEANAFTERAWB: c_uint = 0x01a3;
pub const ZC3XX_R1A4_GREENMEANAFTERAWB: c_uint = 0x01a4;
pub const ZC3XX_R1A5_BLUEMEANAFTERAWB: c_uint = 0x01a5;
pub const ZC3XX_R1A6_YMEANAFTERAE: c_uint = 0x01a6;
pub const ZC3XX_R1A7_CALCGLOBALMEAN: c_uint = 0x01a7;
// Matrixes
// Color matrix is like :
//
pub const ZC3XX_R10A_RGB00: c_uint = 0x010a;
pub const ZC3XX_R10B_RGB01: c_uint = 0x010b;
pub const ZC3XX_R10C_RGB02: c_uint = 0x010c;
pub const ZC3XX_R113_RGB03: c_uint = 0x0113;
pub const ZC3XX_R10D_RGB10: c_uint = 0x010d;
pub const ZC3XX_R10E_RGB11: c_uint = 0x010e;
pub const ZC3XX_R10F_RGB12: c_uint = 0x010f;
pub const ZC3XX_R114_RGB13: c_uint = 0x0114;
pub const ZC3XX_R110_RGB20: c_uint = 0x0110;
pub const ZC3XX_R111_RGB21: c_uint = 0x0111;
pub const ZC3XX_R112_RGB22: c_uint = 0x0112;
pub const ZC3XX_R115_RGB23: c_uint = 0x0115;
// Gamma matrix
pub const ZC3XX_R120_GAMMA00: c_uint = 0x0120;
pub const ZC3XX_R121_GAMMA01: c_uint = 0x0121;
pub const ZC3XX_R122_GAMMA02: c_uint = 0x0122;
pub const ZC3XX_R123_GAMMA03: c_uint = 0x0123;
pub const ZC3XX_R124_GAMMA04: c_uint = 0x0124;
pub const ZC3XX_R125_GAMMA05: c_uint = 0x0125;
pub const ZC3XX_R126_GAMMA06: c_uint = 0x0126;
pub const ZC3XX_R127_GAMMA07: c_uint = 0x0127;
pub const ZC3XX_R128_GAMMA08: c_uint = 0x0128;
pub const ZC3XX_R129_GAMMA09: c_uint = 0x0129;
pub const ZC3XX_R12A_GAMMA0A: c_uint = 0x012a;
pub const ZC3XX_R12B_GAMMA0B: c_uint = 0x012b;
pub const ZC3XX_R12C_GAMMA0C: c_uint = 0x012c;
pub const ZC3XX_R12D_GAMMA0D: c_uint = 0x012d;
pub const ZC3XX_R12E_GAMMA0E: c_uint = 0x012e;
pub const ZC3XX_R12F_GAMMA0F: c_uint = 0x012f;
pub const ZC3XX_R130_GAMMA10: c_uint = 0x0130;
pub const ZC3XX_R131_GAMMA11: c_uint = 0x0131;
pub const ZC3XX_R132_GAMMA12: c_uint = 0x0132;
pub const ZC3XX_R133_GAMMA13: c_uint = 0x0133;
pub const ZC3XX_R134_GAMMA14: c_uint = 0x0134;
pub const ZC3XX_R135_GAMMA15: c_uint = 0x0135;
pub const ZC3XX_R136_GAMMA16: c_uint = 0x0136;
pub const ZC3XX_R137_GAMMA17: c_uint = 0x0137;
pub const ZC3XX_R138_GAMMA18: c_uint = 0x0138;
pub const ZC3XX_R139_GAMMA19: c_uint = 0x0139;
pub const ZC3XX_R13A_GAMMA1A: c_uint = 0x013a;
pub const ZC3XX_R13B_GAMMA1B: c_uint = 0x013b;
pub const ZC3XX_R13C_GAMMA1C: c_uint = 0x013c;
pub const ZC3XX_R13D_GAMMA1D: c_uint = 0x013d;
pub const ZC3XX_R13E_GAMMA1E: c_uint = 0x013e;
pub const ZC3XX_R13F_GAMMA1F: c_uint = 0x013f;
// Luminance gamma
pub const ZC3XX_R140_YGAMMA00: c_uint = 0x0140;
pub const ZC3XX_R141_YGAMMA01: c_uint = 0x0141;
pub const ZC3XX_R142_YGAMMA02: c_uint = 0x0142;
pub const ZC3XX_R143_YGAMMA03: c_uint = 0x0143;
pub const ZC3XX_R144_YGAMMA04: c_uint = 0x0144;
pub const ZC3XX_R145_YGAMMA05: c_uint = 0x0145;
pub const ZC3XX_R146_YGAMMA06: c_uint = 0x0146;
pub const ZC3XX_R147_YGAMMA07: c_uint = 0x0147;
pub const ZC3XX_R148_YGAMMA08: c_uint = 0x0148;
pub const ZC3XX_R149_YGAMMA09: c_uint = 0x0149;
pub const ZC3XX_R14A_YGAMMA0A: c_uint = 0x014a;
pub const ZC3XX_R14B_YGAMMA0B: c_uint = 0x014b;
pub const ZC3XX_R14C_YGAMMA0C: c_uint = 0x014c;
pub const ZC3XX_R14D_YGAMMA0D: c_uint = 0x014d;
pub const ZC3XX_R14E_YGAMMA0E: c_uint = 0x014e;
pub const ZC3XX_R14F_YGAMMA0F: c_uint = 0x014f;
pub const ZC3XX_R150_YGAMMA10: c_uint = 0x0150;
pub const ZC3XX_R151_YGAMMA11: c_uint = 0x0151;
pub const ZC3XX_R1C5_SHARPNESSMODE: c_uint = 0x01c5;
pub const ZC3XX_R1C6_SHARPNESS00: c_uint = 0x01c6;
pub const ZC3XX_R1C7_SHARPNESS01: c_uint = 0x01c7;
pub const ZC3XX_R1C8_SHARPNESS02: c_uint = 0x01c8;
pub const ZC3XX_R1C9_SHARPNESS03: c_uint = 0x01c9;
pub const ZC3XX_R1CA_SHARPNESS04: c_uint = 0x01ca;
pub const ZC3XX_R1CB_SHARPNESS05: c_uint = 0x01cb;
// Dead pixels
pub const ZC3XX_R250_DEADPIXELSMODE: c_uint = 0x0250;
// EEPROM
pub const ZC3XX_R300_EEPROMCONFIG: c_uint = 0x0300;
pub const ZC3XX_R301_EEPROMACCESS: c_uint = 0x0301;
pub const ZC3XX_R302_EEPROMSTATUS: c_uint = 0x0302;
