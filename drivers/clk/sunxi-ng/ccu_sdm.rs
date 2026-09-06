//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu_sdm.h
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
// Copyright (c) 2017 Chen-Yu Tsai. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_sdm_setting {
    pub rate: c_ulong,
//
// XXX We don't know what the step and bottom register fields
// mean. Just copy the whole register value from the vendor
// kernel for now.
//
    pub pattern: u32,
//
// M and N factors here should be the values used in
// calculation, not the raw values written to registers
//
    pub m: u32,
    pub n: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_sdm_internal {
    pub table: *mut ccu_sdm_setting,
    pub table_size: u32,
// early SoCs don't have the SDM enable bit in the PLL register
    pub enable: u32,
// second enable bit in tuning register
    pub tuning_enable: u32,
    pub tuning_reg: u16,
}

