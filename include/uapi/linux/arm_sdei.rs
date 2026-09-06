//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/arm_sdei.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Copyright (C) 2017 Arm Ltd.
pub const SDEI_1_0_FN_BASE: c_uint = 0xC4000020;
pub const SDEI_1_0_MASK: c_uint = 0xFFFFFFE0;

pub const SDEI_VERSION_MAJOR_SHIFT: c_int = 48;
pub const SDEI_VERSION_MAJOR_MASK: c_uint = 0x7fff;
pub const SDEI_VERSION_MINOR_SHIFT: c_int = 32;
pub const SDEI_VERSION_MINOR_MASK: c_uint = 0xffff;
pub const SDEI_VERSION_VENDOR_SHIFT: c_int = 0;
pub const SDEI_VERSION_VENDOR_MASK: c_uint = 0xffffffff;

// SDEI return values
pub const SDEI_SUCCESS: c_int = 0;

// EVENT_REGISTER flags
pub const SDEI_EVENT_REGISTER_RM_ANY: c_int = 0;
pub const SDEI_EVENT_REGISTER_RM_PE: c_int = 1;
// EVENT_STATUS return value bits
pub const SDEI_EVENT_STATUS_RUNNING: c_int = 2;
pub const SDEI_EVENT_STATUS_ENABLED: c_int = 1;
pub const SDEI_EVENT_STATUS_REGISTERED: c_int = 0;
// EVENT_COMPLETE status values
pub const SDEI_EV_HANDLED: c_int = 0;
pub const SDEI_EV_FAILED: c_int = 1;
// GET_INFO values
pub const SDEI_EVENT_INFO_EV_TYPE: c_int = 0;
pub const SDEI_EVENT_INFO_EV_SIGNALED: c_int = 1;
pub const SDEI_EVENT_INFO_EV_PRIORITY: c_int = 2;
pub const SDEI_EVENT_INFO_EV_ROUTING_MODE: c_int = 3;
pub const SDEI_EVENT_INFO_EV_ROUTING_AFF: c_int = 4;
// and their results
pub const SDEI_EVENT_TYPE_PRIVATE: c_int = 0;
pub const SDEI_EVENT_TYPE_SHARED: c_int = 1;
pub const SDEI_EVENT_PRIORITY_NORMAL: c_int = 0;
pub const SDEI_EVENT_PRIORITY_CRITICAL: c_int = 1;
