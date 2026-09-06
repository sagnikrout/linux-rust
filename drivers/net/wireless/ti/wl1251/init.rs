//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/init.h
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
// This file is part of wl1251
//
// Copyright (C) 2009 Nokia Corporation
//

// best effort/legacy
// background
// video
// voice
// broadcast dummy access category
// following are defult values for the IE fields
pub const CWMIN_BK: c_int = 15;
pub const CWMIN_BE: c_int = 15;
pub const CWMIN_VI: c_int = 7;
pub const CWMIN_VO: c_int = 3;
pub const CWMAX_BK: c_int = 1023;
pub const CWMAX_BE: c_int = 63;
pub const CWMAX_VI: c_int = 15;
pub const CWMAX_VO: c_int = 7;
// slot number setting to start transmission at PIFS interval
pub const AIFS_PIFS: c_int = 1;
//
// slot number setting to start transmission at DIFS interval - normal DCF
// access
//
pub const AIFS_DIFS: c_int = 2;
pub const AIFSN_BK: c_int = 7;
pub const AIFSN_BE: c_int = 3;

pub const TXOP_BK: c_int = 0;
pub const TXOP_BE: c_int = 0;
pub const TXOP_VI: c_int = 3008;
pub const TXOP_VO: c_int = 1504;
extern "C" {
    pub fn wl1251_hw_init_hwenc_config(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_templates_config(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_rx_config(wl: *mut wl1251, config: u32, filter: u32) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_phy_config(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_beacon_filter(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_pta(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_energy_detection(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_beacon_broadcast(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_power_auth(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init_mem_config(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_hw_init(wl: *mut wl1251) -> c_int;
}
