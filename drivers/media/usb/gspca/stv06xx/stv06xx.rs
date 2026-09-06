//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/stv06xx/stv06xx.h
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

pub const STV_ISOC_ENDPOINT_ADDR: c_uint = 0x81;
pub const STV_R: c_uint = 0x0509;
pub const STV_REG23: c_uint = 0x0423;
// Control registers of the STV0600 ASIC
pub const STV_I2C_PARTNER: c_uint = 0x1420;
pub const STV_I2C_VAL_REG_VAL_PAIRS_MIN1: c_uint = 0x1421;
pub const STV_I2C_READ_WRITE_TOGGLE: c_uint = 0x1422;
pub const STV_I2C_FLUSH: c_uint = 0x1423;
pub const STV_I2C_SUCC_READ_REG_VALS: c_uint = 0x1424;
pub const STV_ISO_ENABLE: c_uint = 0x1440;
pub const STV_SCAN_RATE: c_uint = 0x1443;
pub const STV_LED_CTRL: c_uint = 0x1445;
pub const STV_STV0600_EMULATION: c_uint = 0x1446;
pub const STV_REG00: c_uint = 0x1500;
pub const STV_REG01: c_uint = 0x1501;
pub const STV_REG02: c_uint = 0x1502;
pub const STV_REG03: c_uint = 0x1503;
pub const STV_REG04: c_uint = 0x1504;
pub const STV_ISO_SIZE_L: c_uint = 0x15c1;
pub const STV_ISO_SIZE_H: c_uint = 0x15c2;
// Refers to the CIF 352x288 and QCIF 176x144
// 1: 288 lines, 2: 144 lines
pub const STV_Y_CTRL: c_uint = 0x15c3;
pub const STV_RESET: c_uint = 0x1620;
// 0xa: 352 columns, 0x6: 176 columns
pub const STV_X_CTRL: c_uint = 0x1680;
pub const STV06XX_URB_MSG_TIMEOUT: c_int = 5000;
pub const I2C_MAX_BYTES: c_int = 16;
pub const I2C_MAX_WORDS: c_int = 8;
pub const I2C_BUFFER_LENGTH: c_uint = 0x23;
pub const I2C_READ_CMD: c_int = 3;
pub const I2C_WRITE_CMD: c_int = 1;
pub const LED_ON: c_int = 1;
pub const LED_OFF: c_int = 0;
// STV06xx device descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd {
    pub gspca_dev: gspca_dev,
// A pointer to the currently connected sensor
    pub sensor: *const stv06xx_sensor,
// Sensor private data
    pub sensor_priv: *mut c_void,
// The first 4 lines produced by the stv6422 are no good, this keeps
    pub to_skip: c_int,
// Bridge / Camera type
    pub bridge: u8,
pub const BRIDGE_STV600: c_int = 0;
pub const BRIDGE_STV602: c_int = 1;
pub const BRIDGE_STV610: c_int = 2;

}

extern "C" {
    pub fn stv06xx_write_bridge(sd: *mut sd, address: u16, i2c_data: u16) -> c_int;
}
extern "C" {
    pub fn stv06xx_read_bridge(sd: *mut sd, address: u16, i2c_data: *mut u8) -> c_int;
}
extern "C" {
    pub fn stv06xx_write_sensor_bytes(sd: *mut sd, data: *const u8, len: u8) -> c_int;
}
extern "C" {
    pub fn stv06xx_write_sensor_words(sd: *mut sd, data: *const u16, len: u8) -> c_int;
}
extern "C" {
    pub fn stv06xx_read_sensor(sd: *mut sd, address: u8, value: *mut u16) -> c_int;
}
extern "C" {
    pub fn stv06xx_write_sensor(sd: *mut sd, address: u8, value: u16) -> c_int;
}
