//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/common-spectral.h
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
// Copyright (c) 2013 Qualcomm Atheros, Inc.
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

// enum spectral_mode:
//
// @SPECTRAL_DISABLED: spectral mode is disabled
// @SPECTRAL_BACKGROUND: hardware sends samples when it is not busy with
// something else.
// @SPECTRAL_MANUAL: spectral scan is enabled, triggering for samples
// is performed manually.
// @SPECTRAL_CHANSCAN: Like manual, but also triggered when changing channels
// during a channel scan.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spectral_mode {
    SPECTRAL_DISABLED = 0,
    SPECTRAL_BACKGROUND,
    SPECTRAL_MANUAL,
    SPECTRAL_CHANSCAN,
}

pub const SPECTRAL_SCAN_BITMASK: c_uint = 0x10;
// Radar info packet format, used for DFS and spectral formats.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_radar_info {
    pub pulse_length_pri: u8,
    pub pulse_length_ext: u8,
    pub pulse_bw_info: u8,
    pub __packed: },
// The HT20 spectral data has 4 bytes of additional information at it's end.
//
// [7:0]: all bins {max_magnitude[1:0], bitmap_weight[5:0]}
// [7:0]: all bins  max_magnitude[9:2]
// [7:0]: all bins {max_index[5:0], max_magnitude[11:10]}
// [3:0]: max_exp (shift amount to size max bin to 8-bit unsigned)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_ht20_mag_info {
    pub all_bins: [u8; 3],
    pub max_exp: u8,
    pub __packed: },
// WARNING: don't actually use this struct! MAC may vary the amount of
// data by -1/+2. This struct is for reference only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_ht20_fft_packet {
    pub data: [u8; SPECTRAL_HT20_NUM_BINS],
    pub mag_info: ath_ht20_mag_info,
    pub radar_info: ath_radar_info,
    pub __packed: },

// Dynamic 20/40 mode:
//
// [7:0]: lower bins {max_magnitude[1:0], bitmap_weight[5:0]}
// [7:0]: lower bins  max_magnitude[9:2]
// [7:0]: lower bins {max_index[5:0], max_magnitude[11:10]}
// [7:0]: upper bins {max_magnitude[1:0], bitmap_weight[5:0]}
// [7:0]: upper bins  max_magnitude[9:2]
// [7:0]: upper bins {max_index[5:0], max_magnitude[11:10]}
// [3:0]: max_exp (shift amount to size max bin to 8-bit unsigned)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_ht20_40_mag_info {
    pub lower_bins: [u8; 3],
    pub upper_bins: [u8; 3],
    pub max_exp: u8,
    pub __packed: },
// WARNING: don't actually use this struct! MAC may vary the amount of
// data. This struct is for reference only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_ht20_40_fft_packet {
    pub data: [u8; SPECTRAL_HT20_40_NUM_BINS],
    pub mag_info: ath_ht20_40_mag_info,
    pub radar_info: ath_radar_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_spec_scan_priv {
    pub ah: *mut ath_hw,
// relay(fs) channel for spectral scan
    pub rfs_chan_spec_scan: *mut rchan,
    pub spectral_mode: spectral_mode,
    pub spec_config: ath_spec_scan,
}

// grabs the max magnitude from the all/upper/lower bins
// return the max magnitude from the all/upper/lower bins
// It's a 5 bit signed int, remove its sign and use one's
// complement interpretation to add the sign back to the 8
// bit int
//
// Bring the zero point to the beginning
// instead of the middle so that we can use
// it for array lookup and that we don't deal
// with negative values later
//
// Sanity check to make sure index is within bounds
// positive values and zero are starting at the beginning
// of the data field.
//
extern "C" {
    pub fn spectral_max_index(_arg: bins, _arg: SPECTRAL_HT20_NUM_BINS) -> return;
}
// return the bitmap weight from the all/upper/lower bins

extern "C" {
    pub fn ath9k_cmn_spectral_init_debug(spec_priv: *mut ath_spec_scan_priv, debugfs_phy: *mut dentry);
}
extern "C" {
    pub fn ath9k_cmn_spectral_deinit_debug(spec_priv: *mut ath_spec_scan_priv);
}

