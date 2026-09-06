//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/m5602/m5602_sensor.h
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
// USB Driver for ALi m5602 based webcams
//
// Copyright (C) 2008 Erik Andrén
// Copyright (C) 2007 Ilyes Gouta. Based on the m5603x Linux Driver Project.
// Copyright (C) 2005 m5603x Linux Driver Project <m5602@x3ng.com.br>
//
// Portions of code to USB interface and ALi driver software,
// Copyright (c) 2006 Willem Duinker
// v4l2 interface modeled after the V4L2 driver
// for SN9C10x PC Camera Controllers
//

// Enumerates all supported sensors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sensors {
    OV9650_SENSOR	= 1,
    S5K83A_SENSOR	= 2,
    S5K4AA_SENSOR	= 3,
    MT9M111_SENSOR	= 4,
    PO1030_SENSOR	= 5,
    OV7660_SENSOR   = 6,
}

// Enumerates all possible instruction types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum instruction {
    BRIDGE,
    SENSOR,
    SENSOR_LONG
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m5602_sensor {
// Defines the name of a sensor
    pub name: [c_char; 32],
// What i2c address the sensor is connected to
    pub i2c_slave_id: u8,
// Width of each i2c register (in bytes)
    pub i2c_regW: u8,
// Probes if the sensor is connected
    pub sd): *mut *mut int (probe)(struct sd,
// Performs a initialization sequence
    pub sd): *mut *mut int (init)(struct sd,
// Controls initialization, maybe NULL
    pub sd): *mut *mut int (init_controls)(struct sd,
// Executed when the camera starts to send data
    pub sd): *mut *mut int (start)(struct sd,
// Executed when the camera ends to send data
    pub sd): *mut *mut int (stop)(struct sd,
// Executed when the device is disconnected
    pub sd): *mut *mut void (disconnect)(struct sd,
}
