//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/rate.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_mcs_info {
// phy rate in kbps [20Mhz]
    pub phy_rate_20: u32,
// phy rate in kbps [40Mhz]
    pub phy_rate_40: u32,
// phy rate in kbps [20Mhz] with SGI
    pub phy_rate_20_sgi: u32,
// phy rate in kbps [40Mhz] with SGI
    pub phy_rate_40_sgi: u32,
// phy ctl byte 3, code rate, modulation type, # of streams
    pub tx_phy_ctl3: u8,
// matching legacy ofdm rate in 500bkps
    pub leg_ofdm: u8,
}

pub const MCS_TXS_MASK: c_uint = 0xc0	/* num tx streams - 1 bit mask */;

// returns num tx streams - 1
// Macro to use the rate_info table
pub const BRCMS_RATE_MASK_FULL: c_uint = 0xff /* Rate value mask with basic rate flag */;
//
// rate spec : holds rate and mode specific information required to generate a
// tx frame. Legacy CCK and OFDM information is held in the same manner as was
// done in the past (in the lower byte) the upper 3 bytes primarily hold MIMO
// specific information
//
// rate spec bit fields
// Either 500Kbps units or MIMO MCS idx
pub const RSPEC_RATE_MASK: c_uint = 0x0000007F;
// mimo MCS is stored in RSPEC_RATE_MASK
pub const RSPEC_MIMORATE: c_uint = 0x08000000;
// mimo bw mask
pub const RSPEC_BW_MASK: c_uint = 0x00000700;
// mimo bw shift
pub const RSPEC_BW_SHIFT: c_int = 8;
// mimo Space/Time/Frequency mode mask
pub const RSPEC_STF_MASK: c_uint = 0x00003800;
// mimo Space/Time/Frequency mode shift
pub const RSPEC_STF_SHIFT: c_int = 11;
// mimo coding type mask
pub const RSPEC_CT_MASK: c_uint = 0x0000C000;
// mimo coding type shift
pub const RSPEC_CT_SHIFT: c_int = 14;
// mimo num STC streams per PLCP defn.
pub const RSPEC_STC_MASK: c_uint = 0x00300000;
// mimo num STC streams per PLCP defn.
pub const RSPEC_STC_SHIFT: c_int = 20;
// mimo bit indicates adv coding in use
pub const RSPEC_LDPC_CODING: c_uint = 0x00400000;
// mimo bit indicates short GI in use
pub const RSPEC_SHORT_GI: c_uint = 0x00800000;
// bit indicates override both rate & mode
pub const RSPEC_OVERRIDE: c_uint = 0x80000000;
// bit indicates override rate only
pub const RSPEC_OVERRIDE_MCS_ONLY: c_uint = 0x40000000;
// Convert encoded rate value in plcp header to numerical rates in 500 KHz
// increments
// Rates specified in brcms_c_rateset_filter()
pub const BRCMS_RATES_CCK_OFDM: c_int = 0;
pub const BRCMS_RATES_CCK: c_int = 1;
pub const BRCMS_RATES_OFDM: c_int = 2;
// sanitize, and sort a rateset with the basic bit(s) preserved, validate
// rateset
// copy rateset src to dst as-is (no masking or sorting)
// would be nice to have these documented ...
extern "C" {
    pub fn brcms_c_compute_rspec(rxh: *mut d11rxhdr, plcp: *mut u8) -> u32;
}
extern "C" {
    pub fn brcms_c_rate_legacy_phyctl(rate: c_uint) -> i16;
}
extern "C" {
    pub fn brcms_c_rateset_mcs_upd(rs: *mut brcms_c_rateset, txstreams: u8);
}
extern "C" {
    pub fn brcms_c_rateset_mcs_clear(rateset: *mut brcms_c_rateset);
}
extern "C" {
    pub fn brcms_c_rateset_mcs_build(rateset: *mut brcms_c_rateset, txstreams: u8);
}
extern "C" {
    pub fn brcms_c_rateset_bw_mcs_filter(rateset: *mut brcms_c_rateset, bw: u8);
}
