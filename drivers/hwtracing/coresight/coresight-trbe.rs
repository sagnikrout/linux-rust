//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-trbe.h
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
// This contains all required hardware related helper functions for
// Trace Buffer Extension (TRBE) driver in the coresight framework.
//
// Copyright (C) 2020 ARM Ltd.
//
// Author: Anshuman Khandual <anshuman.khandual@arm.com>
//

pub const TRBE_EC_OTHERS: c_int = 0;
pub const TRBE_EC_STAGE1_ABORT: c_int = 36;
pub const TRBE_EC_STAGE2_ABORT: c_int = 37;
pub const TRBE_BSC_NOT_STOPPED: c_int = 0;
pub const TRBE_BSC_FILLED: c_int = 1;
pub const TRBE_BSC_TRIGGERED: c_int = 2;
extern "C" {
    pub fn read_sysreg_s(_arg: SYS_TRBPTR_EL1) -> return;
}
