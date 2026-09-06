//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/raspberrypi/vchiq_cfg.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2010-2014 Broadcom. All rights reserved.

// The version of VCHIQ - change with any non-trivial change
pub const VCHIQ_VERSION: c_int = 8;
//
// The minimum compatible version - update to match VCHIQ_VERSION with any
// incompatible change
//
pub const VCHIQ_VERSION_MIN: c_int = 3;
// The version that introduced the VCHIQ_IOC_LIB_VERSION ioctl
pub const VCHIQ_VERSION_LIB_VERSION: c_int = 7;
// The version that introduced the VCHIQ_IOC_CLOSE_DELIVERED ioctl
pub const VCHIQ_VERSION_CLOSE_DELIVERED: c_int = 7;
// The version that made it safe to use SYNCHRONOUS mode
pub const VCHIQ_VERSION_SYNCHRONOUS_MODE: c_int = 8;
pub const VCHIQ_MAX_STATES: c_int = 1;
pub const VCHIQ_MAX_SERVICES: c_int = 4096;
pub const VCHIQ_MAX_SLOTS: c_int = 128;
pub const VCHIQ_MAX_SLOTS_PER_SIDE: c_int = 64;
pub const VCHIQ_NUM_CURRENT_BULKS: c_int = 32;
pub const VCHIQ_NUM_SERVICE_BULKS: c_int = 4;

pub const VCHIQ_ENABLE_DEBUG: c_int = 1;

pub const VCHIQ_ENABLE_STATS: c_int = 1;

