//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/hirschmann/hellcreek_hwtstamp.h
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
// DSA driver for:
// Hirschmann Hellcreek TSN switch.
//
// Copyright (C) 2019,2020 Hochschule Offenburg
// Copyright (C) 2019,2020 Linutronix GmbH
// Authors: Kurt Kanzenbach <kurt@linutronix.de>
// Kamil Alkhouri <kamil.alkhouri@hs-offenburg.de>
//

// Timestamp Register

// TX_TSTAMP_TIMEOUT: This limits the time spent polling for a TX
// timestamp. When working properly, hardware will produce a timestamp
// within 1ms. Software may enounter delays, so the timeout is set
// accordingly.
//

extern "C" {
    pub fn hellcreek_hwtstamp_work(ptp: *mut ptp_clock_info) -> c_long;
}
extern "C" {
    pub fn hellcreek_hwtstamp_setup(chip: *mut hellcreek) -> c_int;
}
extern "C" {
    pub fn hellcreek_hwtstamp_free(chip: *mut hellcreek);
}
