//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/ngbevf/ngbevf_type.h
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
pub const NGBEVF_DEV_ID_EM_WX1860AL_W: c_uint = 0x0110;
pub const NGBEVF_DEV_ID_EM_WX1860A2: c_uint = 0x0111;
pub const NGBEVF_DEV_ID_EM_WX1860A2S: c_uint = 0x0112;
pub const NGBEVF_DEV_ID_EM_WX1860A4: c_uint = 0x0113;
pub const NGBEVF_DEV_ID_EM_WX1860A4S: c_uint = 0x0114;
pub const NGBEVF_DEV_ID_EM_WX1860AL2: c_uint = 0x0115;
pub const NGBEVF_DEV_ID_EM_WX1860AL2S: c_uint = 0x0116;
pub const NGBEVF_DEV_ID_EM_WX1860AL4: c_uint = 0x0117;
pub const NGBEVF_DEV_ID_EM_WX1860AL4S: c_uint = 0x0118;
pub const NGBEVF_DEV_ID_EM_WX1860NCSI: c_uint = 0x0119;
pub const NGBEVF_DEV_ID_EM_WX1860A1: c_uint = 0x011a;
pub const NGBEVF_DEV_ID_EM_WX1860AL1: c_uint = 0x011b;
pub const NGBEVF_MAX_MSIX_VECTORS: c_int = 1;
pub const NGBEVF_MAX_RX_QUEUES: c_int = 1;
pub const NGBEVF_MAX_TX_QUEUES: c_int = 1;
pub const NGBEVF_DEFAULT_TXD: c_int = 128;
pub const NGBEVF_DEFAULT_RXD: c_int = 128;
pub const NGBEVF_DEFAULT_TX_WORK: c_int = 256;
pub const NGBEVF_DEFAULT_RX_WORK: c_int = 256;
