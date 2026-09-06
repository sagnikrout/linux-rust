//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pm_clock.h
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
// pm_clock.h - Definitions and headers related to device clocks.
//
// Copyright (C) 2011 Rafael J. Wysocki <rjw@sisk.pl>, Renesas Electronics Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_clk_notifier_block {
    pub nb: notifier_block,
    pub pm_domain: *mut dev_pm_domain,
    pub con_ids: [*mut c_char; ],
}

extern "C" {
    pub fn pm_clk_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_clk_runtime_resume(dev: *mut device) -> c_int;
}

// Macro flag: #define USE_PM_CLK_RUNTIME_OPS

extern "C" {
    pub fn pm_clk_init(dev: *mut device);
}
extern "C" {
    pub fn pm_clk_create(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_clk_destroy(dev: *mut device);
}
extern "C" {
    pub fn pm_clk_add(dev: *mut device, con_id: *const c_char) -> c_int;
}
extern "C" {
    pub fn pm_clk_add_clk(dev: *mut device, clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn of_pm_clk_add_clks(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_clk_remove_clk(dev: *mut device, clk: *mut clk);
}
extern "C" {
    pub fn pm_clk_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_clk_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn devm_pm_clk_create(dev: *mut device) -> c_int;
}

