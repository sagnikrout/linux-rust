//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtc/ds1307.h
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


//
// ds1307.h - platform_data for the ds1307 (and variants) rtc driver
// (C) Copyright 2012 by Wolfram Sang, Pengutronix e.K.
// same license as the driver
//

pub const DS1307_TRICKLE_CHARGER_250_OHM: c_uint = 0x01;
pub const DS1307_TRICKLE_CHARGER_2K_OHM: c_uint = 0x02;
pub const DS1307_TRICKLE_CHARGER_4K_OHM: c_uint = 0x03;
pub const DS1307_TRICKLE_CHARGER_NO_DIODE: c_uint = 0x04;
pub const DS1307_TRICKLE_CHARGER_DIODE: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds1307_platform_data {
    pub trickle_charger_setup: u8,
}
