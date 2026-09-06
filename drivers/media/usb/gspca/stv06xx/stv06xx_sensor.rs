//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/stv06xx/stv06xx_sensor.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv06xx_sensor {
// Defines the name of a sensor
    pub name: [c_char; 32],
// Sensor i2c address
    pub i2c_addr: u8,
// Flush value
    pub i2c_flush: u8,
// length of an i2c word
    pub i2c_len: u8,
// Isoc packet size (per mode)
    pub min_packet_size: [c_int; 4],
    pub max_packet_size: [c_int; 4],
// Probes if the sensor is connected
    pub sd): *mut *mut int (probe)(struct sd,
// Performs a initialization sequence
    pub sd): *mut *mut int (init)(struct sd,
// Initializes the controls
    pub sd): *mut *mut int (init_controls)(struct sd,
// Reads a sensor register
    pub len): *const *const u8 i2c_data, u8,
// Writes to a sensor register
    pub len): *const *const u8 i2c_data, u8,
// Instructs the sensor to start streaming
    pub sd): *mut *mut int (start)(struct sd,
// Instructs the sensor to stop streaming
    pub sd): *mut *mut int (stop)(struct sd,
// Instructs the sensor to dump all its contents
    pub sd): *mut *mut int (dump)(struct sd,
}
