//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl/hw_atl_b0_internal.h
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
// aQuantia Corporation Network Driver
// Copyright (C) 2014-2019 aQuantia Corporation. All rights reserved
//
// File hw_atl_b0_internal.h: Definition of Atlantic B0 chip specific
// constants.
//

// UCAST/MCAST filters
pub const HW_ATL_B0_UCAST_FILTERS_MAX: c_int = 38;
pub const HW_ATL_B0_MCAST_FILTERS_MAX: c_int = 8;
// interrupts

pub const HW_ATL_B0_MPI_CONTROL_ADR: c_uint = 0x0368U;
pub const HW_ATL_B0_MPI_STATE_ADR: c_uint = 0x036CU;
pub const HW_ATL_B0_MPI_SPEED_MSK: c_uint = 0xFFFFU;

pub const HW_ATL_B0_TCRSS_4_8: c_int = 1;

// (256k -1(max pay_len) - 54(header))

// (256k -1(max pay_len) - 74(header))

pub const HW_ATL_B0_CHIP_REVISION_B0: c_uint = 0xA0U;
pub const HW_ATL_B0_CHIP_REVISION_UNKNOWN: c_uint = 0xFFU;
pub const HW_ATL_B0_FW_SEMA_RAM: c_uint = 0x2U;

pub const HW_ATL_B0_FW_VER_EXPECTED: c_uint = 0x01050006U;
pub const HW_ATL_INTR_MODER_MAX: c_uint = 0x1FF;
pub const HW_ATL_INTR_MODER_MIN: c_uint = 0xFF;

pub const HW_ATL_RSS_DISABLED: c_uint = 0x00000000U;
pub const HW_ATL_RSS_ENABLED_8TCS_2INDEX_BITS: c_uint = 0xA2222222U;
pub const HW_ATL_RSS_ENABLED_4TCS_3INDEX_BITS: c_uint = 0x80003333U;
// HW layer capabilities
