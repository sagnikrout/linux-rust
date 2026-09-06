//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/spectral_common.h
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
pub const SPECTRAL_HT20_NUM_BINS: c_int = 56;
pub const SPECTRAL_HT20_40_NUM_BINS: c_int = 128;
// TODO: could possibly be 512, but no samples this large
// could be acquired so far.
//
pub const SPECTRAL_ATH10K_MAX_NUM_BINS: c_int = 256;
// FFT sample format given to userspace via debugfs.
//
// Please keep the type/length at the front position and change
// other fields after adding another sample type
//
// TODO: this might need rework when switching to nl80211-based
// interface.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_fft_sample_type {
    ATH_FFT_SAMPLE_HT20 = 1,
    ATH_FFT_SAMPLE_HT20_40,
    ATH_FFT_SAMPLE_ATH10K,
    ATH_FFT_SAMPLE_ATH11K
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fft_sample_tlv {
    pub /: *mut *mut u8 type; / see ath_fft_sample,
    pub length: __be16,
// type dependent data follows
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fft_sample_ht20 {
    pub tlv: fft_sample_tlv,
    pub max_exp: u8,
    pub freq: __be16,
    pub rssi: i8,
    pub noise: i8,
    pub max_magnitude: __be16,
    pub max_index: u8,
    pub bitmap_weight: u8,
    pub tsf: __be64,
    pub data: [u8; SPECTRAL_HT20_NUM_BINS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fft_sample_ht20_40 {
    pub tlv: fft_sample_tlv,
    pub channel_type: u8,
    pub freq: __be16,
    pub lower_rssi: i8,
    pub upper_rssi: i8,
    pub tsf: __be64,
    pub lower_noise: i8,
    pub upper_noise: i8,
    pub lower_max_magnitude: __be16,
    pub upper_max_magnitude: __be16,
    pub lower_max_index: u8,
    pub upper_max_index: u8,
    pub lower_bitmap_weight: u8,
    pub upper_bitmap_weight: u8,
    pub max_exp: u8,
    pub data: [u8; SPECTRAL_HT20_40_NUM_BINS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fft_sample_ath10k {
    pub tlv: fft_sample_tlv,
    pub chan_width_mhz: u8,
    pub freq1: __be16,
    pub freq2: __be16,
    pub noise: __be16,
    pub max_magnitude: __be16,
    pub total_gain_db: __be16,
    pub base_pwr_db: __be16,
    pub tsf: __be64,
    pub max_index: i8,
    pub rssi: u8,
    pub relpwr_db: u8,
    pub avgpwr_db: u8,
    pub max_exp: u8,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fft_sample_ath11k {
    pub tlv: fft_sample_tlv,
    pub chan_width_mhz: u8,
    pub max_index: i8,
    pub max_exp: u8,
    pub freq1: __be16,
    pub freq2: __be16,
    pub max_magnitude: __be16,
    pub rssi: __be16,
    pub tsf: __be32,
    pub noise: __be32,
    pub data: [u8; ],
    pub __packed: },
