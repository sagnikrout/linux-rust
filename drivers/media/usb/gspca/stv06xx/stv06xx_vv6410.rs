//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/stv06xx/stv06xx_vv6410.h
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

pub const VV6410_COLS: c_int = 416;
pub const VV6410_ROWS: c_int = 320;
// Status registers
// Chip identification number including revision indicator
pub const VV6410_DEVICEH: c_uint = 0x00;
pub const VV6410_DEVICEL: c_uint = 0x01;
// User can determine whether timed I2C data
pub const VV6410_STATUS0: c_uint = 0x02;
// Current line counter value
pub const VV6410_LINECOUNTH: c_uint = 0x03;
pub const VV6410_LINECOUNTL: c_uint = 0x04;
// End x coordinate of image size
pub const VV6410_XENDH: c_uint = 0x05;
pub const VV6410_XENDL: c_uint = 0x06;
// End y coordinate of image size
pub const VV6410_YENDH: c_uint = 0x07;
pub const VV6410_YENDL: c_uint = 0x08;
// This is the average pixel value returned from the
pub const VV6410_DARKAVGH: c_uint = 0x09;
pub const VV6410_DARKAVGL: c_uint = 0x0a;
// This is the average pixel value returned from the
pub const VV6410_BLACKAVGH: c_uint = 0x0b;
pub const VV6410_BLACKAVGL: c_uint = 0x0c;
// Flags to indicate whether the x or y image coordinates have been clipped
pub const VV6410_STATUS1: c_uint = 0x0d;
// Setup registers
// Low-power/sleep modes & video timing
pub const VV6410_SETUP0: c_uint = 0x10;
// Various parameters
pub const VV6410_SETUP1: c_uint = 0x11;
// Contains pixel counter reset value used by external sync
pub const VV6410_SYNCVALUE: c_uint = 0x12;
// Frame grabbing modes (FST, LST and QCK)
pub const VV6410_FGMODES: c_uint = 0x14;
// FST and QCK mapping modes.
pub const VV6410_PINMAPPING: c_uint = 0x15;
// Data resolution
pub const VV6410_DATAFORMAT: c_uint = 0x16;
// Output coding formats
pub const VV6410_OPFORMAT: c_uint = 0x17;
// Various mode select bits
pub const VV6410_MODESELECT: c_uint = 0x18;
// Exposure registers
// Fine exposure.
pub const VV6410_FINEH: c_uint = 0x20;
pub const VV6410_FINEL: c_uint = 0x21;
// Coarse exposure
pub const VV6410_COARSEH: c_uint = 0x22;
pub const VV6410_COARSEL: c_uint = 0x23;
// Analog gain setting
pub const VV6410_ANALOGGAIN: c_uint = 0x24;
// Clock division
pub const VV6410_CLKDIV: c_uint = 0x25;
// Dark line offset cancellation value
pub const VV6410_DARKOFFSETH: c_uint = 0x2c;
pub const VV6410_DARKOFFSETL: c_uint = 0x2d;
// Dark line offset cancellation enable
pub const VV6410_DARKOFFSETSETUP: c_uint = 0x2e;
// Video timing registers
// Line Length (Pixel Clocks)
pub const VV6410_LINELENGTHH: c_uint = 0x52;
pub const VV6410_LINELENGTHL: c_uint = 0x53;
// X-co-ordinate of top left corner of region of interest (x-offset)
pub const VV6410_XOFFSETH: c_uint = 0x57;
pub const VV6410_XOFFSETL: c_uint = 0x58;
// Y-coordinate of top left corner of region of interest (y-offset)
pub const VV6410_YOFFSETH: c_uint = 0x59;
pub const VV6410_YOFFSETL: c_uint = 0x5a;
// Field length (Lines)
pub const VV6410_FIELDLENGTHH: c_uint = 0x61;
pub const VV6410_FIELDLENGTHL: c_uint = 0x62;
// System registers
// Black offset cancellation default value
pub const VV6410_BLACKOFFSETH: c_uint = 0x70;
pub const VV6410_BLACKOFFSETL: c_uint = 0x71;
// Black offset cancellation setup
pub const VV6410_BLACKOFFSETSETUP: c_uint = 0x72;
// Analog Control Register 0
pub const VV6410_CR0: c_uint = 0x75;
// Analog Control Register 1
pub const VV6410_CR1: c_uint = 0x76;
// ADC Setup Register
pub const VV6410_AS0: c_uint = 0x77;
// Analog Test Register
pub const VV6410_AT0: c_uint = 0x78;
// Audio Amplifier Setup Register
pub const VV6410_AT1: c_uint = 0x79;

pub const VV6410_FINE_EXPOSURE: c_int = 320;
pub const VV6410_COARSE_EXPOSURE: c_int = 192;
pub const VV6410_DEFAULT_GAIN: c_int = 5;
pub const VV6410_SUBSAMPLE: c_uint = 0x01;
pub const VV6410_CROP_TO_QVGA: c_uint = 0x02;
pub const VV6410_CIF_LINELENGTH: c_int = 415;
extern "C" {
    pub fn vv6410_probe(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn vv6410_start(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn vv6410_init(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn vv6410_init_controls(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn vv6410_stop(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn vv6410_dump(sd: *mut sd) -> static int;
}
// V4L2 controls supported by the driver
extern "C" {
    pub fn vv6410_set_hflip(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn vv6410_set_vflip(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn vv6410_set_analog_gain(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
extern "C" {
    pub fn vv6410_set_exposure(gspca_dev: *mut gspca_dev, val: __s32) -> static int;
}
// FIXME (see if we can lower packet_size-s, needs testing, and also
// If NULL, only single value to write, stored in len
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv_init {
    pub addr: u16,
    pub data: u8,
}

// This reg is written twice. Some kind of reset?
// Setup registers
// Use shuffled read-out mode
// All modes to 1, FST, Fast QCK, Free running QCK, Free running LST, FST will qualify visible pixels
// Pre-clock generator divide off
// System registers
// Enable voltage doubler
// Power up audio, differential
