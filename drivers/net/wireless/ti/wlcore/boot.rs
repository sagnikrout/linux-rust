//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/boot.h
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
// This file is part of wl1271
//
// Copyright (C) 2008-2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

extern "C" {
    pub fn wlcore_boot_upload_firmware(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wlcore_boot_upload_nvs(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wlcore_boot_run_firmware(wl: *mut wl1271) -> c_int;
}
pub const WL1271_NO_SUBBANDS: c_int = 8;
pub const WL1271_NO_POWER_LEVELS: c_int = 4;
pub const WL1271_FW_VERSION_MAX_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_static_data {
    pub mac_address: [u8; ETH_ALEN],
    pub padding: [u8; 2],
    pub fw_version: [u8; WL1271_FW_VERSION_MAX_LEN],
    pub hw_version: u32,
    pub tx_power_table: [u8; WL1271_NO_SUBBANDS][WL1271_NO_POWER_LEVELS],
    pub priv: [u8; ],
}

// number of times we try to read the INIT interrupt
pub const INIT_LOOP: c_int = 20000;
// delay between retries
pub const INIT_LOOP_DELAY: c_int = 50;
pub const WU_COUNTER_PAUSE_VAL: c_uint = 0x3FF;
pub const WELP_ARM_COMMAND_VAL: c_uint = 0x4;
