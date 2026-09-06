//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/common-debug.h
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
// struct ath_rx_stats - RX Statistics
// @rx_pkts_all:  No. of total frames received, including ones that
// may have had errors.
// @rx_bytes_all:  No. of total bytes received, including ones that
// may have had errors.
// @crc_err: No. of frames with incorrect CRC value
// @decrypt_crc_err: No. of frames whose CRC check failed after
// decryption process completed
// @phy_err: No. of frames whose reception failed because the PHY
// encountered an error
// @mic_err: No. of frames with incorrect TKIP MIC verification failure
// @pre_delim_crc_err: Pre-Frame delimiter CRC error detections
// @post_delim_crc_err: Post-Frame delimiter CRC error detections
// @decrypt_busy_err: Decryption interruptions counter
// @phy_err_stats: Individual PHY error statistics
// @rx_len_err:  No. of frames discarded due to bad length.
// @rx_oom_err:  No. of frames dropped due to OOM issues.
// @rx_rate_err:  No. of frames dropped due to rate errors.
// @rx_too_many_frags_err:  Frames dropped due to too-many-frags received.
// @rx_beacons:  No. of beacons received.
// @rx_frags:  No. of rx-fragements received.
// @rx_spectral: No of spectral packets received.
// @rx_spectral_sample_good: No. of good spectral samples
// @rx_spectral_sample_err: No. of good spectral samples
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_rx_stats {
    pub rx_pkts_all: u32,
    pub rx_bytes_all: u32,
    pub crc_err: u32,
    pub decrypt_crc_err: u32,
    pub phy_err: u32,
    pub mic_err: u32,
    pub pre_delim_crc_err: u32,
    pub post_delim_crc_err: u32,
    pub decrypt_busy_err: u32,
    pub phy_err_stats: [u32; ATH9K_PHYERR_MAX],
    pub rx_len_err: u32,
    pub rx_oom_err: u32,
    pub rx_rate_err: u32,
    pub rx_too_many_frags_err: u32,
    pub rx_beacons: u32,
    pub rx_frags: u32,
    pub rx_spectral: u32,
    pub rx_spectral_sample_good: u32,
    pub rx_spectral_sample_err: u32,
}

