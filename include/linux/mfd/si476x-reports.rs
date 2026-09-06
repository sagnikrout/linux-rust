//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/si476x-reports.h
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
// include/media/si476x-platform.h -- Definitions of the data formats
// returned by debugfs hooks
//
// Copyright (C) 2013 Andrey Smirnov
//
// Author: Andrey Smirnov <andrew.smirnov@gmail.com>
//
// struct si476x_rsq_status - structure containing received signal
// quality
// @multhint:   Multipath Detect High.
// true  - Indicatedes that the value is below
// FM_RSQ_MULTIPATH_HIGH_THRESHOLD
// false - Indicatedes that the value is above
// FM_RSQ_MULTIPATH_HIGH_THRESHOLD
// @multlint:   Multipath Detect Low.
// true  - Indicatedes that the value is below
// FM_RSQ_MULTIPATH_LOW_THRESHOLD
// false - Indicatedes that the value is above
// FM_RSQ_MULTIPATH_LOW_THRESHOLD
// @snrhint:    SNR Detect High.
// true  - Indicatedes that the value is below
// FM_RSQ_SNR_HIGH_THRESHOLD
// false - Indicatedes that the value is above
// FM_RSQ_SNR_HIGH_THRESHOLD
// @snrlint:    SNR Detect Low.
// true  - Indicatedes that the value is below
// FM_RSQ_SNR_LOW_THRESHOLD
// false - Indicatedes that the value is above
// FM_RSQ_SNR_LOW_THRESHOLD
// @rssihint:   RSSI Detect High.
// true  - Indicatedes that the value is below
// FM_RSQ_RSSI_HIGH_THRESHOLD
// false - Indicatedes that the value is above
// FM_RSQ_RSSI_HIGH_THRESHOLD
// @rssilint:   RSSI Detect Low.
// true  - Indicatedes that the value is below
// FM_RSQ_RSSI_LOW_THRESHOLD
// false - Indicatedes that the value is above
// FM_RSQ_RSSI_LOW_THRESHOLD
// @bltf:       Band Limit.
// Set if seek command hits the band limit or wrapped to
// the original frequency.
// @snr_ready:  SNR measurement in progress.
// @rssiready:  RSSI measurement in progress.
// @afcrl:      Set if FREQOFF >= MAX_TUNE_ERROR
// @valid:      Set if the channel is valid
// rssi < FM_VALID_RSSI_THRESHOLD
// snr  < FM_VALID_SNR_THRESHOLD
// tune_error < FM_VALID_MAX_TUNE_ERROR
// @readfreq:   Current tuned frequency.
// @freqoff:    Signed frequency offset.
// @rssi:       Received Signal Strength Indicator(dBuV).
// @snr:        RF SNR Indicator(dB).
// @lassi:
// @hassi:      Low/High side Adjacent(100 kHz) Channel Strength Indicator
// @mult:       Multipath indicator
// @dev:        Who knows? But values may vary.
// @readantcap: Antenna tuning capacity value.
// @assi:       Adjacent Channel(+/- 200kHz) Strength Indicator
// @usn:        Ultrasonic Noise Inticator in -DBFS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_rsq_status_report {
    pub multlint: __u8 multhint,,
    pub snrlint: __u8 snrhint,,
    pub rssilint: __u8 rssihint,,
    pub bltf: __u8,
    pub snr_ready: __u8,
    pub rssiready: __u8,
    pub injside: __u8,
    pub afcrl: __u8,
    pub valid: __u8,
    pub readfreq: __u16,
    pub freqoff: __s8,
    pub rssi: __s8,
    pub snr: __s8,
    pub issi: __s8,
    pub hassi: __s8 lassi,,
    pub mult: __s8,
    pub dev: __u8,
    pub readantcap: __u16,
    pub assi: __s8,
    pub usn: __s8,
    pub pilotdev: __u8,
    pub rdsdev: __u8,
    pub assidev: __u8,
    pub strongdev: __u8,
    pub rdspi: __u16,
    pub __packed: },
//
// si476x_acf_status_report - ACF report results
//
// @blend_int: If set, indicates that stereo separation has crossed
// below the blend threshold as set by FM_ACF_BLEND_THRESHOLD
// @hblend_int: If set, indicates that HiBlend cutoff frequency is
// lower than threshold as set by FM_ACF_HBLEND_THRESHOLD
// @hicut_int:  If set, indicates that HiCut cutoff frequency is lower
// than the threshold set by ACF_
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_acf_status_report {
    pub blend_int: __u8,
    pub hblend_int: __u8,
    pub hicut_int: __u8,
    pub chbw_int: __u8,
    pub softmute_int: __u8,
    pub smute: __u8,
    pub smattn: __u8,
    pub chbw: __u8,
    pub hicut: __u8,
    pub hiblend: __u8,
    pub pilot: __u8,
    pub stblend: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_fmagc {
    SI476X_FMAGC_10K_OHM	= 0,
    SI476X_FMAGC_800_OHM	= 1,
    SI476X_FMAGC_400_OHM	= 2,
    SI476X_FMAGC_200_OHM	= 4,
    SI476X_FMAGC_100_OHM	= 8,
    SI476X_FMAGC_50_OHM	= 16,
    SI476X_FMAGC_25_OHM	= 32,
    SI476X_FMAGC_12P5_OHM	= 64,
    SI476X_FMAGC_6P25_OHM	= 128,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_agc_status_report {
    pub mxhi: __u8,
    pub mxlo: __u8,
    pub lnahi: __u8,
    pub lnalo: __u8,
    pub fmagc1: __u8,
    pub fmagc2: __u8,
    pub pgagain: __u8,
    pub fmwblang: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_rds_blockcount_report {
    pub expected: __u16,
    pub received: __u16,
    pub uncorrectable: __u16,
    pub __packed: },
