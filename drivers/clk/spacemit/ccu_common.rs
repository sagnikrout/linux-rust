//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/spacemit/ccu_common.h
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
pub struct ccu_common {
    pub regmap: *mut regmap,
    pub lock_regmap: *mut regmap,
// For DDN and MIX
    pub reg_ctrl: u32,
    pub reg_fc: u32,
    pub mask_fc: u32,
}

// For PLL
extern "C" {
    pub fn container_of(_arg: hw, ccu_common: struct, _arg: hw) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spacemit_ccu_data {
    pub reset_name: *const c_char,
    pub hws: *mut clk_hw,
    pub num: usize,
}

extern "C" {
    pub fn spacemit_ccu_probe(pdev: *mut platform_device, compat: *const c_char) -> c_int;
}
