//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/trace.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2016 Qualcomm Atheros, Inc.
//

// In some rare cases (e.g. fcs error) device reports frame buffer
// shorter than what frame header implies (e.g. len = 0). The buffer
// can still be accessed so do a simple min() to guarantee caller
// doesn't get value greater than len.
//
extern "C" {
    pub fn min_t(_arg: u32, _arg: len, _arg: ieee80211_hdrlen(hdr->frame_control)) -> return;
}

// create empty functions when tracing is disabled

// Macro flag: #define DECLARE_EVENT_CLASS(...)

pub const ATH10K_MSG_MAX: c_int = 400;

// we don't want to use include/trace/events

// This part must be outside protection
