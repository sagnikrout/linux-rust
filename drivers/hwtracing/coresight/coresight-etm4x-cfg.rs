//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-etm4x-cfg.h
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
//
// Copyright (c) 2014-2020, The Linux Foundation. All rights reserved.
//

// ETMv4 specific config defines
// resource IDs
pub const ETM4_CFG_RES_CTR: c_uint = 0x001;
pub const ETM4_CFG_RES_CMP: c_uint = 0x002;
pub const ETM4_CFG_RES_CMP_PAIR0: c_uint = 0x003;
pub const ETM4_CFG_RES_CMP_PAIR1: c_uint = 0x004;
pub const ETM4_CFG_RES_SEL: c_uint = 0x005;
pub const ETM4_CFG_RES_SEL_PAIR0: c_uint = 0x006;
pub const ETM4_CFG_RES_SEL_PAIR1: c_uint = 0x007;
pub const ETM4_CFG_RES_SEQ: c_uint = 0x008;
pub const ETM4_CFG_RES_TS: c_uint = 0x009;
pub const ETM4_CFG_RES_MASK: c_uint = 0x00F;
// ETMv4 specific config functions
extern "C" {
    pub fn etm4_cscfg_register(csdev: *mut coresight_device) -> c_int;
}
