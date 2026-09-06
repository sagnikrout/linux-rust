//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/debugfs_netdev.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Portions:
// Copyright (C) 2023 Intel Corporation
//
// routines exported for debugfs handling

extern "C" {
    pub fn ieee80211_debugfs_remove_netdev(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_debugfs_rename_netdev(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_link_debugfs_add(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_link_debugfs_remove(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_link_debugfs_drv_add(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_link_debugfs_drv_remove(link: *mut ieee80211_link_data);
}

