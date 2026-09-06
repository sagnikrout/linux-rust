//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/arm-ux500-pm.h
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
// Copyright (C) ST-Ericsson SA 2010-2013
// Author: Rickard Andersson <rickard.andersson@stericsson.com> for
// ST-Ericsson.
// Author: Daniel Lezcano <daniel.lezcano@linaro.org> for Linaro.
//
extern "C" {
    pub fn prcmu_gic_decouple() -> c_int;
}
extern "C" {
    pub fn prcmu_gic_recouple() -> c_int;
}
extern "C" {
    pub fn prcmu_gic_pending_irq() -> bool;
}
extern "C" {
    pub fn prcmu_pending_irq() -> bool;
}
extern "C" {
    pub fn prcmu_is_cpu_in_wfi(cpu: c_int) -> bool;
}
extern "C" {
    pub fn prcmu_copy_gic_settings() -> c_int;
}
extern "C" {
    pub fn ux500_pm_init(phy_base: u32, size: u32);
}
