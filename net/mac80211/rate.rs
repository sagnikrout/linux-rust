//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/rate.h
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
// Copyright 2002-2005, Instant802 Networks, Inc.
// Copyright 2005, Devicescape Software, Inc.
// Copyright (c) 2006 Jiri Benc <jbenc@suse.cz>
// Copyright (C) 2022, 2024 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rate_control_ref {
    pub ops: *const rate_control_ops,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn rate_control_rate_init(link_sta: *mut link_sta_info);
}
extern "C" {
    pub fn rate_control_rate_init_all_links(sta: *mut sta_info);
}

extern "C" {
    pub fn ieee80211_check_rate_mask(link: *mut ieee80211_link_data);
}
// Get a reference to the rate control algorithm. If `name' is NULL, get the
// first available algorithm.
extern "C" {
    pub fn rate_control_deinitialize(local: *mut ieee80211_local);
}
// Rate control algorithms

extern "C" {
    pub fn rc80211_minstrel_init() -> c_int;
}
extern "C" {
    pub fn rc80211_minstrel_exit();
}

