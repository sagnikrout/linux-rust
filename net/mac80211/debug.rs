//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/debug.h
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
// Portions
// Copyright (C) 2022 - 2025 Intel Corporation
//

pub const MAC80211_OCB_DEBUG: c_int = 1;

pub const MAC80211_OCB_DEBUG: c_int = 0;

pub const MAC80211_IBSS_DEBUG: c_int = 1;

pub const MAC80211_IBSS_DEBUG: c_int = 0;

pub const MAC80211_PS_DEBUG: c_int = 1;

pub const MAC80211_PS_DEBUG: c_int = 0;

pub const MAC80211_HT_DEBUG: c_int = 1;

pub const MAC80211_HT_DEBUG: c_int = 0;

pub const MAC80211_MPL_DEBUG: c_int = 1;

pub const MAC80211_MPL_DEBUG: c_int = 0;

pub const MAC80211_MPATH_DEBUG: c_int = 1;

pub const MAC80211_MPATH_DEBUG: c_int = 0;

pub const MAC80211_MHWMP_DEBUG: c_int = 1;

pub const MAC80211_MHWMP_DEBUG: c_int = 0;

pub const MAC80211_MESH_SYNC_DEBUG: c_int = 1;

pub const MAC80211_MESH_SYNC_DEBUG: c_int = 0;

pub const MAC80211_MESH_CSA_DEBUG: c_int = 1;

pub const MAC80211_MESH_CSA_DEBUG: c_int = 0;

pub const MAC80211_MESH_PS_DEBUG: c_int = 1;

pub const MAC80211_MESH_PS_DEBUG: c_int = 0;

pub const MAC80211_TDLS_DEBUG: c_int = 1;

pub const MAC80211_TDLS_DEBUG: c_int = 0;

pub const MAC80211_STA_DEBUG: c_int = 1;

pub const MAC80211_STA_DEBUG: c_int = 0;

pub const MAC80211_MLME_DEBUG: c_int = 1;

pub const MAC80211_MLME_DEBUG: c_int = 0;

extern "C" {
    pub fn __sdata_info(fmt: *const c_char, __printf(1: ...), _arg: 2);
}
extern "C" {
    pub fn __sdata_dbg(print: bool, fmt: *const c_char, __printf(2: ...), _arg: 3);
}
extern "C" {
    pub fn __sdata_err(fmt: *const c_char, __printf(1: ...), _arg: 2);
}

