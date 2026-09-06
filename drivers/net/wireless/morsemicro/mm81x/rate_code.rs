//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/rate_code.h
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
// Copyright (c) 2017-2026 Morse Micro
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dot11_bandwidth {
    DOT11_BANDWIDTH_1MHZ = 0,
    DOT11_BANDWIDTH_2MHZ = 1,
    DOT11_BANDWIDTH_4MHZ = 2,
    DOT11_BANDWIDTH_8MHZ = 3,
    DOT11_BANDWIDTH_16MHZ = 4,

    DOT11_MAX_BANDWIDTH = DOT11_BANDWIDTH_16MHZ,
    DOT11_INVALID_BANDWIDTH = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_rate_preamble {
// S1G LONG format (with SIG-A and SIG-B)
    MM81X_RATE_PREAMBLE_S1G_LONG = 0,
// This is the most common format used
    MM81X_RATE_PREAMBLE_S1G_SHORT = 1,
// S1G 1M format
    MM81X_RATE_PREAMBLE_S1G_1M = 2,

    MM81X_RATE_MAX_PREAMBLE = MM81X_RATE_PREAMBLE_S1G_1M,
    MM81X_RATE_INVALID_PREAMBLE = 7
}

pub type mm81x_rate_code_t = __le32;

extern "C" {
    pub fn le32_get_bits(_arg: rc, _arg: MM81X_RATECODE_MCS_INDEX) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: rc, _arg: MM81X_RATECODE_NSS_INDEX) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: rc, _arg: MM81X_RATECODE_RTS_FLAG) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: rc, _arg: MM81X_RATECODE_SHORT_GI_FLAG) -> return;
}

extern "C" {
    pub fn MM81X_RATECODE_INIT(_arg: bw_index, _arg: nss_index, _arg: mcs_index, _arg: preamble) -> return;
}
// rc = (*rc & cpu_to_le32(~MM81X_RATECODE_PREAMBLE)) |
// rc = (*rc & cpu_to_le32(~MM81X_RATECODE_MCS_INDEX)) |
// rc = (*rc & cpu_to_le32(~MM81X_RATECODE_NSS_INDEX)) |
// rc = (*rc & cpu_to_le32(~MM81X_RATECODE_BW_INDEX)) |
// rc = (*rc & cpu_to_le32(~MM81X_RATECODE_DUP_BW_INDEX)) |
// rc |= cpu_to_le32(MM81X_RATECODE_RTS_FLAG);
// rc |= cpu_to_le32(MM81X_RATECODE_SHORT_GI_FLAG);
