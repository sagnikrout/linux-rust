//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/stv06xx/stv06xx_hdcs.h
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
// Copyright (c) 2008 Chia-I Wu
//
// P/N 861037:      Sensor HDCS1000        ASIC STV0600
// P/N 861050-0010: Sensor HDCS1000        ASIC STV0600
// P/N 861050-0020: Sensor Photobit PB100  ASIC STV0600-1 - QuickCam Express
// P/N 861055:      Sensor ST VV6410       ASIC STV0610   - LEGO cam
// P/N 861075-0040: Sensor HDCS1000        ASIC
// P/N 961179-0700: Sensor ST VV6410       ASIC STV0602   - Dexxa WebCam USB
// P/N 861040-0000: Sensor ST VV6410       ASIC STV0610   - QuickCam Web
//

pub const HDCS_1X00_DEF_WIDTH: c_int = 360;
pub const HDCS_1X00_DEF_HEIGHT: c_int = 296;
pub const HDCS_1020_DEF_WIDTH: c_int = 352;
pub const HDCS_1020_DEF_HEIGHT: c_int = 292;
pub const HDCS_1020_BOTTOM_Y_SKIP: c_int = 4;
pub const HDCS_CLK_FREQ_MHZ: c_int = 25;
pub const HDCS_ADC_START_SIG_DUR: c_int = 3;
// LSB bit of I2C or register address signifies write (0) or read (1)
// I2C Registers common for both HDCS-1000/1100 and HDCS-1020
// Identifications Register

// Status Register

// Interrupt Mask Register

// Pad Control Register

// Pad Drive Control Register

// Interface Control Register

// Interface Timing Register

// Baud Fraction Register

// Baud Rate Register

// ADC Control Register

// First Window Row Register

// First Window Column Register

// Last Window Row Register

// Last Window Column Register

// Timing Control Register

// PGA Gain Register: Even Row, Even Column

// PGA Gain Register: Even Row, Odd Column

// PGA Gain Register: Odd Row, Even Column

// PGA Gain Register: Odd Row, Odd Column

// Row Exposure Low Register

// Row Exposure High Register

// I2C Registers only for HDCS-1000/1100
// Sub-Row Exposure Low Register

// Sub-Row Exposure High Register

// Configuration Register

// Control Register

// I2C Registers only for HDCS-1020
// Sub-Row Exposure Register

// Error Control Register

// Interface Timing 2 Register

// Interface Control 2 Register

// Horizontal Blank Register

// Vertical Blank Register

// Configuration Register

// Control Register

pub const HDCS_DEFAULT_EXPOSURE: c_int = 48;
pub const HDCS_DEFAULT_GAIN: c_int = 50;
extern "C" {
    pub fn hdcs_probe_1x00(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn hdcs_probe_1020(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn hdcs_start(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn hdcs_init(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn hdcs_init_controls(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn hdcs_stop(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn hdcs_dump(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn hdcs_set_exposure(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn hdcs_set_gain(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
// FIXME (see if we can lower min_packet_size, needs testing, and also
// Clear status (writing 1 will clear the corresponding status bit)
// Disable all interrupts
// ADC output resolution to 10 bits
