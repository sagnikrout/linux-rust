//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/fwio.h
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
// Firmware API for mac80211 ST-Ericsson CW1200 drivers
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//
// Based on:
// ST-Ericsson UMAC CW1200 driver which is
// Copyright (c) 2010, ST-Ericsson
// Author: Ajitpal Singh <ajitpal.singh@stericsson.com>
//

// Macro flag: #define FWIO_H_INCLUDED

extern "C" {
    pub fn cw1200_load_firmware(priv: *mut cw1200_common) -> c_int;
}
// SDD definitions
pub const SDD_PTA_CFG_ELT_ID: c_uint = 0xEB;
pub const SDD_REFERENCE_FREQUENCY_ELT_ID: c_uint = 0xc5;
extern "C" {
    pub fn cw1200_dpll_from_clk(clk: u16) -> u32;
}
