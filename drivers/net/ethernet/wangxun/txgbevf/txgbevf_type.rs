//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/txgbevf/txgbevf_type.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2015 - 2025 Beijing WangXun Technology Co., Ltd.
// Device IDs
pub const TXGBEVF_DEV_ID_SP1000: c_uint = 0x1000;
pub const TXGBEVF_DEV_ID_WX1820: c_uint = 0x2000;
pub const TXGBEVF_DEV_ID_AML500F: c_uint = 0x500F;
pub const TXGBEVF_DEV_ID_AML510F: c_uint = 0x510F;
pub const TXGBEVF_DEV_ID_AML5024: c_uint = 0x5024;
pub const TXGBEVF_DEV_ID_AML5124: c_uint = 0x5124;
pub const TXGBEVF_DEV_ID_AML503F: c_uint = 0x503f;
pub const TXGBEVF_DEV_ID_AML513F: c_uint = 0x513f;
pub const TXGBEVF_MAX_MSIX_VECTORS: c_int = 2;
pub const TXGBEVF_MAX_RSS_NUM: c_int = 4;
pub const TXGBEVF_MAX_RX_QUEUES: c_int = 4;
pub const TXGBEVF_MAX_TX_QUEUES: c_int = 4;
pub const TXGBEVF_DEFAULT_TXD: c_int = 128;
pub const TXGBEVF_DEFAULT_RXD: c_int = 128;
pub const TXGBEVF_DEFAULT_TX_WORK: c_int = 256;
pub const TXGBEVF_DEFAULT_RX_WORK: c_int = 256;
