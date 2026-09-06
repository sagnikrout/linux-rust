//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu_common.h
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
// Copyright (c) 2016 Maxime Ripard. All rights reserved.
//

// MMC timing mode switch bit

// Some clocks need this bit to actually apply register changes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_common {
    pub base: *mut void __iomem,
    pub reg: u16,
    pub lock_reg: u16,
    pub prediv: u32,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub features: c_ulong,
    pub lock: *mut spinlock_t,
    pub hw: clk_hw,
}

extern "C" {
    pub fn container_of(_arg: hw, ccu_common: struct, _arg: hw) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_ccu_desc {
    pub ccu_clks: *mut ccu_common,
    pub num_ccu_clks: c_ulong,
    pub hw_clks: *mut clk_hw_onecell_data,
    pub resets: *const ccu_reset_map,
    pub num_resets: c_ulong,
}

extern "C" {
    pub fn ccu_helper_wait_for_lock(common: *mut ccu_common, lock: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_pll_nb {
    pub clk_nb: notifier_block,
    pub common: *mut ccu_common,
    pub enable: u32,
    pub lock: u32,
}

extern "C" {
    pub fn ccu_pll_notifier_register(pll_nb: *mut ccu_pll_nb) -> c_int;
}
