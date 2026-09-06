//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/channel.h
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
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// conversion for phy txpwr calculations that use .25 dB units
pub const BRCMS_TXPWR_DB_FACTOR: c_int = 4;
// bits for locale_info flags
pub const BRCMS_PEAK_CONDUCTED: c_uint = 0x00	/* Peak for locals */;
pub const BRCMS_EIRP: c_uint = 0x01	/* Flag for EIRP */;
pub const BRCMS_DFS_TPC: c_uint = 0x02	/* Flag for DFS TPC */;
pub const BRCMS_NO_OFDM: c_uint = 0x04	/* Flag for No OFDM */;
pub const BRCMS_NO_40MHZ: c_uint = 0x08	/* Flag for No MIMO 40MHz */;
pub const BRCMS_NO_MIMO: c_uint = 0x10	/* Flag for No MIMO, 20 or 40 MHz */;
pub const BRCMS_RADAR_TYPE_EU: c_uint = 0x20	/* Flag for EU */;

extern "C" {
    pub fn brcms_c_channel_mgr_detach(wlc_cm: *mut brcms_cm_info);
}
extern "C" {
    pub fn brcms_c_valid_chanspec_db(wlc_cm: *mut brcms_cm_info, chspec: u16) -> bool;
}
extern "C" {
    pub fn brcms_c_regd_init(wlc: *mut brcms_c_info);
}
