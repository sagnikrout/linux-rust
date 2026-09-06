//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/dfs_debug.h
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


//
// Copyright (c) 2008-2011 Atheros Communications Inc.
// Copyright (c) 2011 Neratec Solutions AG
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// struct ath_dfs_stats - DFS Statistics per wiphy
// @pulses_total:     pulses reported by HW
// @pulses_no_dfs:    pulses wrongly reported as DFS
// @pulses_detected:  pulses detected so far
// @datalen_discards: pulses discarded due to invalid datalen
// @rssi_discards:    pulses discarded due to invalid RSSI
// @bwinfo_discards:  pulses discarded due to invalid BW info
// @pri_phy_errors:   pulses reported for primary channel
// @ext_phy_errors:   pulses reported for extension channel
// @dc_phy_errors:    pulses reported for primary + extension channel
// @pulses_processed: pulses forwarded to detector
// @radar_detected:   radars detected
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_dfs_stats {
// pulse stats
    pub pulses_total: u32,
    pub pulses_no_dfs: u32,
    pub pulses_detected: u32,
    pub datalen_discards: u32,
    pub rssi_discards: u32,
    pub bwinfo_discards: u32,
    pub pri_phy_errors: u32,
    pub ext_phy_errors: u32,
    pub dc_phy_errors: u32,
// pattern detection stats
    pub pulses_processed: u32,
    pub radar_detected: u32,
}

extern "C" {
    pub fn ath9k_dfs_init_debug(sc: *mut ath_softc);
}

