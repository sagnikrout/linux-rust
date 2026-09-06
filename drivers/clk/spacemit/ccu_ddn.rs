//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/spacemit/ccu_ddn.h
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
// Copyright (c) 2024 SpacemiT Technology Co. Ltd
// Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_ddn {
    pub common: ccu_common,
    pub num_mask: c_uint,
    pub num_shift: c_uint,
    pub den_mask: c_uint,
    pub den_shift: c_uint,
    pub pre_div: c_uint,
}

extern "C" {
    pub fn container_of(_arg: common, ccu_ddn: struct, _arg: common) -> return;
}
