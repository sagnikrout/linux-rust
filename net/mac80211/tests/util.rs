//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/tests/util.h
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
// Utilities for mac80211 unit testing
//
// Copyright (C) 2024 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t_sdata {
    pub sdata: *mut ieee80211_sub_if_data,
    pub wiphy: *mut wiphy,
    pub local: ieee80211_local,
    pub ctx: *mut c_void,
    pub band_2ghz: ieee80211_supported_band,
    pub band_5ghz: ieee80211_supported_band,
}

extern "C" {
    pub fn t_sdata_init(resource: *mut kunit_resource, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn t_sdata_exit(resource: *mut kunit_resource);
}
