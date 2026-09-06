//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl2/hw_atl2.h
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
// Atlantic Network Driver
// Copyright (C) 2020 Marvell International Ltd.
//

pub const HW_ATL2_RX_TS_SIZE: c_int = 8;
pub const HW_ATL2_PTP_OFFSET_INGRESS_100: c_int = 768;
pub const HW_ATL2_PTP_OFFSET_EGRESS_100: c_int = 336;
pub const HW_ATL2_PTP_OFFSET_INGRESS_1000: c_int = 510;
pub const HW_ATL2_PTP_OFFSET_EGRESS_1000: c_int = 105;
pub const HW_ATL2_PTP_OFFSET_INGRESS_2500: c_int = 2447;
pub const HW_ATL2_PTP_OFFSET_EGRESS_2500: c_int = 634;
pub const HW_ATL2_PTP_OFFSET_INGRESS_5000: c_int = 1426;
pub const HW_ATL2_PTP_OFFSET_EGRESS_5000: c_int = 361;
pub const HW_ATL2_PTP_OFFSET_INGRESS_10000: c_int = 997;
pub const HW_ATL2_PTP_OFFSET_EGRESS_10000: c_int = 203;
