//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/hirschmann/hellcreek_ptp.h
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

// Every jump in time is 7 ns

// Correct offset at every clock cycle
pub const MIN_CLK_CYCLES_BETWEEN_STEPS: c_int = 0;
// Maximum available slow offset resources

// four times a second overflow check

// PTP Register

pub const PR_SETTINGS_C_TS_SRC_TK_SHIFT: c_int = 8;

extern "C" {
    pub fn hellcreek_ptp_setup(hellcreek: *mut hellcreek) -> c_int;
}
extern "C" {
    pub fn hellcreek_ptp_free(hellcreek: *mut hellcreek);
}
extern "C" {
    pub fn hellcreek_ptp_read(hellcreek: *mut hellcreek, offset: c_uint) -> u16;
}
extern "C" {
    pub fn hellcreek_ptp_gettime_seconds(hellcreek: *mut hellcreek, ns: u64) -> u64;
}

