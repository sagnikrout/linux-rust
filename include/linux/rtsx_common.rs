//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtsx_common.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Driver for Realtek driver-based card reader
//
// Copyright(c) 2009-2013 Realtek Semiconductor Corp. All rights reserved.
//
// Author:
// Wei WANG <wei_wang@realsil.com.cn>
//

pub const RTSX_SSC_DEPTH_4M: c_uint = 0x01;
pub const RTSX_SSC_DEPTH_2M: c_uint = 0x02;
pub const RTSX_SSC_DEPTH_1M: c_uint = 0x03;
pub const RTSX_SSC_DEPTH_500K: c_uint = 0x04;
pub const RTSX_SSC_DEPTH_250K: c_uint = 0x05;
pub const RTSX_SD_CARD: c_int = 0;
pub const RTSX_MS_CARD: c_int = 1;
pub const CLK_TO_DIV_N: c_int = 0;
pub const DIV_N_TO_CLK: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtsx_slot {
    pub p_dev: *mut platform_device,
    pub p_dev): *mut *mut void (card_event)(struct platform_device,
}
