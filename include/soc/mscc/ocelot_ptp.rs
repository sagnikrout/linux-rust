//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/mscc/ocelot_ptp.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Microsemi Ocelot Switch driver
//
// License: Dual MIT/GPL
// Copyright (c) 2017 Microsemi Corporation
// Copyright 2020 NXP
//

pub const OCELOT_MAX_PTP_ID: c_int = 63;
pub const OCELOT_PTP_FIFO_SIZE: c_int = 128;
pub const PTP_PIN_CFG_RSZ: c_uint = 0x20;

extern "C" {
    pub fn ocelot_ptp_gettime64(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> c_int;
}
extern "C" {
    pub fn ocelot_ptp_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int;
}
extern "C" {
    pub fn ocelot_ptp_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int;
}
extern "C" {
    pub fn ocelot_deinit_timestamp(ocelot: *mut ocelot) -> c_int;
}
