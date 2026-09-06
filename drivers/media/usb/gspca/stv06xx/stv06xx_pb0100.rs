//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/stv06xx/stv06xx_pb0100.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2001 Jean-Fredric Clere, Nikolas Zimmermann, Georg Acher
// Mark Cave-Ayland, Carlo E Prelz, Dick Streefland
// Copyright (c) 2002, 2003 Tuukka Toivonen
// Copyright (c) 2008 Erik Andrén
//
// P/N 861037:      Sensor HDCS1000        ASIC STV0600
// P/N 861050-0010: Sensor HDCS1000        ASIC STV0600
// P/N 861050-0020: Sensor Photobit PB100  ASIC STV0600-1 - QuickCam Express
// P/N 861055:      Sensor ST VV6410       ASIC STV0610   - LEGO cam
// P/N 861075-0040: Sensor HDCS1000        ASIC
// P/N 961179-0700: Sensor ST VV6410       ASIC STV0602   - Dexxa WebCam USB
// P/N 861040-0000: Sensor ST VV6410       ASIC STV0610   - QuickCam Web
//

// mode priv field flags
pub const PB0100_CROP_TO_VGA: c_uint = 0x01;
pub const PB0100_SUBSAMPLE: c_uint = 0x02;
// I2C Registers
pub const PB_IDENT: c_uint = 0x00	/* Chip Version */;
pub const PB_RSTART: c_uint = 0x01	/* Row Window Start */;
pub const PB_CSTART: c_uint = 0x02	/* Column Window Start */;
pub const PB_RWSIZE: c_uint = 0x03	/* Row Window Size */;
pub const PB_CWSIZE: c_uint = 0x04	/* Column  Window Size */;
pub const PB_CFILLIN: c_uint = 0x05	/* Column Fill-In */;
pub const PB_VBL: c_uint = 0x06	/* Vertical Blank Count */;
pub const PB_CONTROL: c_uint = 0x07	/* Control Mode */;
pub const PB_FINTTIME: c_uint = 0x08	/* Integration Time/Frame Unit Count */;
pub const PB_RINTTIME: c_uint = 0x09	/* Integration Time/Row Unit Count */;
pub const PB_ROWSPEED: c_uint = 0x0a	/* Row Speed Control */;
pub const PB_ABORTFRAME: c_uint = 0x0b	/* Abort Frame */;
pub const PB_R12: c_uint = 0x0c	/* Reserved */;
pub const PB_RESET: c_uint = 0x0d	/* Reset */;
pub const PB_EXPGAIN: c_uint = 0x0e	/* Exposure Gain Command */;
pub const PB_R15: c_uint = 0x0f	/* Expose0 */;
pub const PB_R16: c_uint = 0x10	/* Expose1 */;
pub const PB_R17: c_uint = 0x11	/* Expose2 */;
pub const PB_R18: c_uint = 0x12	/* Low0_DAC */;
pub const PB_R19: c_uint = 0x13	/* Low1_DAC */;
pub const PB_R20: c_uint = 0x14	/* Low2_DAC */;
pub const PB_R21: c_uint = 0x15	/* Threshold11 */;
pub const PB_R22: c_uint = 0x16	/* Threshold0x */;
pub const PB_UPDATEINT: c_uint = 0x17	/* Update Interval */;
pub const PB_R24: c_uint = 0x18	/* High_DAC */;
pub const PB_R25: c_uint = 0x19	/* Trans0H */;
pub const PB_R26: c_uint = 0x1a	/* Trans1L */;
pub const PB_R27: c_uint = 0x1b	/* Trans1H */;
pub const PB_R28: c_uint = 0x1c	/* Trans2L */;
pub const PB_R29: c_uint = 0x1d	/* Reserved */;
pub const PB_R30: c_uint = 0x1e	/* Reserved */;
pub const PB_R31: c_uint = 0x1f	/* Wait to Read */;
pub const PB_PREADCTRL: c_uint = 0x20	/* Pixel Read Control Mode */;
pub const PB_R33: c_uint = 0x21	/* IREF_VLN */;
pub const PB_R34: c_uint = 0x22	/* IREF_VLP */;
pub const PB_R35: c_uint = 0x23	/* IREF_VLN_INTEG */;
pub const PB_R36: c_uint = 0x24	/* IREF_MASTER */;
pub const PB_R37: c_uint = 0x25	/* IDACP */;
pub const PB_R38: c_uint = 0x26	/* IDACN */;
pub const PB_R39: c_uint = 0x27	/* DAC_Control_Reg */;
pub const PB_R40: c_uint = 0x28	/* VCL */;
pub const PB_R41: c_uint = 0x29	/* IREF_VLN_ADCIN */;
pub const PB_R42: c_uint = 0x2a	/* Reserved */;
pub const PB_G1GAIN: c_uint = 0x2b	/* Green 1 Gain */;
pub const PB_BGAIN: c_uint = 0x2c	/* Blue Gain */;
pub const PB_RGAIN: c_uint = 0x2d	/* Red Gain */;
pub const PB_G2GAIN: c_uint = 0x2e	/* Green 2 Gain */;
pub const PB_R47: c_uint = 0x2f	/* Dark Row Address */;
pub const PB_R48: c_uint = 0x30	/* Dark Row Options */;
pub const PB_R49: c_uint = 0x31	/* Reserved */;
pub const PB_R50: c_uint = 0x32	/* Image Test Data */;
pub const PB_ADCMAXGAIN: c_uint = 0x33	/* Maximum Gain */;
pub const PB_ADCMINGAIN: c_uint = 0x34	/* Minimum Gain */;
pub const PB_ADCGLOBALGAIN: c_uint = 0x35	/* Global Gain */;
pub const PB_R54: c_uint = 0x36	/* Maximum Frame */;
pub const PB_R55: c_uint = 0x37	/* Minimum Frame */;
pub const PB_R56: c_uint = 0x38	/* Reserved */;
pub const PB_VOFFSET: c_uint = 0x39	/* VOFFSET */;
pub const PB_R58: c_uint = 0x3a	/* Snap-Shot Sequence Trigger */;
pub const PB_ADCGAINH: c_uint = 0x3b	/* VREF_HI */;
pub const PB_ADCGAINL: c_uint = 0x3c	/* VREF_LO */;
pub const PB_R61: c_uint = 0x3d	/* Reserved */;
pub const PB_R62: c_uint = 0x3e	/* Reserved */;
pub const PB_R63: c_uint = 0x3f	/* Reserved */;
pub const PB_R64: c_uint = 0x40	/* Red/Blue Gain */;
pub const PB_R65: c_uint = 0x41	/* Green 2/Green 1 Gain */;
pub const PB_R66: c_uint = 0x42	/* VREF_HI/LO */;
pub const PB_R67: c_uint = 0x43	/* Integration Time/Row Unit Count */;
pub const PB_R240: c_uint = 0xf0	/* ADC Test */;
pub const PB_R241: c_uint = 0xf1    /* Chip Enable */;
pub const PB_R242: c_uint = 0xf2	/* Reserved */;
extern "C" {
    pub fn pb0100_probe(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn pb0100_start(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn pb0100_init(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn pb0100_init_controls(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn pb0100_stop(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn pb0100_dump(sd: *mut sd) -> static int;
}
// V4L2 controls supported by the driver
extern "C" {
    pub fn pb0100_set_gain(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn pb0100_set_red_balance(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn pb0100_set_blue_balance(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn pb0100_set_exposure(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn pb0100_set_autogain(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn pb0100_set_autogain_target(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
